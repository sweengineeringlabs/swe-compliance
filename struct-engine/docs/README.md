# struct-engine Documentation

**Audience**: Developers, architects, project stakeholders

## Who

Developers, architects, CI systems, and library consumers who need to validate Rust project structure compliance.

## What

struct-engine is a Rust CLI tool and library that audits Rust project structure against configurable compliance rules. It enforces the `{main,tests}` layout, SEA layering, Cargo target paths, naming conventions, and test organization standards.

## Why

Manual structure review is error-prone and inconsistent. struct-engine automates compliance checking with 44 rules across 7 categories, providing fast feedback during development and automated gating in CI pipelines.

## How

- [Requirements (SRS)](srs.md)
- [Backlog](backlog.md)
- [Implementation Plan](implementation_plan.md)
- [Glossary](glossary.md)

## Where

| Phase | Directory | Contents |
|-------|-----------|----------|
| 7-operations | [7-operations/](7-operations/) | Compliance audit reports |
