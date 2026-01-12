use anyhow::Result;
use ignore::WalkBuilder;
use std::path::Path;

/// Mapping of file extension to language name for syntax highlighting
const FILE_TYPES: &[(&str, &str)] = &[
    // Rust
    ("rs", "rust"),
    // TypeScript/JavaScript
    ("ts", "typescript"),
    ("tsx", "tsx"),
    ("js", "javascript"),
    ("jsx", "jsx"),
    ("mjs", "javascript"),
    ("cjs", "javascript"),
    // Python
    ("py", "python"),
    // Go
    ("go", "go"),
    // Java/JVM
    ("java", "java"),
    ("kt", "kotlin"),
    ("kts", "kotlin"),
    ("scala", "scala"),
    // C/C++
    ("c", "c"),
    ("h", "c"),
    ("cpp", "cpp"),
    ("cc", "cpp"),
    ("cxx", "cpp"),
    ("hpp", "cpp"),
    ("hxx", "cpp"),
    // C#
    ("cs", "csharp"),
    // Ruby
    ("rb", "ruby"),
    // PHP
    ("php", "php"),
    // Swift
    ("swift", "swift"),
    // Shell
    ("sh", "bash"),
    ("bash", "bash"),
    ("zsh", "zsh"),
    // Web
    ("html", "html"),
    ("htm", "html"),
    ("css", "css"),
    ("scss", "scss"),
    ("sass", "sass"),
    ("less", "less"),
    ("vue", "vue"),
    ("svelte", "svelte"),
    // Data/Config
    ("json", "json"),
    ("yaml", "yaml"),
    ("yml", "yaml"),
    ("toml", "toml"),
    ("xml", "xml"),
    // SQL
    ("sql", "sql"),
    // GraphQL
    ("graphql", "graphql"),
    ("gql", "graphql"),
    // Protocol Buffers
    ("proto", "protobuf"),
    // Markdown
    ("md", "markdown"),
    ("mdx", "mdx"),
    // Elixir/Erlang
    ("ex", "elixir"),
    ("exs", "elixir"),
    ("erl", "erlang"),
    // Haskell
    ("hs", "haskell"),
    // Lua
    ("lua", "lua"),
    // Zig
    ("zig", "zig"),
    // Nim
    ("nim", "nim"),
    // OCaml
    ("ml", "ocaml"),
    ("mli", "ocaml"),
    // F#
    ("fs", "fsharp"),
    ("fsi", "fsharp"),
    ("fsx", "fsharp"),
    // Dart
    ("dart", "dart"),
    // R
    ("r", "r"),
    ("R", "r"),
    // Julia
    ("jl", "julia"),
    // Clojure
    ("clj", "clojure"),
    ("cljs", "clojure"),
    ("cljc", "clojure"),
];

/// Get the language name for syntax highlighting from a file extension
pub fn get_language(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?;
    FILE_TYPES
        .iter()
        .find(|(e, _)| *e == ext)
        .map(|(_, lang)| *lang)
}

/// Check if a file is a supported target file
pub fn is_target_file(path: &Path) -> bool {
    get_language(path).is_some()
}

/// Process a single file and return formatted output with syntax highlighting
pub fn process_file(file_path: &Path, base_path: &Path) -> Result<String> {
    use anyhow::Context;
    use std::fs;

    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    let relative_path = file_path
        .strip_prefix(base_path)
        .with_context(|| format!("Failed to get relative path for: {}", file_path.display()))?;

    let language = get_language(file_path).unwrap_or("");

    Ok(format!(
        "## {}\n```{}\n{}\n```\n\n",
        relative_path.display(),
        language,
        content.trim_end()
    ))
}

/// Iterator over files respecting .gitignore
pub fn walk_files(path: &Path) -> impl Iterator<Item = std::path::PathBuf> {
    WalkBuilder::new(path)
        .hidden(true) // Respect hidden files settings
        .git_ignore(true) // Respect .gitignore
        .git_global(true) // Respect global gitignore
        .git_exclude(true) // Respect .git/info/exclude
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| is_target_file(e.path()))
        .map(|e| e.path().to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_is_target_file() {
        let test_cases = vec![
            // Original supported types
            ("test.rs", true),
            ("test.ts", true),
            ("test.js", true),
            ("test.py", true),
            ("test.go", true),
            // New supported types
            ("test.tsx", true),
            ("test.jsx", true),
            ("test.java", true),
            ("test.c", true),
            ("test.cpp", true),
            ("test.h", true),
            ("test.rb", true),
            ("test.php", true),
            ("test.swift", true),
            ("test.kt", true),
            ("test.cs", true),
            ("test.sh", true),
            ("test.yaml", true),
            ("test.yml", true),
            ("test.toml", true),
            ("test.json", true),
            ("test.md", true),
            ("test.html", true),
            ("test.css", true),
            ("test.sql", true),
            // Unsupported
            ("test.txt", false),
            ("test", false),
            ("test.RS", false), // Case sensitive
        ];

        for (file_name, expected) in test_cases {
            let path = PathBuf::from(file_name);
            assert_eq!(is_target_file(&path), expected, "Testing {}", file_name);
        }
    }

    #[test]
    fn test_get_language() {
        let test_cases = vec![
            ("test.rs", Some("rust")),
            ("test.ts", Some("typescript")),
            ("test.tsx", Some("tsx")),
            ("test.js", Some("javascript")),
            ("test.py", Some("python")),
            ("test.go", Some("go")),
            ("test.java", Some("java")),
            ("test.cpp", Some("cpp")),
            ("test.txt", None),
        ];

        for (file_name, expected) in test_cases {
            let path = PathBuf::from(file_name);
            assert_eq!(get_language(&path), expected, "Testing {}", file_name);
        }
    }

    #[test]
    fn test_process_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path();
        let file_path = base_path.join("test.rs");

        let test_content = "fn main() {\n    println!(\"Hello\");\n}";
        let mut file = File::create(&file_path)?;
        file.write_all(test_content.as_bytes())?;

        let result = process_file(&file_path, base_path)?;
        let expected = format!("## test.rs\n```rust\n{}\n```\n\n", test_content);

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_process_file_typescript() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path();
        let file_path = base_path.join("app.tsx");

        let test_content = "export const App = () => <div>Hello</div>;";
        let mut file = File::create(&file_path)?;
        file.write_all(test_content.as_bytes())?;

        let result = process_file(&file_path, base_path)?;
        assert!(result.contains("```tsx"));
        Ok(())
    }

    #[test]
    fn test_process_file_not_found() {
        let base_path = Path::new(".");
        let file_path = Path::new("nonexistent.rs");

        assert!(process_file(file_path, base_path).is_err());
    }

    #[test]
    fn test_walk_files_respects_gitignore() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path();

        // Initialize a git repository so .gitignore is respected
        std::fs::create_dir(base_path.join(".git"))?;

        // Create .gitignore
        let gitignore_path = base_path.join(".gitignore");
        let mut gitignore = File::create(&gitignore_path)?;
        writeln!(gitignore, "ignored/")?;
        writeln!(gitignore, "*.log")?;

        // Create files
        let included_file = base_path.join("main.rs");
        File::create(&included_file)?;

        let ignored_dir = base_path.join("ignored");
        std::fs::create_dir(&ignored_dir)?;
        let ignored_file = ignored_dir.join("ignored.rs");
        File::create(&ignored_file)?;

        let log_file = base_path.join("debug.log");
        File::create(&log_file)?;

        // Test walk_files
        let files: Vec<_> = walk_files(base_path).collect();

        assert!(files.iter().any(|p| p.ends_with("main.rs")));
        assert!(!files.iter().any(|p| p.ends_with("ignored.rs")));
        // Note: *.log is not a supported file type, so it won't be in the list anyway
        assert!(!files.iter().any(|p| p.ends_with("debug.log")));

        Ok(())
    }
}
