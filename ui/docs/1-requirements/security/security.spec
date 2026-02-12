# Feature Spec: Security

**Version:** 1.0
**Status:** Draft
**Section:** 5.2

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | NFR-200 | Authentication required | Must | Test | Every API endpoint (except `/health` and `/api/v1/auth/login`) returns 401 when no Authorization header is provided; JWT tokens expire after a configurable period (default: 24 hours) |
| REQ-002 | NFR-201 | Path traversal prevention | Must | Test | Project root_path values containing `..`, symbolic links outside allowed directories, or paths above a configurable base directory are rejected with 422; the server resolves and validates all paths before passing to engine libraries |
| REQ-003 | NFR-202 | API key protection | Must | Inspection | LLM API keys are read from server environment variables and never exposed in API responses, logs, or error messages; the `/api/v1/ai/status` endpoint reports only whether a key is configured (true/false), never the key value |
| REQ-004 | NFR-203 | CORS configuration | Must | Inspection | CORS origins are configurable via environment variable; the default allows only `localhost` origins; production deployments must explicitly configure allowed origins |

## Acceptance Criteria

- **REQ-001** (NFR-200): Every API endpoint (except `/health` and `/api/v1/auth/login`) returns 401 when no Authorization header is provided; JWT tokens expire after a configurable period (default: 24 hours)
- **REQ-002** (NFR-201): Project root_path values containing `..`, symbolic links outside allowed directories, or paths above a configurable base directory are rejected with 422; the server resolves and validates all paths before passing to engine libraries
- **REQ-003** (NFR-202): LLM API keys are read from server environment variables and never exposed in API responses, logs, or error messages; the `/api/v1/ai/status` endpoint reports only whether a key is configured (true/false), never the key value
- **REQ-004** (NFR-203): CORS origins are configurable via environment variable; the default allows only `localhost` origins; production deployments must explicitly configure allowed origins

