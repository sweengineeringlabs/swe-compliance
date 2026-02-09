# doc-engine

[![CI](https://github.com/sweengineeringlabs/swe-compliance/actions/workflows/ci.yml/badge.svg)](https://github.com/sweengineeringlabs/swe-compliance/actions/workflows/ci.yml)

Documentation compliance engine enforcing template-engine framework standards.

## Overview

doc-engine is a Rust CLI tool and library that programmatically audits any
project against the compliance checks defined by the template-engine
documentation framework. It reads rules from a TOML configuration and reports
pass/fail/skip for each check.

## Quick Start

```bash
cargo build --release
doc-engine scan .
```

## Usage

```bash
doc-engine scan <PATH>                   # scan project, exit 1 on failures
doc-engine scan <PATH> --json            # JSON output
doc-engine scan <PATH> --checks 1-13     # run specific checks only
doc-engine scan <PATH> --checks 33,40-43 # comma-separated ranges
doc-engine scan <PATH> --type internal   # override project type
doc-engine scan <PATH> --rules custom.toml  # custom rules file
```

### Spec Subcommand

```bash
doc-engine spec validate <PATH>              # validate spec files
doc-engine spec cross-ref <PATH>             # cross-reference analysis
doc-engine spec generate <FILE>              # generate markdown
doc-engine spec generate <FILE> --output DIR # output to directory
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All checks passed |
| 1 | One or more checks failed |
| 2 | Error (bad path, invalid config) |

## Documentation

Full project documentation is available at [docs/README.md](docs/README.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
