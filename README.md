# cr-prep

[![Crates.io](https://img.shields.io/crates/v/cr-prep.svg)](https://crates.io/crates/cr-prep)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/kyuki3rain/cr-prep/workflows/CI/badge.svg)](https://github.com/kyuki3rain/cr-prep/actions)

A CLI tool for collecting code files for code review. This tool recursively searches through a specified directory and outputs the content of code files in a Markdown format suitable for code review with LLMs.

## Installation

You can install `cr-prep` using cargo:

```bash
cargo install cr-prep
```

Or build from source:

```bash
git clone https://github.com/kyuki3rain/cr-prep
cd cr-prep
cargo build --release
```

## Usage

Basic usage:

```bash
# Output to stdout
cr-prep --path /path/to/your/project

# Output to a file
cr-prep --path /path/to/your/project --output review.md
```

### Options

- `-p, --path <DIRECTORY>`: Directory to search for code files (required)
- `-o, --output <FILE>`: Output file path (optional, defaults to stdout)

### Example Output

The output is in Markdown format with each file wrapped in a code block:

```
## src/main.rs
\`\`\`
fn main() {
    println!("Hello, world!");
}
\`\`\`

## src/lib.rs
\`\`\`
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
\`\`\`
```

## Supported File Types

- Rust (`.rs`)
- TypeScript (`.ts`)
- JavaScript (`.js`)
- Python (`.py`)
- Go (`.go`)

## Error Handling

- If the specified path is not a directory, the program will exit with an error message
- If a file cannot be read, a warning message will be displayed and the file will be skipped
- If the output file cannot be written, the program will exit with an error message

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
