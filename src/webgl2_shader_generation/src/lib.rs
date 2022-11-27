use proc_macro::TokenStream;
use quote::quote;
use std::{
    collections::HashMap,
    fs,
    path::{self, PathBuf},
};
use syn::{parse_macro_input, LitStr};
use webgl2_shader_definition::*;

// Checks that the `import_file_path` is valid; if so, add the file to `promise_imports` and `function_imports`.
fn push_file_function_imports(
    function_imports: &mut HashMap<String, FunctionImports>,
    promise_imports: &mut Vec<(String, String)>,
    import_file_path: PathBuf,
    path: String,
    fn_name: String,
) {
    match fs::read_to_string(import_file_path) {
        Ok(file_string) => match serde_yaml::from_str(&file_string) {
            Ok(fn_imports) => {
                function_imports.insert(path.clone(), fn_imports);
                promise_imports.push((path.clone(), fn_name));
            }
            Err(err) => {
                panic!("Could not deserialize imported function text {}", err);
            }
        },
        Err(err) => {
            panic!(
                "Could not read imported file |path: {}| |error: {}|",
                path, err
            );
        }
    }
}

// Takes an array of Function Definitions and the path of the shader and transforms them
// into an array of Function Definitions with type InlineFn. For the InlineFn functions in the
// array is a simple copy, for the ImportFn, the necessary file is read.
fn generate_imported_functions(
    source_imported_fn: &Vec<FunctionDefinition>,
    file_dir: &path::Path,
) -> Vec<FunctionDefinition> {
    let mut imported_fn = Vec::<FunctionDefinition>::new();
    let mut function_imports = HashMap::<String, FunctionImports>::new();
    let mut promise_imports = Vec::<(String, String)>::new();

    for import_fn in source_imported_fn {
        match &import_fn.definition {
            FunctionDefinitionType::InlineFn { .. } => {
                imported_fn.push(import_fn.clone());
            }
            FunctionDefinitionType::ImportFn { path } => {
                let import_file_path = file_dir.join(path.clone());
                if !function_imports.contains_key(path) {
                    push_file_function_imports(
                        &mut function_imports,
                        &mut promise_imports,
                        import_file_path,
                        path.clone(),
                        import_fn.name.clone(),
                    );
                } else {
                    promise_imports.push((path.clone(), import_fn.name.clone()));
                }
            }
        }
    }

    for promise in promise_imports {
        match function_imports.get(&promise.0) {
            Some(fn_imports) => {
                match fn_imports
                    .function_definitions
                    .iter()
                    .find(|def| def.name == promise.1)
                {
                    Some(fn_def) => imported_fn.push(fn_def.clone()),
                    None => {
                        panic!("Could not find function definition inside the imported file | path: {}", promise.0);
                    }
                }
            }
            None => {
                panic!("File was not imported | path: {}", promise.0);
            }
        }
    }

    imported_fn
}

// Reads the YAML shader definition in the given path and returns the compiled String of the vertex
// shader and the fragment shader (in that order).
fn generate_shader_str(file_path: String) -> (String, String) {
    let f_path = path::Path::new(&file_path);

    if f_path.is_file() && !f_path.is_absolute() {
        panic!("File path should be absolute and point to a file")
    }

    let file_directory = f_path.parent().unwrap();

    match fs::read_to_string(f_path) {
        Ok(file_str) => match toml::from_str::<ShaderSource>(&file_str) {
            Ok(source) => {
                let imported_fn =
                    generate_imported_functions(&source.imported_functions, file_directory);
                let vertex_shader_code = generate_vertex_stage_str(&source, &imported_fn);
                let fragment_shader_code = generate_fragment_stage_str(&source, &imported_fn);
                (vertex_shader_code, fragment_shader_code)
            }
            Err(err) => {
                panic!("Cannot deserialize shader source from file | error {}", err)
            }
        },
        Err(err) => {
            panic!("Cannot read file | error {}", err)
        }
    }
}

#[proc_macro]
pub fn generate_shader_from_yaml_file(input: TokenStream) -> TokenStream {
    let file_path: LitStr = parse_macro_input!(input as LitStr);
    let path = file_path.value();
    let file_path = path::Path::new(&path);

    let file_directory = file_path.parent().unwrap();
    let file_str = fs::read_to_string(file_path).expect("Could not read the shader file");

    let source: ShaderSource = serde_yaml::from_str(&file_str).expect("Could not deserialize");

    let imported_fn = generate_imported_functions(&source.imported_functions, file_directory);
    let vertex_shader = generate_vertex_stage_str(&source, &imported_fn);
    let fragment_shader = generate_fragment_stage_str(&source, &imported_fn);

    let expanded = quote! {
        (#vertex_shader, #fragment_shader)
    };
    return TokenStream::from(expanded);
}

#[proc_macro]
pub fn generate_shader_from_file(input: TokenStream) -> TokenStream {
    let file_path: LitStr = parse_macro_input!(input as LitStr);
    let path = file_path.value();

    let (vert_shader, frag_shader) = generate_shader_str(path.clone());

    let expanded = quote! {
        (#vert_shader, #frag_shader)
    };
    return TokenStream::from(expanded);
}
