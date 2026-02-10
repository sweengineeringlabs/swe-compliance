# Feature Spec: Reliability

**Version:** 1.0
**Status:** Draft
**Section:** 5.5

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | NFR-500 | Graceful error handling | Must | Test | Scanning a non-existent path produces exit code 2 and a message, not a panic |
| REQ-002 | NFR-501 | Invalid rules detection | Must | Test | Malformed TOML produces exit code 2 with a parse error message identifying the line |

## Acceptance Criteria

- **REQ-001** (NFR-500): Scanning a non-existent path produces exit code 2 and a message, not a panic
- **REQ-002** (NFR-501): Malformed TOML produces exit code 2 with a parse error message identifying the line

