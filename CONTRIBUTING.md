# Contributing to Redirector

Thank you for your interest in contributing to Redirector!

## Development Setup

1. Install Rust (1.75+)
2. Install Docker and Docker Compose
3. Clone the repository
4. Copy `config.yaml.example` to `config.yaml` and adjust settings
5. Start dependencies: `docker-compose up -d redis`
6. Run tests: `cargo test`
7. Run the service: `cargo run`

## Code Style

- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Write tests for new functionality
- Keep commits focused and atomic

## Pull Request Process

1. Create a feature branch from `master`
2. Make your changes
3. Ensure all tests pass
4. Update documentation if needed
5. Submit a pull request

## Reporting Issues

- Use GitHub Issues
- Include steps to reproduce
- Include relevant logs and configuration (redact secrets)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
