# Changelog

All notable changes to struct-engine will be documented in this file.

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-02-10

### Added

- Initial release with 44 compliance checks across 7 categories
- Declarative rule engine supporting 11 TOML-defined rule types
- Builtin handler system for complex check logic
- CLI with `scan` command, `--json`, `--checks`, `--kind`, `--rules` flags
- Library API via SAF layer (`scan`, `scan_with_config`)
- Auto-detection of project kind (library, binary, both, workspace)
- Text and JSON output formats
- Exit codes: 0 (pass), 1 (fail), 2 (error)
