use std::str::FromStr;

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_cpp() -> Language;
    fn tree_sitter_go() -> Language;
    fn tree_sitter_python() -> Language;
}

#[derive(Debug, PartialEq)]
pub enum LanguageEnum {
    Cpp,
    Go,
    Python,
}

impl LanguageEnum {
    pub fn extensions(&self) -> Vec<&str> {
        match self {
            LanguageEnum::Cpp => vec![".cpp", ".cc", ".cxx", ".h", ".hpp", ".hxx"],
            LanguageEnum::Go => vec![".go"],
            LanguageEnum::Python => vec![".py"],
        }
    }

    pub fn language(&self) -> Language {
        unsafe {
            match self {
                LanguageEnum::Cpp => tree_sitter_cpp(),
                LanguageEnum::Go => tree_sitter_go(),
                LanguageEnum::Python => tree_sitter_python(),
            }
        }
    }
}

impl FromStr for LanguageEnum {
    type Err = ();

    fn from_str(input: &str) -> Result<LanguageEnum, Self::Err> {
        match input.to_lowercase().as_ref() {
            "cpp" => Ok(LanguageEnum::Cpp),
            "go" => Ok(LanguageEnum::Go),
            "python" => Ok(LanguageEnum::Python),
            _ => Err(()),
        }
    }
}
