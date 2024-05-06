# Simple Password Generator

A Rust-based password generator that generates random and secure passwords with various configurable options. The generator can output passwords directly to the clipboard for easy use.

## Features

- **Customizable Character Types**: Choose to include/exclude lowercase, uppercase, digits, and special characters.
- **Custom Length**: Generate passwords of customizable lengths.
- **Guaranteed Character Distribution**: Ensure an equal number of characters from each character group.
- **Clipboard Support**: Automatically copy the generated password to your system clipboard.

## Installation

Clone the repository:

```bash
git clone \<repository-url\>
```

Navigate to the project directory:

```bash
cd password-generator
```

Build the project:

```bash
cargo build --release
```

## Usage

To run the program and generate a password with default settings:

```bash
cargo run --release
```

## Configuration

You can modify the password generator directly in `main.rs` by adjusting the parameters passed to `PasswordGenerator::new`.

Example:

```rust
// src/main.rs

let generator = PasswordGenerator::new(20, true, true, true, true);
let password = generator.generate();
println!("Generated password: {}", password);
```

This example generates a 20-character password including lowercase, uppercase, digits, and special characters.

## Testing

To run tests:

```bash
cargo test
```

The tests include various scenarios to ensure the generator is working correctly.

## Contributions

Contributions are welcome! Please fork this repository and submit a pull request with your changes.
