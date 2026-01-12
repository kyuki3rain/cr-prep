# cr-prep

[![Crates.io](https://img.shields.io/crates/v/cr-prep.svg)](https://crates.io/crates/cr-prep)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/kyuki3rain/cr-prep/workflows/CI/badge.svg)](https://github.com/kyuki3rain/cr-prep/actions)

A CLI tool for collecting code files for code review. This tool recursively searches through a specified directory and outputs the content of code files in a Markdown format suitable for code review with LLMs.

## Features

- Outputs code with **syntax highlighting** support (language-specific code blocks)
- **Respects `.gitignore`** - automatically excludes files matching gitignore patterns
- Supports **50+ file types** including all major programming languages
- Cross-platform support (Linux, macOS, Windows)

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

The output is in Markdown format with each file wrapped in a language-specific code block:

```
## src/main.rs
\`\`\`rust
fn main() {
    println!("Hello, world!");
}
\`\`\`

## src/app.tsx
\`\`\`tsx
export const App = () => <div>Hello</div>;
\`\`\`
```

## Supported File Types

### Programming Languages

| Language | Extensions |
|----------|------------|
| Rust | `.rs` |
| TypeScript | `.ts`, `.tsx` |
| JavaScript | `.js`, `.jsx`, `.mjs`, `.cjs` |
| Python | `.py` |
| Go | `.go` |
| Java | `.java` |
| Kotlin | `.kt`, `.kts` |
| Scala | `.scala` |
| C/C++ | `.c`, `.h`, `.cpp`, `.cc`, `.cxx`, `.hpp`, `.hxx` |
| C# | `.cs` |
| Ruby | `.rb` |
| PHP | `.php` |
| Swift | `.swift` |
| Dart | `.dart` |
| Elixir | `.ex`, `.exs` |
| Erlang | `.erl` |
| Haskell | `.hs` |
| OCaml | `.ml`, `.mli` |
| F# | `.fs`, `.fsi`, `.fsx` |
| Clojure | `.clj`, `.cljs`, `.cljc` |
| Lua | `.lua` |
| Zig | `.zig` |
| Nim | `.nim` |
| R | `.r`, `.R` |
| Julia | `.jl` |

### Web & Markup

| Type | Extensions |
|------|------------|
| HTML | `.html`, `.htm` |
| CSS | `.css`, `.scss`, `.sass`, `.less` |
| Vue | `.vue` |
| Svelte | `.svelte` |
| Markdown | `.md`, `.mdx` |

### Data & Config

| Type | Extensions |
|------|------------|
| JSON | `.json` |
| YAML | `.yaml`, `.yml` |
| TOML | `.toml` |
| XML | `.xml` |
| SQL | `.sql` |
| GraphQL | `.graphql`, `.gql` |
| Protocol Buffers | `.proto` |

### Shell

| Type | Extensions |
|------|------------|
| Bash | `.sh`, `.bash` |
| Zsh | `.zsh` |

## Gitignore Support

cr-prep automatically respects `.gitignore` files in your project. Files and directories matching gitignore patterns will be excluded from the output. This includes:

- `.gitignore` in the project root and subdirectories
- Global gitignore settings
- `.git/info/exclude` patterns

## Error Handling

- If the specified path is not a directory, the program will exit with an error message
- If a file cannot be read, a warning message will be displayed and the file will be skipped
- If the output file cannot be written, the program will exit with an error message

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
