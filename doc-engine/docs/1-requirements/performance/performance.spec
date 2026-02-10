# Feature Spec: Performance

**Version:** 1.0
**Status:** Draft
**Section:** 5.2

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | NFR-200 | Synchronous execution | Must | Inspection | No `tokio`, `async-std`, or other async runtime in dependencies |
| REQ-002 | NFR-201 | Single pass | Should | Analysis | Profiling shows exactly one `walkdir` traversal per scan invocation |

## Acceptance Criteria

- **REQ-001** (NFR-200): No `tokio`, `async-std`, or other async runtime in dependencies
- **REQ-002** (NFR-201): Profiling shows exactly one `walkdir` traversal per scan invocation

