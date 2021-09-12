use std::str::FromStr;

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_c_sharp() -> Language;
    fn tree_sitter_cpp() -> Language;
    fn tree_sitter_java() -> Language;
    fn tree_sitter_javascript() -> Language;
    fn tree_sitter_go() -> Language;
    fn tree_sitter_python() -> Language;
    fn tree_sitter_rust() -> Language;
}

#[derive(Debug, PartialEq)]
pub enum LanguageEnum {
    Cpp,
    Csharp,
    Go,
    Java,
    JavaScript,
    Python,
    Rust,
}

impl LanguageEnum {
    pub fn extensions(&self) -> Vec<&str> {
        match self {
            LanguageEnum::Cpp => vec![".cpp", ".cc", ".cxx", ".h", ".hpp", ".hxx"],
            LanguageEnum::Csharp => vec![".cs"],
            LanguageEnum::Go => vec![".go"],
            LanguageEnum::Java => vec![".java"],
            LanguageEnum::JavaScript => vec![".js"],
            LanguageEnum::Python => vec![".py"],
            LanguageEnum::Rust => vec![".rs"],
        }
    }

    pub fn language(&self) -> Language {
        unsafe {
            match self {
                LanguageEnum::Cpp => tree_sitter_cpp(),
                LanguageEnum::Csharp => tree_sitter_c_sharp(),
                LanguageEnum::Go => tree_sitter_go(),
                LanguageEnum::Java => tree_sitter_java(),
                LanguageEnum::JavaScript => tree_sitter_javascript(),
                LanguageEnum::Python => tree_sitter_python(),
                LanguageEnum::Rust => tree_sitter_rust(),
            }
        }
    }
}

impl FromStr for LanguageEnum {
    type Err = ();

    fn from_str(input: &str) -> Result<LanguageEnum, Self::Err> {
        match input.to_lowercase().as_ref() {
            "cpp" => Ok(LanguageEnum::Cpp),
            "csharp" => Ok(LanguageEnum::Csharp),
            "go" => Ok(LanguageEnum::Go),
            "java" => Ok(LanguageEnum::Java),
            "javascript" => Ok(LanguageEnum::JavaScript),
            "python" => Ok(LanguageEnum::Python),
            "rust" => Ok(LanguageEnum::Rust),
            _ => Err(()),
        }
    }
}
