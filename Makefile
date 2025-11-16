.PHONY: all build release debug clean install test help

# Default target
all: release

# Build in release mode
release:
	@echo "Building Bendis in release mode..."
	cd bendis && cargo build --release
	@echo "✓ Build complete: bendis/target/release/bendis"

# Build in debug mode
debug:
	@echo "Building Bendis in debug mode..."
	cd bendis && cargo build
	@echo "✓ Build complete: bendis/target/debug/bendis"

# Alias for release
build: release

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cd bendis && cargo clean
	@echo "✓ Clean complete"

# Install to /usr/local/bin (requires sudo)
install: release
	@echo "Installing Bendis to /usr/local/bin..."
	sudo cp bendis/target/release/bendis /usr/local/bin/
	@echo "✓ Bendis installed successfully"
	@echo "Run 'bendis --version' to verify"

# Run tests
test:
	@echo "Running tests..."
	cd bendis && cargo test
	@echo "✓ Tests complete"

# Check code without building
check:
	@echo "Checking code..."
	cd bendis && cargo check
	@echo "✓ Check complete"

# Format code
fmt:
	@echo "Formatting code..."
	cd bendis && cargo fmt
	@echo "✓ Format complete"

# Lint code
lint:
	@echo "Linting code..."
	cd bendis && cargo clippy
	@echo "✓ Lint complete"

# Show help
help:
	@echo "Bendis Build System"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  all      - Build in release mode (default)"
	@echo "  build    - Build in release mode"
	@echo "  release  - Build optimized release binary"
	@echo "  debug    - Build debug binary"
	@echo "  clean    - Remove build artifacts"
	@echo "  install  - Install to /usr/local/bin (requires sudo)"
	@echo "  test     - Run tests"
	@echo "  check    - Check code without building"
	@echo "  fmt      - Format code"
	@echo "  lint     - Lint code with clippy"
	@echo "  help     - Show this help message"
	@echo ""
	@echo "Prerequisites:"
	@echo "  - Rust toolchain (rustc, cargo)"
	@echo "  - Bender (for runtime)"
	@echo ""
	@echo "See BUILD.md for detailed build instructions."
