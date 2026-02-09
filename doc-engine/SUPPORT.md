# Support

## Getting Help

- **Documentation**: See [docs/README.md](docs/README.md) for full project documentation
- **Issues**: Open an issue on GitHub for bug reports or feature requests
- **Discussions**: Use GitHub Discussions for questions and general help

## Frequently Asked Questions

### How do I run a compliance scan?

```bash
cargo run -- scan /path/to/project
```

### How do I use custom rules?

```bash
cargo run -- scan --rules my_rules.toml /path/to/project
```

### What output formats are supported?

doc-engine supports plain text (default) and JSON (`--format json`) output.
