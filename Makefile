# Automated checks and tests
# Run with: make [target]
#
# Available targets:
#   check      - Run all static checks (fmt + clippy)
#   test       - Run all tests (unit, integration, doc)
#   doc        - Generate documentation
#   build      - Build release binary
#   clean      - Clean build artifacts
#   all        - Run all checks and tests (default)
#
# Static analysis:
#   fmt        - Check formatting only (fast)
#   clippy     - Run clippy only (fast)
#   verify     - Full verification suite (check + test + doc + build)

.PHONY: all check test doc build clean fmt clippy verify

# Default target
all: check test

# Static analysis and formatting checks
check:
	@echo "=== Running cargo fmt --check ==="
	cargo fmt --check || (echo "❌ Formatting check failed"; exit 1)
	@echo "✅ Formatting check passed"
	@echo "=== Running cargo clippy --all-targets -- -D warnings ==="
	cargo clippy --all-targets -- -D warnings || (echo "❌ Clippy check failed"; exit 1)
	@echo "✅ Clippy check passed"
	@echo "=== All static checks passed ==="

# Run all tests (unit, integration, doc) - excludes benchmarks
test:
	@echo "=== Running cargo test ==="
	cargo test || (echo "❌ Tests failed"; exit 1)
	@echo "✅ All tests passed"

# Generate documentation
doc:
	@echo "=== Running cargo doc --no-deps ==="
	cargo doc --no-deps || (echo "❌ Documentation generation failed"; exit 1)
	@echo "✅ Documentation generated"
	@echo "Docs available at: target/doc/mercury_tool_shell/index.html"

# Build release binary
build:
	@echo "=== Building release ==="
	cargo build --release || (echo "❌ Build failed"; exit 1)
	@echo "✅ Release build completed"
	@echo "Binary available at: target/release/mercury-tool-shell"

# Clean build artifacts
clean:
	@echo "=== Cleaning build artifacts ==="
	cargo clean
	rm -rf target/doc
	@echo "✅ Build artifacts cleaned"

# Check only formatting (fast)
fmt:
	cargo fmt --check

# Check only clippy (fast)
clippy:
	cargo clippy --all-targets -- -D warnings

# Full verification suite (recommended for CI)
verify: check test doc build
