use crate::util::auth::get_token;

/// Base URL for API requests (proxied through rsc dev server).
const API_BASE: &str = "/api/v1";

/// HTTP methods supported by the API client.
enum Method {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

/// Shared API client error.
struct ApiError {
    pub code: String,
    pub message: String,
}

/// Perform a GET request with JWT header injection.
pub async fn api_get(path: &str) -> Result<String, ApiError> {
    api_request(Method::Get, path, None).await
}

/// Perform a POST request with JSON body.
pub async fn api_post(path: &str, body: &str) -> Result<String, ApiError> {
    api_request(Method::Post, path, Some(body)).await
}

/// Perform a PATCH request with JSON body.
pub async fn api_patch(path: &str, body: &str) -> Result<String, ApiError> {
    api_request(Method::Patch, path, Some(body)).await
}

/// Perform a PUT request with JSON body.
pub async fn api_put(path: &str, body: &str) -> Result<String, ApiError> {
    api_request(Method::Put, path, Some(body)).await
}

/// Perform a DELETE request.
pub async fn api_delete(path: &str) -> Result<String, ApiError> {
    api_request(Method::Delete, path, None).await
}

/// Core fetch wrapper injecting JWT Authorization header.
async fn api_request(method: Method, path: &str, body: Option<&str>) -> Result<String, ApiError> {
    let url = format!("{API_BASE}{path}");
    let token = get_token();

    let method_str = match method {
        Method::Get => "GET",
        Method::Post => "POST",
        Method::Patch => "PATCH",
        Method::Put => "PUT",
        Method::Delete => "DELETE",
    };

    let mut headers = vec![
        ("Content-Type", "application/json"),
        ("Accept", "application/json"),
    ];

    if let Some(ref t) = token {
        headers.push(("Authorization", &format!("Bearer {t}")));
    }

    let response = fetch(method_str, &url, headers, body).await;

    match response.status {
        200..=299 => Ok(response.body),
        401 => Err(ApiError {
            code: "UNAUTHORIZED".into(),
            message: "session expired — please log in again".into(),
        }),
        status => {
            let error = parse_error_body(&response.body).unwrap_or_else(|| ApiError {
                code: format!("HTTP_{status}"),
                message: response.body.clone(),
            });
            Err(error)
        }
    }
}

/// Parse structured error response from the API.
fn parse_error_body(body: &str) -> Option<ApiError> {
    let parsed = json_parse(body).unwrap_or_default();
    let error = parsed.get("error").unwrap_or_default();
    Some(ApiError {
        code: error.get_str("code").unwrap_or_default().into(),
        message: error.get_str("message").unwrap_or_default().into(),
    })
}

/// Create a WebSocket connection with JWT query parameter.
pub fn api_ws(path: &str) -> WebSocket {
    let token = get_token().unwrap_or_default();
    let protocol = if window_location_protocol() == "https:" {
        "wss:"
    } else {
        "ws:"
    };
    let host = window_location_host();
    let url = format!("{protocol}//{host}{API_BASE}{path}?token={token}");
    websocket_connect(&url)
}
