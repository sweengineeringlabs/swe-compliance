# Resource Plan

**Audience**: Developers, project stakeholders

## Team

| Role | Responsibility |
|------|---------------|
| Developer | Implementation, testing, documentation |
| Reviewer | Code review, architecture decisions |

## Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Language | Rust | Core engine, CLI, library |
| Build | Cargo | Compilation, testing, packaging |
| Testing | assert_cmd, tempfile | E2E and integration tests |
| Config | TOML | Rule definitions |
| Output | serde_json | JSON report format |

## Infrastructure

- GitHub for source control and CI
- Cargo for dependency management and publishing

See [architecture.md](../../docs/3-design/architecture.md) for system design context.
