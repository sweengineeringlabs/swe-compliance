# struct-engine — Implementation Plan

**Audience**: Developers

## Overview

Implement backlog P0 + P1 items to enforce `{main,tests}` as the default layout for all projects (not workspace-only).

**Backlog items covered:**
- B-1.1 (P0): Change check 2 from `src/` to `main/src/`
- B-1.2 (P0): Change check 3 to look for `main/src/lib.rs` or `main/src/main.rs`
- B-1.3 (P1): Add check: warn if legacy `src/` directory exists
- B-1.4 (P1): Update check 7 from `src/src/` to `main/src/src/`
- B-1.6 (P1): Remove workspace-only scoping from checks 4, 5
- B-3.1 (P0): Update check 19: `[lib] path` default to `main/src/lib.rs`
- B-3.2 (P0): Update check 20: `[[bin]] path` default to `main/src/main.rs`
- B-4.1 (P0): Update check 33: enforce `<crate>_<category>_test.rs` naming
- B-4.6 (P1): Remove workspace-only scoping from test org checks

---

## Changes

### 1. config/rules.toml — Declarative rule updates

| Check | Current | Target |
|-------|---------|--------|
| 2 | `dir_exists path = "src"` | `dir_exists path = "main/src"` |
| 4 | `dir_exists path = "main/src" project_kind = "workspace"` | Remove `project_kind` (redundant with check 2, keep for transition) |
| 5 | `builtin rustboot_crate_root_exists project_kind = "workspace"` | Remove `project_kind` |
| 7 | `dir_not_exists path = "src/src"` | `dir_not_exists path = "main/src/src"` |
| 33 | `builtin test_file_suffixes project_kind = "workspace"` | Remove `project_kind` |
| NEW 45 | — | `dir_exists path = "src"` severity = warning, message = "Legacy src/ detected — should be main/src/" |

### 2. main/src/core/builtins/cargo_toml.rs — Handler updates

**`crate_root_exists` (Check 3):**
- Change `src/lib.rs` → `main/src/lib.rs`
- Change `src/main.rs` → `main/src/main.rs`
- Update violation message path

**`lib_path_correct` (Check 19):**
- Change default fallback from `src/lib.rs` → `main/src/lib.rs`
- Update violation message

### 3. main/src/core/builtins/test_org.rs — Test suffix updates

**`test_file_suffixes` (Check 33):**

Before: `["_test.rs", "_int_test.rs", "_sec_test.rs", "_feat_test.rs", "_e2e_test.rs"]`

After: `["_int_test.rs", "_stress_test.rs", "_perf_test.rs", "_load_test.rs", "_e2e_test.rs", "_security_test.rs"]`

Aligns with rustboot testing strategy categories: int, stress, perf, load, e2e, security.

**`no_test_in_src` (Check 38):**
- Update scan path from `src/` to `main/src/`

### 4. Test fixture updates

**`create_minimal_project()`** in both test files:
- `src/lib.rs` → `main/src/lib.rs`
- `src/utils.rs` → `main/src/utils.rs`
- `Cargo.toml [lib] path` → `main/src/lib.rs`

**`create_rustboot_project()`** in both test files:
- `tests/src/api_int_test.rs` → `tests/rustboot_example_int_test.rs` (flat, `<crate>_<category>_test.rs`)
- Update `Cargo.toml [[test]]` path accordingly

### 5. main/src/core/rules.rs — Rule count

Update `default_rule_count()` from 44 to 45 (adding legacy src/ warning check).

---

## Files to Modify

| File | Changes |
|------|---------|
| `config/rules.toml` | Checks 2, 4, 5, 7, 33 edits + new check 45 |
| `main/src/core/builtins/cargo_toml.rs` | `crate_root_exists` paths, `lib_path_correct` default |
| `main/src/core/builtins/test_org.rs` | `test_file_suffixes` valid suffixes, `no_test_in_src` path |
| `tests/struct_engine_int_test.rs` | `create_minimal_project()` → main/src/, `create_rustboot_project()` → flat tests/ |
| `tests/struct_engine_e2e_test.rs` | Same fixture updates |
| `main/src/core/rules.rs` | `default_rule_count()` 44 → 45 |

## Implementation Order

1. Update `config/rules.toml` (declarative changes)
2. Update `cargo_toml.rs` handlers (crate_root_exists, lib_path_correct)
3. Update `test_org.rs` handlers (test_file_suffixes, no_test_in_src)
4. Update test fixtures in both test files
5. Update individual test assertions that reference old paths
6. Update `default_rule_count()`
7. `cargo test -p struct-engine` — verify all tests pass

## Verification

```bash
# Build
cargo build -p struct-engine

# Run all tests
cargo test -p struct-engine

# Smoke test: scan struct-engine itself (it uses main/src/ layout)
cargo run -p struct-engine -- scan struct-engine/

# Verify check count
cargo run -p struct-engine -- scan struct-engine/ --json 2>/dev/null | python3 -c "import sys,json; r=json.load(sys.stdin); print(f'{len(r[\"results\"])} checks')"
```
