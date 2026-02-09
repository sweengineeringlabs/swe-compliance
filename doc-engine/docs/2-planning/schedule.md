# Schedule

**Audience**: Developers, project stakeholders

## Milestones

| Milestone | Target | Status |
|-----------|--------|--------|
| M1: Core engine with declarative checks (1-32) | Phase 1-6 | Complete |
| M2: Content and navigation checks (33-47) | Phase 7-10 | Complete |
| M3: ADR and traceability checks (48-53) | Phase 11-13 | Complete |
| M4: Backlog and module checks (69-82) | Phase 14-16 | Complete |
| M5: Planning phase checks (83-88) | Phase 17 | Complete |
| M6: CI/CD integration | Phase 18-19 | Planned |

## Dependencies

- M2 depends on M1 (engine must exist before content checks)
- M3 depends on M1 (traceability uses file scanning)
- M4 depends on M1 (module discovery extends scanner)
- M5 depends on M1 (declarative checks only)
- M6 depends on all prior milestones

See [architecture.md](../../docs/3-design/architecture.md) for system design context.
