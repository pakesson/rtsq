use std::fs;
use std::str::FromStr;

use clap::{App, Arg};
use tree_sitter::{Parser, Query, QueryCursor};
use walkdir::WalkDir;

mod grammar;
use grammar::LanguageEnum;

fn main() -> Result<(), std::io::Error> {
    let opts = App::new("rtsq")
        .arg(
            Arg::with_name("query")
                .help("Sets the search query")
                .short("q")
                .long("query")
                .value_name("QUERY_STRING")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("language")
                .help("Sets the language to use")
                .short("l")
                .long("language")
                .value_name("LANGUAGE_NAME")
                .possible_values(&["python"])
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .help("Set the path to search (defaults to the current directory")
                .index(1)
                .required(false),
        )
        .get_matches();

    let language_string = opts.value_of("language").unwrap();
    let query_string = opts.value_of("query").unwrap();
    let path = opts.value_of("path").unwrap_or(".");

    let language_value = LanguageEnum::from_str(language_string).unwrap();
    let file_extensions = language_value.extensions();
    let language = language_value.language();

    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let query = Query::new(language, query_string).expect("could not create query");

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();

        if file_extensions.iter().any(|ext| filename.ends_with(ext)) {
            let contents = fs::read_to_string(entry.path())?;

            if let Some(tree) = parser.parse(&contents, None) {
                let mut cursor = QueryCursor::new();
                let matches = cursor.matches(&query, tree.root_node(), contents.as_bytes());

                for m in matches {
                    for c in m.captures {
                        let capture_name = query.capture_names()[c.index as usize].as_str();
                        let start = c.node.start_position();
                        println!(
                            "{}:{} matches {}",
                            entry.path().display(),
                            start.row,
                            capture_name
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
