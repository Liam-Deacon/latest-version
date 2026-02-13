# latest-version

Find the latest version of commands across all available paths. A cross-platform command-line tool written in Rust, with optional Python bindings.

## Problem Statement

When you have multiple versions of the same command installed on your system, it can be challenging to determine which version is the latest. This tool solves that problem by:

1. Finding all available paths to the command
2. Extracting version information from each executable
3. Comparing versions to determine the latest
4. Returning the path to the latest version

## Features

- **Cross-platform**: Works on Windows, macOS, and Linux
- **Fast and efficient**: Written in Rust for performance
- **Robust version detection**: Handles various version string formats
- **Multiple installation options**: pipx, Homebrew, Scoop, or direct binary
- **Python bindings**: Optional Python API for integration into Python scripts

## Installation

### Option 1: pipx (Python wrapper)

```bash
pipx install latest-version
```

### Option 2: Homebrew (macOS)

```bash
brew install liam-deacon/homebrew-tap/latest-version
```

### Option 3: Scoop (Windows)

```bash
scoop bucket add liam-deacon https://github.com/Liam-Deacon/scoop-bucket
scoop install latest-version
```

### Option 4: Binary Releases

Download the latest binary from the [Releases page](https://github.com/Liam-Deacon/latest-version/releases) and add it to your system PATH.

## Usage

### Command-line interface

```bash
latest-version <command>
```

#### Examples:

```bash
# Find the latest version of Python
latest-version python3

# Find the latest version of Node.js
latest-version node

# Find the latest version of GCC
latest-version gcc
```

### Python API

```python
from latest_version import find_latest_command

try:
    result = find_latest_command("python3")
    print(f"Latest Python version is at: {result.path}")
    print(f"Version: {result.version}")
except Exception as e:
    print(f"Error: {e}")
```

## How it works

1. **Finding executables**: Uses Rust's `which` crate to locate all executable files in the system PATH that match the command name
2. **Extracting version information**: Runs each executable with `--version`, `-v`, or `-V` flags and parses the output
3. **Comparing versions**: First attempts strict semantic version comparison using the `semver` crate, falls back to flexible comparison using `version-compare`
4. **Returning result**: Returns the path to the executable with the latest version

## Version Detection

The tool tries to extract version information using these methods (in order):

1. **Strict Semantic Versioning**: Matches versions like `1.0.0`, `2.3.4`, `0.5.2`
2. **Major.Minor format**: Matches versions like `1.0`, `2.3` and converts to `1.0.0`, `2.3.0`
3. **Major format**: Matches versions like `1`, `2` and converts to `1.0.0`, `2.0.0`
4. **Fallback comparison**: If semantic version parsing fails, it uses a flexible comparison algorithm

## Development

### Prerequisites

- Rust 1.70 or later
- Python 3.8 or later (for Python bindings)
- maturin (for building Python packages)

### Building from source

```bash
# Clone the repository
git clone https://github.com/Liam-Deacon/latest-version.git
cd latest-version

# Build the Rust binary
cargo build --release

# Run tests
cargo test

# Build and install the Python package
pip install maturin
maturin develop
```

### Running tests

```bash
# Run Rust tests
cargo test

# Run Python tests
pytest tests/
```

## Contributing

Contributions are welcome! Please feel free to:

1. Open an issue to report bugs or suggest features
2. Submit a pull request with improvements or bug fixes
3. Improve the documentation

## License

MIT License

## Contact

Created by Liam Deacon. Feel free to reach out with questions or suggestions.