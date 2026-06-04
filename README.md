# makerom-rs

Rust FFI bindings for the [makerom](https://github.com/3DSGuy/Project_CTR/tree/master/makerom) utility, which creates 3DS ROM files in various formats (CCI, CIA, CXI, CFA).

## About makerom

makerom is a command-line utility for creating Nintendo 3DS ROM files. It supports:
- Building NCCH (Nintendo CTR Content Header) files
- Creating CCI (CTR Card Image) files
- Generating CIA (CTR Importable Archive) files
- Exporting raw CXI/CFA files

## Features

- Safe Rust wrappers around C FFI functions
- Support for user settings and configuration
- Command-line argument parsing
- RSF (ROM Specification File) settings loading
- Build operations for various 3DS formats

## Building from Source

### Prerequisites

1. Rust 1.70+ installed via [rustup](https://rustup.rs/)
2. A C compiler (gcc, clang, or MSVC)
3. The makerom C library compiled

### Compilation Steps

1. Clone this repository:
   ```bash
   git clone https://github.com/TurboToad12/makerom-rs.git
   cd makerom-rs
   ```

2. Clone the Project_CTR repository to get the makerom source:
   ```bash
   git clone https://github.com/3DSGuy/Project_CTR.git
   ```

3. Update `build.rs` to point to your makerom library:
   ```rust
   println!("cargo:rustc-link-search=native=/path/to/your/makerom/build");
   println!("cargo:rustc-link-lib=makerom");
   ```

4. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Basic Example

```rust
use makerom::UserSettings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new settings instance
    let settings = UserSettings::new()?;
    
    // Use the settings to configure and build a 3DS ROM
    // (More examples coming as the library develops)
    
    Ok(())
}
```

## Project Structure

```
makerom-rs/
├── Cargo.toml           # Rust project manifest
├── build.rs             # Build script for FFI bindings generation
├── makerom-headers.h    # Header wrapper for bindgen
├── src/
│   └── lib.rs          # Main library code with FFI wrappers
├── examples/
│   └── basic_usage.rs  # Basic usage examples
└── README.md           # This file
```

## FFI Bindings

This crate uses [bindgen](https://github.com/rust-lang/rust-bindgen) to automatically generate FFI bindings from the makerom C headers. The bindings are generated at compile time and included in the binary.

## Safety

While this library provides some safe wrappers, many FFI functions remain unsafe by nature. Users should be familiar with C conventions and the makerom documentation before using advanced features.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Related Projects

- [Project_CTR](https://github.com/3DSGuy/Project_CTR) - Original makerom source code
- [bindgen](https://github.com/rust-lang/rust-bindgen) - Automated FFI binding generator

## Disclaimer

This is an unofficial binding. Please refer to the original makerom documentation and the Project_CTR repository for authoritative information.
