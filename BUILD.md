# Bendis Build Guide

## Prerequisites

### 1. Install Rust

If Rust is not installed, install it using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts and restart your shell or run:

```bash
source $HOME/.cargo/env
```

Verify installation:

```bash
rustc --version
cargo --version
```

### 2. Install Bender

Bendis wraps Bender, so you need Bender installed. Follow the instructions at:
https://github.com/pulp-platform/bender

Or use the quick install:

```bash
curl --proto '=https' --tlsv1.2 https://pulp-platform.github.io/bender/init -sSf | sh
```

## Building Bendis

### Build in Release Mode (Recommended)

```bash
cd bendis
cargo build --release
```

The binary will be at: `target/release/bendis`

### Build in Debug Mode (For Development)

```bash
cd bendis
cargo build
```

The binary will be at: `target/debug/bendis`

## Installation

### Option 1: Add to PATH via settings.sh

Add this line to your `~/.bashrc` or `~/.zshrc`:

```bash
source /path/to/bendis/settings.sh
```

Then reload your shell:

```bash
source ~/.bashrc  # or source ~/.zshrc
```

### Option 2: Copy Binary to System Path

```bash
sudo cp bendis/target/release/bendis /usr/local/bin/
```

### Option 3: Create Symlink

```bash
sudo ln -s $(pwd)/bendis/target/release/bendis /usr/local/bin/bendis
```

## Verify Installation

```bash
bendis --version
bendis --help
```

## Quick Start

1. Navigate to your hardware project directory
2. Initialize Bendis:
   ```bash
   bendis init
   ```
3. Edit `.bendis/Bender.yml` with your dependencies
4. Run update:
   ```bash
   bendis update
   ```

## Development

### Running Tests

```bash
cd bendis
cargo test
```

### Checking Code

```bash
cd bendis
cargo check
```

### Formatting Code

```bash
cd bendis
cargo fmt
```

### Linting

```bash
cd bendis
cargo clippy
```

## Troubleshooting

### "bender: command not found"

Make sure Bender is installed and in your PATH.

### "cargo: command not found"

Rust is not installed. Follow the Rust installation steps above.

### Build errors

Try updating Rust:

```bash
rustup update
```

Clean and rebuild:

```bash
cd bendis
cargo clean
cargo build --release
```

## Performance Notes

- Release builds are ~10x faster than debug builds
- Always use `cargo build --release` for production use
- The format converter in Rust is significantly faster than the Python version

## Next Steps

After building, see [README.md](README.md) for usage instructions.
