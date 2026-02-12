# Feature Spec: Reliability

**Version:** 1.0
**Status:** Draft
**Section:** 5.5

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | NFR-500 | Graceful engine errors | Must | Test | When `scan_with_config()` returns `ScanError::Path` or `ScanError::Config`, the API returns a structured error response (422 or 500) with the engine's error message; the server does not panic or crash |
| REQ-002 | NFR-501 | AI graceful degradation | Must | Test | When AI features are not configured (missing API key or disabled), all AI endpoints return 503 with `{"error": {"code": "AI_NOT_CONFIGURED", "message": "..."}}` and the UI disables AI panels; LLM API failures return 502 with the provider's error message |
| REQ-003 | NFR-502 | Data persistence | Must | Test | Project configurations and scan results survive server restarts; the storage backend (SQLite or PostgreSQL, deferred to design) persists all data to disk; no in-memory-only state is required for correctness |

## Acceptance Criteria

- **REQ-001** (NFR-500): When `scan_with_config()` returns `ScanError::Path` or `ScanError::Config`, the API returns a structured error response (422 or 500) with the engine's error message; the server does not panic or crash
- **REQ-002** (NFR-501): When AI features are not configured (missing API key or disabled), all AI endpoints return 503 with `{"error": {"code": "AI_NOT_CONFIGURED", "message": "..."}}` and the UI disables AI panels; LLM API failures return 502 with the provider's error message
- **REQ-003** (NFR-502): Project configurations and scan results survive server restarts; the storage backend (SQLite or PostgreSQL, deferred to design) persists all data to disk; no in-memory-only state is required for correctness

