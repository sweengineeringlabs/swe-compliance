# doc-engine Documentation

**Audience**: Developers, architects, project stakeholders

## What

doc-engine is a Rust CLI tool and library that audits project documentation against the 53 compliance checks (50 structural + 3 traceability) defined by the template-engine documentation framework.

## Who

This documentation is for developers building or extending doc-engine, architects reviewing its design, and project stakeholders evaluating compliance coverage.

## Why

Manual documentation audits are slow, inconsistent, and error-prone. doc-engine automates compliance verification so teams can enforce documentation standards as part of their CI pipeline.

## How

doc-engine reads rules from a TOML configuration file, walks the project directory, and evaluates each rule against the file system and file contents. Results are reported as pass, fail, or skip for each check.

## Documentation Map

| Phase | Directory | Description |
|-------|-----------|-------------|
| Requirements | [1-requirements](1-requirements/) | Software requirements specification |
| Planning | [2-planning](2-planning/) | Implementation plan and milestones |
| Design | [3-design](3-design/) | Architecture, ADRs, and compliance checklist |
