# Feature Spec: Extensibility

**Version:** 1.0
**Status:** Draft
**Section:** 5.4

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | NFR-400 | Declarative rule extensibility | Must | Demonstration | Adding a `[[rules]]` entry with `type = "file_exists"` to `rules.toml` and running `--rules` enforces the new rule without recompilation |
| REQ-002 | NFR-401 | Builtin handler extensibility | Should | Inspection | A new handler can be added in 3 steps: implement function, register in mod.rs, add TOML entry |

## Acceptance Criteria

- **REQ-001** (NFR-400): Adding a `[[rules]]` entry with `type = "file_exists"` to `rules.toml` and running `--rules` enforces the new rule without recompilation
- **REQ-002** (NFR-401): A new handler can be added in 3 steps: implement function, register in mod.rs, add TOML entry

