# Glossary

**Audience**: Developers, architects

**Both** - Project kind where Cargo.toml declares both `[lib]` and `[[bin]]` targets

**Builtin rule** - A compliance check referencing a named Rust handler for complex logic beyond declarative capabilities

**Compliance check** - A single rule that produces Pass, Fail (with violations), or Skip

**Declarative rule** - A check defined entirely in TOML, executed by the generic DeclarativeCheck runner

**Project kind** - Auto-detected type: Library, Binary, Both, or Workspace

**SAF** - Surface API Facade — public re-export layer for library consumers

**SEA** - Stratified Encapsulation Architecture — layered module pattern (API/Core/SAF)

**Umbrella** - A virtual workspace (`[workspace]` only, no `[package]`) grouping two or more sub-crates

**{main,tests}** - Layout convention where source lives in `main/src/` and tests live in `tests/`
