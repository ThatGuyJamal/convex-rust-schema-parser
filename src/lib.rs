mod codegen;
mod convex;
mod errors;

use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use codegen::generate_code;
use convex::{create_functions_ast, create_schema_ast, parse_function_ast, parse_schema_ast};
use errors::ConvexTypeGeneratorError;

/// Configuration options for the type generator.
#[derive(Debug, Clone)]
pub struct Configuration
{
    /// Path to the Convex schema file (default: "convex/schema.ts")
    pub schema_path: PathBuf,

    /// Output file path for generated Rust types (default: "src/convex_types.rs")
    pub out_file: String,

    /// Paths to Convex function files for generating function argument types
    pub function_paths: Vec<PathBuf>,
}

impl Default for Configuration
{
    fn default() -> Self
    {
        Self {
            schema_path: PathBuf::from("convex/schema.ts"),
            out_file: "src/convex_types.rs".to_string(),
            function_paths: Vec::new(),
        }
    }
}

/// Generates Rust types from Convex schema and function definitions.
///
/// # Arguments
/// * `config` - Configuration options for the type generation process
///
/// # Returns
/// * `Ok(())` if type generation succeeds
/// * `Err(ConvexTypeGeneratorError)` if an error occurs during generation
///
/// # Errors
/// This function can fail for several reasons:
/// * Schema file not found
/// * Invalid schema structure
/// * IO errors when reading/writing files
/// * Parse errors in schema or function files
pub fn generate(config: Configuration) -> Result<(), ConvexTypeGeneratorError>
{
    let start_time = Instant::now();

    let schema_path = config
        .schema_path
        .canonicalize()
        .map_err(|e| ConvexTypeGeneratorError::IOError {
            file: config.schema_path.to_string_lossy().to_string(),
            error: e,
        })?;

    let schema_ast = create_schema_ast(schema_path)?;
    let functions_ast = create_functions_ast(config.function_paths)?;

    write_to_file("./debug/schema_ast.json", &serde_json::to_string_pretty(&schema_ast).unwrap())?;
    write_to_file(
        "./debug/functions_ast.json",
        &serde_json::to_string_pretty(&functions_ast).unwrap(),
    )?;

    let parsed_schema = parse_schema_ast(schema_ast)?;
    let parsed_functions = parse_function_ast(functions_ast)?;

    write_to_file(
        "./debug/parsed_schema.json",
        &serde_json::to_string_pretty(&parsed_schema).unwrap(),
    )?;
    write_to_file(
        "./debug/parsed_functions.json",
        &serde_json::to_string_pretty(&parsed_functions).unwrap(),
    )?;

    generate_code(&config.out_file, (parsed_schema, parsed_functions))?;

    let elapsed = start_time.elapsed();
    println!("Convex Types generated in {}ms", elapsed.as_millis());

    Ok(())
}

// fn main()
// {
//     let config = Configuration {
//         function_paths: vec![PathBuf::from("./convex/test.ts"), PathBuf::from("./convex/test2.ts")],
//         ..Default::default()
//     };

//     match generate(config) {
//         Ok(_) => println!("Types generated successfully"),
//         Err(e) => println!("Error generating types: {}", e),
//     }
// }

#[cfg(debug_assertions)]
fn write_to_file(path: &str, content: &str) -> Result<(), std::io::Error>
{
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
