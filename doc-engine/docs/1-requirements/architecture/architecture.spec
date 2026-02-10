# Feature Spec: Architecture

**Version:** 1.0
**Status:** Draft
**Section:** 5.1

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | NFR-100 | SEA compliance | Must | Inspection | Code review confirms 5-layer SEA: SPI (no deps) <- API <- Core (private) <- SAF (re-exports) <- CLI |
| REQ-002 | NFR-101 | Dependency direction | Must | Inspection | No `use core::` in spi/ or api/; no `use api::` in spi/ |

## Acceptance Criteria

- **REQ-001** (NFR-100): Code review confirms 5-layer SEA: SPI (no deps) <- API <- Core (private) <- SAF (re-exports) <- CLI
- **REQ-002** (NFR-101): No `use core::` in spi/ or api/; no `use api::` in spi/

