# struct-engine — Backlog

## Overview

Enhance struct-engine rules to enforce the updated conventions from:
- `rustboot/docs/3-design/project_structure.md`
- `rustboot/doc/5_testing/testing_strategy.md`

Current state: 44 rules across 7 categories (mixed standard `src/` and rustboot `main/src/` checks).
Target state: unified ruleset enforcing `{main,tests}` layout, SEA layers, and testing strategy.

---

## Phase 1: Structure Rules — {main,tests} Layout

Rewrite structure checks (currently IDs 1-8) to enforce `{main,tests}` as the default layout.

| Task | SRS Ref | Priority | Description |
|------|---------|----------|-------------|
| B-1.1 | FR-1.2 | P0 | Change check 2 from `src/` to `main/src/` |
| B-1.2 | FR-1.3 | P0 | Change check 3 to look for `main/src/lib.rs` or `main/src/main.rs` |
| B-1.3 | FR-1.5 | P1 | Add check: warn if legacy `src/` directory exists (should be `main/src/`) |
| B-1.4 | FR-1.6 | P1 | Update check 7 from `src/src/` to `main/src/src/` |
| B-1.5 | FR-1.4 | P1 | Update check 6 to validate `tests/` (already correct) |
| B-1.6 | — | P1 | Remove workspace-only scoping from checks 4,5 (make `main/src/` the default, not workspace-only) |
| B-1.7 | — | P2 | Remove legacy `src/`-based checks (old IDs 2,3) or consolidate into `main/src/` |

## Phase 2: SEA Layer Validation (New)

Add new checks for SEA layering inside `main/src/`.

| Task | SRS Ref | Priority | Description |
|------|---------|----------|-------------|
| B-2.1 | FR-2.1 | P1 | Add check: `main/src/api/` exists (libraries) |
| B-2.2 | FR-2.2 | P1 | Add check: `main/src/core/` exists (libraries) |
| B-2.3 | FR-2.3 | P1 | Add check: `main/src/saf/` exists (libraries) |
| B-2.4 | FR-2.4 | P2 | Add check: `main/src/api/mod.rs` exists |
| B-2.5 | FR-2.5 | P2 | Add check: `main/src/core/mod.rs` exists |
| B-2.6 | FR-2.6 | P2 | Add check: `main/src/saf/mod.rs` exists |
| B-2.7 | FR-2.7 | P2 | Add check: `lib.rs` declares `pub mod api; mod core; mod saf;` |
| B-2.8 | FR-2.8 | P2 | Add check: `lib.rs` has `pub use saf::*;` |
| B-2.9 | FR-2.9 | P2 | Add check: no `spi/` unless justified |

## Phase 3: Cargo Target Path Updates

Update target path checks to expect `main/src/` paths.

| Task | SRS Ref | Priority | Description |
|------|---------|----------|-------------|
| B-3.1 | FR-4.1 | P0 | Update check 19: `[lib] path` should point to `main/src/lib.rs` |
| B-3.2 | FR-4.2 | P0 | Update check 20: `[[bin]] path` should point to `main/src/main.rs` |
| B-3.3 | FR-4.5 | P1 | Add check: `[[test]] name` follows `<crate>_<category>_test` pattern |
| B-3.4 | FR-4.3 | P1 | Update check 21: validate `[[test]]` targets for `tests/` files |
| B-3.5 | FR-4.4 | P1 | Update check 26: validate `[[test]] path` resolves |

## Phase 4: Test Organization Updates

Align test organization checks with testing strategy.

| Task | SRS Ref | Priority | Description |
|------|---------|----------|-------------|
| B-4.1 | FR-6.1 | P0 | Update check 33: enforce `<crate>_<category>_test.rs` naming |
| B-4.2 | FR-6.2 | P1 | Add check: validate category is one of `int`, `stress`, `perf`, `load`, `e2e`, `security` |
| B-4.3 | FR-6.3 | P1 | Add check: `tests/` directory is flat (no subdirectories) |
| B-4.4 | FR-6.5 | P1 | Update check 38: path from `src/` to `main/src/` |
| B-4.5 | FR-6.4 | P2 | Update check 37: unit tests colocated in `main/src/` |
| B-4.6 | — | P1 | Remove workspace-only scoping from test org checks (make default) |
| B-4.7 | — | P2 | Remove check 36 (`int_tests_location` checking `tests/src/` — obsolete, tests are flat) |

## Phase 5: Naming Convention Updates

Update naming checks to reference `main/src/` paths.

| Task | SRS Ref | Priority | Description |
|------|---------|----------|-------------|
| B-5.1 | FR-5.3 | P1 | Update check 30: glob from `src/**/*.rs` to `main/src/**/*.rs` |
| B-5.2 | FR-5.6 | P2 | Add check: folder names use kebab-case |
| B-5.3 | FR-5.1 | P2 | Update check 27: enforce kebab-case (not just snake_case-or-kebab) |

## Phase 6: Umbrella Validation (New)

Add checks for umbrella (virtual workspace) crates.

| Task | SRS Ref | Priority | Description |
|------|---------|----------|-------------|
| B-6.1 | FR-9.1 | P1 | Add check: umbrella has `[workspace]` and no `[package]` |
| B-6.2 | FR-9.2 | P1 | Add check: umbrella has no `src/` or `main/` |
| B-6.3 | FR-9.3 | P1 | Add check: umbrella has no `lib.rs` |
| B-6.4 | FR-9.4 | P2 | Add check: all workspace members exist |

## Phase 7: Optional Directory Validation (New)

Add checks for crate-level vs project-level directory placement.

| Task | SRS Ref | Priority | Description |
|------|---------|----------|-------------|
| B-7.1 | FR-10.1 | P2 | Add check: only recognized directories at crate level |
| B-7.2 | FR-10.3 | P2 | Add check: project-level dirs (`docs/`, `infra/`, `scripts/`) not inside crates |

## Phase 8: Rule Renumbering and Cleanup

After all phases, renumber rules for clean sequencing.

| Task | SRS Ref | Priority | Description |
|------|---------|----------|-------------|
| B-8.1 | — | P2 | Renumber all rules sequentially by category |
| B-8.2 | — | P2 | Update `rules-rustboot.toml` to match (or deprecate if merged) |
| B-8.3 | — | P2 | Update integration and E2E tests for new rule IDs |
| B-8.4 | — | P1 | Update `default_rule_count()` to reflect new total |

---

## Priority Legend

| Priority | Meaning |
|----------|---------|
| P0 | Must have — core layout enforcement |
| P1 | Should have — important convention checks |
| P2 | Nice to have — polish and completeness |

## Estimated New Rule Count

- Current: 44 rules
- Removed/merged: ~5 (legacy `src/` checks, obsolete `tests/src/` check)
- Added: ~20 (SEA layers, umbrella, test naming, optional dirs)
- Estimated total: ~59 rules
