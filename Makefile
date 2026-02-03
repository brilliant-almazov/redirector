.PHONY: test coverage coverage-html fmt lint check

# Run all tests
test:
	cargo test

# Run coverage (excluding non-testable files)
coverage:
	cargo llvm-cov --ignore-filename-regex='tests/|_test\.rs|main\.rs|tools/|admin/pages/|admin/sse\.rs'

# Generate HTML coverage report
coverage-html:
	cargo llvm-cov --ignore-filename-regex='tests/|_test\.rs|main\.rs|tools/|admin/pages/|admin/sse\.rs' --html
	@echo "Report: target/llvm-cov/html/index.html"

# Format code
fmt:
	cargo fmt

# Run clippy
lint:
	cargo clippy -- -D warnings

# Run all checks (format + lint + test)
check: fmt lint test
