use std::path::PathBuf;
use std::vec;

fn build_grammar(name: &str) {
    let dir: PathBuf = ["grammars", name, "src"].iter().collect();

    let mut c_sources: Vec<PathBuf> = vec![];
    let c_candidates = vec!["parser.c", "scanner.c"];
    for name in c_candidates {
        let file = dir.join(name);
        if file.exists() {
            c_sources.push(file);
        }
    }
    if !c_sources.is_empty() {
        cc::Build::new()
            .include(&dir)
            .files(c_sources)
            .warnings(false)
            .compile(&format!("{}-c", name));
    }

    let mut cpp_sources: Vec<PathBuf> = vec![];
    let cpp_candidates = vec!["parser.cc", "scanner.cc"];
    for name in cpp_candidates {
        let file = dir.join(name);
        if file.exists() {
            cpp_sources.push(file);
        }
    }
    if !cpp_sources.is_empty() {
        cc::Build::new()
            .cpp(true)
            .include(&dir)
            .files(cpp_sources)
            .warnings(false)
            .compile(&format!("{}-cpp", name));
    }
}

fn main() {
    build_grammar("tree-sitter-c-sharp");
    build_grammar("tree-sitter-cpp");
    build_grammar("tree-sitter-go");
    build_grammar("tree-sitter-java");
    build_grammar("tree-sitter-javascript");
    build_grammar("tree-sitter-python");
    build_grammar("tree-sitter-rust");
}
