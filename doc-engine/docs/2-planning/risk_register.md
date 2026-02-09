# Risk Register

**Audience**: Developers, project stakeholders

## Risks

| ID | Risk | Likelihood | Impact | Mitigation |
|----|------|-----------|--------|------------|
| R-01 | Check ID collisions when multiple contributors add checks | Low | High | Reserve ID ranges per category; document in CONTRIBUTING.md |
| R-02 | rules.toml grows beyond u8 ID limit (255) | Low | Medium | Plan migration to u16 if check count exceeds 200 |
| R-03 | File system traversal performance degrades on large repos | Medium | Medium | Single-traversal design (NFR-201); benchmark on 10k+ file repos |
| R-04 | Breaking changes to rule TOML schema | Low | High | ADR process for schema changes; semver compliance |
| R-05 | Self-compliance drift as new checks are added | Medium | Low | CI runs `cargo run -- scan .` on every commit |

See [architecture.md](../../docs/3-design/architecture.md) for system design context.
