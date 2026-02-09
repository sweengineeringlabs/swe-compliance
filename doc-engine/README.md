# doc-engine

Documentation compliance engine enforcing template-engine framework standards.

## Overview

doc-engine is a Rust CLI tool and library that programmatically audits any project against the 50 compliance checks defined by the template-engine documentation framework. It reads rules from a TOML configuration and reports pass/fail/skip for each check.

## Quick Start

```bash
cargo build --release
doc-engine scan .
```

## Documentation

Full project documentation is available at [docs/README.md](docs/README.md).

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
