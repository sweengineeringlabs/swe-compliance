use std::fmt;
use std::io::{self, Read, Write};
use std::net::TcpStream;

use crate::crc32::crc32;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors returned by Kafka wire-protocol operations.
pub enum KafkaError {
    Io(io::Error),
    Protocol(String),
}

impl From<io::Error> for KafkaError {
    fn from(e: io::Error) -> Self {
        KafkaError::Io(e)
    }
}

impl fmt::Display for KafkaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KafkaError::Io(e) => write!(f, "Kafka IO error: {}", e),
            KafkaError::Protocol(s) => write!(f, "Kafka protocol error: {}", s),
        }
    }
}

impl fmt::Debug for KafkaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

// ---------------------------------------------------------------------------
// RequestBuffer — accumulates big-endian bytes
// ---------------------------------------------------------------------------

struct RequestBuffer {
    buf: Vec<u8>,
}

impl RequestBuffer {
    fn new() -> Self {
        Self { buf: Vec::new() }
    }

    fn put_i8(&mut self, v: i8) {
        self.buf.push(v as u8);
    }

    fn put_i16(&mut self, v: i16) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }

    fn put_i32(&mut self, v: i32) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }

    fn put_i64(&mut self, v: i64) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }

    fn put_string(&mut self, s: &str) {
        self.put_i16(s.len() as i16);
        self.buf.extend_from_slice(s.as_bytes());
    }

    fn put_bytes(&mut self, data: &[u8]) {
        self.put_i32(data.len() as i32);
        self.buf.extend_from_slice(data);
    }

    fn put_null_bytes(&mut self) {
        self.put_i32(-1);
    }

    fn put_raw(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    fn into_vec(self) -> Vec<u8> {
        self.buf
    }
}

// ---------------------------------------------------------------------------
// Message encoding (Kafka MessageSet v0)
// ---------------------------------------------------------------------------

fn encode_message_v0(key: Option<&[u8]>, value: &[u8]) -> Vec<u8> {
    // Message v0: CRC(4) + MagicByte(1) + Attributes(1) + Key(bytes) + Value(bytes)
    let mut payload = RequestBuffer::new();
    payload.put_i8(0); // magic byte
    payload.put_i8(0); // attributes (no compression)
    match key {
        Some(k) => payload.put_bytes(k),
        None => payload.put_null_bytes(),
    }
    payload.put_bytes(value);

    let payload_bytes = payload.into_vec();
    let checksum = crc32(&payload_bytes);

    let mut msg = RequestBuffer::new();
    msg.put_i32(checksum as i32); // CRC
    msg.put_raw(&payload_bytes);
    msg.into_vec()
}

fn encode_message_set(key: Option<&[u8]>, value: &[u8]) -> Vec<u8> {
    let message = encode_message_v0(key, value);
    let mut ms = RequestBuffer::new();
    ms.put_i64(0); // offset
    ms.put_i32(message.len() as i32); // message size
    ms.put_raw(&message);
    ms.into_vec()
}

// ---------------------------------------------------------------------------
// Produce Request v0
// ---------------------------------------------------------------------------

/// Encode a full Kafka Produce Request v0 frame (size-prefixed).
///
/// Wire layout:
/// ```text
/// Size(i32) | ApiKey(i16=0) | ApiVersion(i16=0) | CorrelationId(i32)
/// | ClientId(string) | Acks(i16=1) | Timeout(i32)
/// | TopicCount(i32=1) | TopicName(string) | PartitionCount(i32=1)
/// | Partition(i32) | MessageSetSize(i32) | MessageSet
/// ```
pub fn encode_produce_request(
    correlation_id: i32,
    client_id: &str,
    topic: &str,
    partition: i32,
    timeout_ms: i32,
    key: Option<&[u8]>,
    value: &[u8],
) -> Vec<u8> {
    let message_set = encode_message_set(key, value);

    let mut body = RequestBuffer::new();
    // Request header
    body.put_i16(0); // api_key = Produce
    body.put_i16(0); // api_version = 0
    body.put_i32(correlation_id);
    body.put_string(client_id);
    // Produce body
    body.put_i16(1); // acks = 1
    body.put_i32(timeout_ms);
    body.put_i32(1); // topic count
    body.put_string(topic);
    body.put_i32(1); // partition count
    body.put_i32(partition);
    body.put_i32(message_set.len() as i32); // message set size
    body.put_raw(&message_set);

    let body_bytes = body.into_vec();

    // Frame: size prefix + body
    let mut frame = RequestBuffer::new();
    frame.put_i32(body_bytes.len() as i32);
    frame.put_raw(&body_bytes);
    frame.into_vec()
}

// ---------------------------------------------------------------------------
// Response decoding helpers
// ---------------------------------------------------------------------------

fn read_i16(stream: &mut TcpStream) -> Result<i16, KafkaError> {
    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf)?;
    Ok(i16::from_be_bytes(buf))
}

fn read_i32(stream: &mut TcpStream) -> Result<i32, KafkaError> {
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf)?;
    Ok(i32::from_be_bytes(buf))
}

fn read_i64(stream: &mut TcpStream) -> Result<i64, KafkaError> {
    let mut buf = [0u8; 8];
    stream.read_exact(&mut buf)?;
    Ok(i64::from_be_bytes(buf))
}

fn read_string(stream: &mut TcpStream) -> Result<String, KafkaError> {
    let len = read_i16(stream)? as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    String::from_utf8(buf)
        .map_err(|e| KafkaError::Protocol(format!("invalid UTF-8 in string: {}", e)))
}

// ---------------------------------------------------------------------------
// Produce Response v0
// ---------------------------------------------------------------------------

/// Decoded Produce Response v0 from a Kafka broker.
pub struct ProduceResponse {
    pub correlation_id: i32,
    pub topic: String,
    pub partition: i32,
    pub error_code: i16,
    pub offset: i64,
}

/// Decode a Produce Response v0 from a TCP stream.
pub fn decode_produce_response(stream: &mut TcpStream) -> Result<ProduceResponse, KafkaError> {
    let _size = read_i32(stream)?;
    let correlation_id = read_i32(stream)?;
    let _topic_count = read_i32(stream)?;
    let topic = read_string(stream)?;
    let _partition_count = read_i32(stream)?;
    let partition = read_i32(stream)?;
    let error_code = read_i16(stream)?;
    let offset = read_i64(stream)?;

    if error_code != 0 {
        return Err(KafkaError::Protocol(format!(
            "broker returned error code {}: {}",
            error_code,
            kafka_error_message(error_code),
        )));
    }

    Ok(ProduceResponse {
        correlation_id,
        topic,
        partition,
        error_code,
        offset,
    })
}

fn kafka_error_message(code: i16) -> &'static str {
    match code {
        0 => "No error",
        1 => "Offset out of range",
        2 => "Corrupt message",
        3 => "Unknown topic or partition",
        4 => "Invalid fetch size",
        5 => "Leader not available",
        6 => "Not leader for partition",
        7 => "Request timed out",
        8 => "Broker not available",
        9 => "Replica not available",
        10 => "Message too large",
        _ => "Unknown error",
    }
}

// ---------------------------------------------------------------------------
// KafkaProducer
// ---------------------------------------------------------------------------

/// A simple Kafka producer that speaks the wire protocol directly.
///
/// Each call to [`produce`](KafkaProducer::produce) opens a new TCP connection,
/// sends a single Produce Request v0, and reads the response.
pub struct KafkaProducer {
    pub broker: String,
    pub topic: String,
    pub client_id: String,
    pub partition: i32,
    pub timeout_ms: i32,
}

impl KafkaProducer {
    /// Create a new producer with sensible defaults.
    pub fn new(broker: impl Into<String>, topic: impl Into<String>) -> Self {
        Self {
            broker: broker.into(),
            topic: topic.into(),
            client_id: "compliance-scan".to_string(),
            partition: 0,
            timeout_ms: 30_000,
        }
    }

    /// Create a producer from a [`KafkaConfig`](crate::KafkaConfig).
    pub fn from_config(config: &crate::config::KafkaConfig) -> Self {
        Self {
            broker: config.broker.clone(),
            topic: config.topic.clone(),
            client_id: config.client_id.clone(),
            partition: config.partition,
            timeout_ms: config.timeout_ms,
        }
    }

    /// Produce a single message and return the broker-assigned offset.
    pub fn produce(&self, value: &[u8]) -> Result<i64, KafkaError> {
        let correlation_id = 1;
        let frame = encode_produce_request(
            correlation_id,
            &self.client_id,
            &self.topic,
            self.partition,
            self.timeout_ms,
            None,
            value,
        );

        let mut stream = TcpStream::connect(&self.broker)?;
        stream.write_all(&frame)?;
        stream.flush()?;

        let response = decode_produce_response(&mut stream)?;
        Ok(response.offset)
    }
}

// ---------------------------------------------------------------------------
// Tests (no broker needed)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_produce_request_frame_size() {
        let frame = encode_produce_request(42, "test-client", "my-topic", 0, 5000, None, b"hello");
        // First 4 bytes = big-endian i32 = remaining length
        let size = i32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
        assert_eq!(size, frame.len() - 4);
    }

    #[test]
    fn test_produce_request_header() {
        let frame = encode_produce_request(99, "cid", "t", 0, 1000, None, b"v");
        // After 4-byte size prefix:
        // api_key(i16) at offset 4
        let api_key = i16::from_be_bytes([frame[4], frame[5]]);
        assert_eq!(api_key, 0);
        // api_version(i16) at offset 6
        let api_version = i16::from_be_bytes([frame[6], frame[7]]);
        assert_eq!(api_version, 0);
        // correlation_id(i32) at offset 8
        let corr = i32::from_be_bytes([frame[8], frame[9], frame[10], frame[11]]);
        assert_eq!(corr, 99);
    }

    #[test]
    fn test_message_v0_crc() {
        let msg = encode_message_v0(None, b"test-value");
        // First 4 bytes = CRC, remaining bytes = payload
        let stored_crc = u32::from_be_bytes([msg[0], msg[1], msg[2], msg[3]]);
        let computed_crc = crc32(&msg[4..]);
        assert_eq!(stored_crc, computed_crc);
    }

    #[test]
    fn test_null_key_encoding() {
        let msg = encode_message_v0(None, b"val");
        // After CRC(4) + magic(1) + attrs(1) = offset 6: key length(i32)
        let key_len = i32::from_be_bytes([msg[6], msg[7], msg[8], msg[9]]);
        assert_eq!(key_len, -1);
    }

    #[test]
    fn test_value_encoding() {
        let value = b"hello-kafka";
        let msg = encode_message_v0(None, value);
        // After CRC(4) + magic(1) + attrs(1) + null_key(4) = offset 10: value length(i32)
        let val_len = i32::from_be_bytes([msg[10], msg[11], msg[12], msg[13]]) as usize;
        assert_eq!(val_len, value.len());
        assert_eq!(&msg[14..14 + val_len], value);
    }

    #[test]
    fn test_produce_no_broker() {
        let producer = KafkaProducer::new("127.0.0.1:1", "test-topic");
        let result = producer.produce(b"data");
        assert!(result.is_err());
        match result.unwrap_err() {
            KafkaError::Io(_) => {} // expected
            other => panic!("expected Io error, got: {}", other),
        }
    }
}
