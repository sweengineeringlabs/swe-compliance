# Contributing to doc-engine

Thank you for your interest in contributing to doc-engine.

## Getting Started

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run compliance check (`cargo run -- scan .`)
6. Commit your changes (`git commit -am 'Add my feature'`)
7. Push to the branch (`git push origin feature/my-feature`)
8. Open a Pull Request

## Development Setup

```bash
cargo build
cargo test
```

## Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Run `cargo clippy` before submitting
- All public items should have documentation comments

## Reporting Issues

Please use the GitHub issue tracker to report bugs or request features.
