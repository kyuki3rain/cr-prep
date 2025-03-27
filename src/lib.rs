use anyhow::Result;
use std::path::PathBuf;

pub fn is_target_file(path: &PathBuf) -> bool {
    if let Some(extension) = path.extension() {
        matches!(
            extension.to_str(),
            Some("rs" | "ts" | "js" | "py" | "go")
        )
    } else {
        false
    }
}

pub fn process_file(file_path: &PathBuf, base_path: &PathBuf) -> Result<String> {
    use std::fs;
    use anyhow::Context;

    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;
    
    let relative_path = file_path
        .strip_prefix(base_path)
        .with_context(|| format!("Failed to get relative path for: {}", file_path.display()))?;
    
    Ok(format!(
        "## {}\n```\n{}\n```\n",
        relative_path.display(),
        content
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_is_target_file() {
        let test_cases = vec![
            ("test.rs", true),
            ("test.ts", true),
            ("test.js", true),
            ("test.py", true),
            ("test.go", true),
            ("test.txt", false),
            ("test", false),
            ("test.RS", false),  // Case sensitive
        ];

        for (file_name, expected) in test_cases {
            let path = PathBuf::from(file_name);
            assert_eq!(is_target_file(&path), expected, "Testing {}", file_name);
        }
    }

    #[test]
    fn test_process_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path().to_path_buf();
        let file_path = base_path.join("test.rs");

        let test_content = "fn main() {\n    println!(\"Hello\");\n}\n";
        let mut file = File::create(&file_path)?;
        file.write_all(test_content.as_bytes())?;

        let result = process_file(&file_path, &base_path)?;
        let expected = format!("## test.rs\n```\n{}\n```\n", test_content);

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_process_file_not_found() {
        let base_path = PathBuf::from(".");
        let file_path = PathBuf::from("nonexistent.rs");

        assert!(process_file(&file_path, &base_path).is_err());
    }
}