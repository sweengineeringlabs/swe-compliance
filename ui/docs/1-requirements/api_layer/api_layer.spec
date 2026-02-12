# Feature Spec: API Layer

**Version:** 1.0
**Status:** Draft
**Section:** 4.12

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-1200 | REST API versioning | Must | Inspection | All API endpoints are prefixed with `/api/v1/`; the version is part of the URL path; no endpoints exist outside the versioned prefix (except `/health`) |
| REQ-002 | FR-1201 | Authentication | Must | Test | `POST /api/v1/auth/login` with `{"username": "...", "password": "..."}` returns a JWT token; all other endpoints (except `/health` and `/api/v1/auth/login`) require a valid `Authorization: Bearer <token>` header; expired or invalid tokens return 401 |
| REQ-003 | FR-1202 | Health check | Must | Test | `GET /health` returns 200 with `{"status": "ok", "version": "x.y.z", "engines": {"doc-engine": "x.y.z", "struct-engine": "x.y.z"}}` |
| REQ-004 | FR-1203 | WebSocket connection management | Must | Test | WebSocket connections at `/api/v1/ws` require a valid JWT token as a query parameter (`?token=...`); connections without a valid token are rejected with 401; the server sends a ping every 30 seconds and closes idle connections after 120 seconds |
| REQ-005 | FR-1204 | Error response format | Must | Inspection | All error responses follow the format `{"error": {"code": "ERROR_CODE", "message": "Human-readable description", "details": {...}}}` with appropriate HTTP status codes: 400 (bad request), 401 (unauthorized), 404 (not found), 422 (validation error), 500 (internal error), 503 (service unavailable) |
| REQ-006 | FR-1205 | Request rate limiting | Should | Test | API endpoints enforce rate limiting of 100 requests per minute per authenticated user; scan execution endpoints are limited to 10 concurrent scans per user; exceeding limits returns 429 with a `Retry-After` header |

## Acceptance Criteria

- **REQ-001** (FR-1200): All API endpoints are prefixed with `/api/v1/`; the version is part of the URL path; no endpoints exist outside the versioned prefix (except `/health`)
- **REQ-002** (FR-1201): `POST /api/v1/auth/login` with `{"username": "...", "password": "..."}` returns a JWT token; all other endpoints (except `/health` and `/api/v1/auth/login`) require a valid `Authorization: Bearer <token>` header; expired or invalid tokens return 401
- **REQ-003** (FR-1202): `GET /health` returns 200 with `{"status": "ok", "version": "x.y.z", "engines": {"doc-engine": "x.y.z", "struct-engine": "x.y.z"}}`
- **REQ-004** (FR-1203): WebSocket connections at `/api/v1/ws` require a valid JWT token as a query parameter (`?token=...`); connections without a valid token are rejected with 401; the server sends a ping every 30 seconds and closes idle connections after 120 seconds
- **REQ-005** (FR-1204): All error responses follow the format `{"error": {"code": "ERROR_CODE", "message": "Human-readable description", "details": {...}}}` with appropriate HTTP status codes: 400 (bad request), 401 (unauthorized), 404 (not found), 422 (validation error), 500 (internal error), 503 (service unavailable)
- **REQ-006** (FR-1205): API endpoints enforce rate limiting of 100 requests per minute per authenticated user; scan execution endpoints are limited to 10 concurrent scans per user; exceeding limits returns 429 with a `Retry-After` header

