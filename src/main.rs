#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// A Rust Type Generator for the ConvexDB Schema
//
// The main goal of this project is to convex our schema.ts file into rust types so that
// the database can be used in a type-safe manner in rust.

mod ast;
mod parser;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::ast::Table;

use oxc::allocator::Allocator;
use oxc::parser::Parser;
use oxc::span::SourceType;
use serde_json::Value;

fn main() {
    let file = env::args()
        .nth(1)
        .unwrap_or_else(|| "./convex/schema.ts".to_string());

    let allocator = Allocator::default();
    let source_type = SourceType::from_path(Path::new(&file)).unwrap();

    let schema_content = read_file_contents(&file).unwrap();

    let ret = Parser::new(&allocator, &schema_content, source_type).parse();

    if ret.errors.is_empty() {
        // let ast = serde_json::to_string_pretty(&ret.program).unwrap();
        let ast = serde_json::to_string(&ret.program).unwrap();

        // println!("{}", ast);

        let ast: Value = serde_json::from_str(&ast).unwrap();

        let tables = crate::parser::ASTParser::new(&ast).parse();
        println!("{:#?}", tables);

        // generate_rust_types(tables);

        println!("Parsed Successfully.");
    } else {
        for error in ret.errors {
            let error = error.with_source_code(schema_content.clone());
            println!("{error:?}");
        }
    }
}

fn read_file_contents(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn generate_rust_types(tables: Vec<Table>) {
    todo!()
}
