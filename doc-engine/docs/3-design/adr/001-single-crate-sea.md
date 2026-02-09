# ADR-001: Single-Crate Modular SEA Architecture

**Audience**: Developers, architects

## Status

Accepted

## Context

doc-engine needs an architecture that supports a CLI binary and a reusable library in a single crate. The expected codebase size is 2k-10k lines. We need clean separation between the public API and internal implementation without the overhead of a multi-crate workspace for the engine itself.

## Decision

We adopt the Single-Crate Modular SEA (Stratified Encapsulation Architecture) pattern. The crate exposes a library (`lib.rs`) and a binary (`main.rs`). Internal modules are organized into layers: CLI, core engine, rule evaluation, and builtins.

## Consequences

- Single crate simplifies builds, testing, and dependency management
- Module visibility controls enforce encapsulation boundaries
- Library consumers get a clean public API via `lib.rs`
- The CLI binary is a thin wrapper over the library
