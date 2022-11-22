use std::borrow::Cow;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WebGLDataType {
    Void,

    Bool,
    Int,
    Uint,

    Float,
    Vec2,
    Vec3,
    Vec4,

    BVec2,
    BVec3,
    BVec4,

    IVec2,
    IVec3,
    IVec4,

    UVec2,
    UVec3,
    UVec4,

    Mat2,
    Mat3,
    Mat4,
}

impl WebGLDataType {
    pub fn as_str<'ret, 'b>(&'b self) -> &'ret str {
        match self {
            WebGLDataType::Void => "void",
            WebGLDataType::Bool => "bool",
            WebGLDataType::Int => "int",
            WebGLDataType::Uint => "uint",
            WebGLDataType::Float => "float",
            WebGLDataType::Vec2 => "vec2",
            WebGLDataType::Vec3 => "vec3",
            WebGLDataType::Vec4 => "vec4",

            WebGLDataType::BVec2 => "bvec2",
            WebGLDataType::BVec3 => "bvec3",
            WebGLDataType::BVec4 => "bvec4",

            WebGLDataType::IVec2 => "ivec2",
            WebGLDataType::IVec3 => "ivec3",
            WebGLDataType::IVec4 => "ivec4",

            WebGLDataType::UVec2 => "uvec2",
            WebGLDataType::UVec3 => "uvec3",
            WebGLDataType::UVec4 => "uvec4",

            WebGLDataType::Mat2 => "mat2",
            WebGLDataType::Mat3 => "mat3",
            WebGLDataType::Mat4 => "mat4",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderAttribute {
    pub layout_loc: u32,
    pub kind: WebGLDataType,
    pub name: String,
}

impl ShaderAttribute{
    pub fn get_default_frag_attribute()->Self{
        Self {
            layout_loc: 0,
            kind: WebGLDataType::Vec4,
            name: "frag_color".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderVarying {
    pub interp: Option<String>,
    pub kind: WebGLDataType,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ParameterQualifier {
    In,
    Out,
    InOut,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionParameter {
    pub qualifier: Option<ParameterQualifier>,
    pub kind: WebGLDataType,
    pub name: String,
    pub array_length: Option<u32>,
}

impl FunctionParameter {
    pub fn as_str(&self) -> String {
        let array_len: Cow<'_, str> = if self.array_length.is_none() {
            Cow::Borrowed("")
        } else {
            Cow::Owned(format!("[{}]", self.array_length.unwrap()))
        };
        match self.kind {
            WebGLDataType::Void => {
                panic!("Variable cannot be of type void")
            }
            _ => {
                let qualifier_str = {
                    if let Some(qualifier) = &self.qualifier {
                        match qualifier {
                            ParameterQualifier::In => "in",
                            ParameterQualifier::Out => "out",
                            ParameterQualifier::InOut => "inout",
                        }
                    } else {
                        ""
                    }
                };
                format!(
                    "{} {} {}{}",
                    qualifier_str,
                    self.kind.as_str(),
                    self.name,
                    array_len
                )
            }
        }
    }
}

#[allow(dead_code)]
fn create_fn_param_str(params: &Vec<FunctionParameter>) -> String {
    params
        .iter()
        .map(|param| param.as_str())
        .collect::<Vec<String>>()
        .join(" , ")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FunctionDefinitionType {
    InlineFn {
        return_type: WebGLDataType,
        parameters: String, //Vec<FunctionParameter>,
        body: String,
    },
    ImportFn {
        path: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub definition: FunctionDefinitionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderUniform {
    pub kind: WebGLDataType,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderUniformBlock {
    pub binding_number: u32,
    pub name: String,
    pub uniforms: Vec<ShaderUniform>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniformCollection {
    pub uniforms: Vec<ShaderUniform>,
    pub uniform_blocks: Vec<ShaderUniformBlock>,
}

impl UniformCollection {
    pub fn new() -> Self {
        Self {
            uniforms: vec![],
            uniform_blocks: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderStage {
    pub import_fn: Vec<String>,
    pub main_fn: String,
    pub attributes: Vec<ShaderAttribute>,
    pub uniform_collection: UniformCollection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionImports {
    pub function_definitions: Vec<FunctionDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderSource {
    pub name: String,
    pub varyings: Vec<ShaderVarying>,
    pub common_uniforms: UniformCollection,
    pub imported_functions: Vec<FunctionDefinition>,
    pub vertex_shader: ShaderStage,
    pub fragment_shader: ShaderStage,
}

pub fn shader_source_from_str(shader: &str) -> Result<ShaderSource, toml::de::Error> {
    match toml::from_str::<ShaderSource>(shader) {
        Ok(source) => Ok(source),
        Err(err) => Err(err),
    }
}

fn push_imported_function(
    shader_code: &mut String,
    import_fn: &Vec<String>,
    imported_functions: &Vec<FunctionDefinition>,
) {
    for imported_fn in import_fn {
        match imported_functions
            .iter()
            .find(|fn_def| fn_def.name == *imported_fn)
        {
            Some(imp_fn) => {
                if let FunctionDefinitionType::InlineFn {
                    return_type,
                    parameters,
                    body,
                } = &imp_fn.definition
                {
                    let mut imported_function_str = format!(
                        "{} {} ( {} ) {{\n",
                        return_type.as_str(),
                        imp_fn.name,
                        parameters
                    );
                    imported_function_str.extend(body.chars());
                    imported_function_str.push('\n');
                    imported_function_str.push('}');
                    imported_function_str.push('\n');

                    shader_code.extend(imported_function_str.chars());
                }
            }
            None => panic!("Function not found"),
        }
    }
}

fn push_uniform_collection(shader_code: &mut String, u_collection: &UniformCollection) {
    for uniform_block in &u_collection.uniform_blocks {
        let mut uniform_block_str = format!("layout (std140) uniform {} {{\n", uniform_block.name);
        for uniform in &uniform_block.uniforms {
            let uniform_line = format!(" {} {};\n", uniform.kind.as_str(), uniform.name);
            uniform_block_str.extend(uniform_line.chars());
        }
        uniform_block_str.push('}');
        uniform_block_str.push(';');
        uniform_block_str.push('\n');
        shader_code.extend(uniform_block_str.chars());
    }

    for uniform in &u_collection.uniforms {
        let uniform_line = format!("uniform {} {};\n", uniform.kind.as_str(), uniform.name);
        shader_code.extend(uniform_line.chars());
    }
}

fn push_main_function(shader_code: &mut String, main_fn: &String) {
    let mut main_function = format!("void main() {{\n");
    main_function.extend(main_fn.chars());
    main_function.push('\n');
    main_function.push('}');

    shader_code.extend(main_function.chars());
}

fn push_varying(shader_code: &mut String, is_input: bool, varyings: &Vec<ShaderVarying>) {
    for vary in varyings {
        if vary.interp.is_none() {
            let vary_str = format!(
                "{} {} {};\n",
                if is_input { "in" } else { "out" },
                vary.kind.as_str(),
                vary.name
            );
            shader_code.extend(vary_str.chars());
        } else {
            let vary_str = format!(
                "{} {} {} {};\n",
                vary.interp.as_ref().unwrap(),
                if is_input { "in" } else { "out" },
                vary.kind.as_str(),
                vary.name
            );
            shader_code.extend(vary_str.chars());
        }
    }
}

fn push_stage_attributes(
    shader_code: &mut String,
    attributes: &Vec<ShaderAttribute>,
    is_input: bool,
) {
    for attr in attributes {
        let attr_line = format!(
            "layout(location = {}) {} {} {};\n",
            attr.layout_loc,
            if is_input { "in" } else { "out" },
            attr.kind.as_str(),
            attr.name
        );
        shader_code.extend(attr_line.as_str().chars());
    }
}

pub fn generate_fragment_stage_str(
    shader_src: &ShaderSource,
    imported_functions: &Vec<FunctionDefinition>,
) -> String {
    let mut shader_code = String::from("#version 300 es\nprecision highp float;\n");

    push_stage_attributes(
        &mut shader_code,
        &shader_src.fragment_shader.attributes,
        false,
    );
    push_varying(&mut shader_code, true, &shader_src.varyings);
    push_uniform_collection(&mut shader_code, &shader_src.common_uniforms);
    push_uniform_collection(
        &mut shader_code,
        &shader_src.fragment_shader.uniform_collection,
    );
    push_imported_function(
        &mut shader_code,
        &shader_src.fragment_shader.import_fn,
        imported_functions,
    );
    push_main_function(&mut shader_code, &shader_src.fragment_shader.main_fn);

    shader_code
}

pub fn generate_vertex_stage_str(
    shader_src: &ShaderSource,
    imported_functions: &Vec<FunctionDefinition>,
) -> String {
    let mut shader_code = String::from("#version 300 es\nprecision highp float;\n");

    push_stage_attributes(&mut shader_code, &shader_src.vertex_shader.attributes, true);
    push_varying(&mut shader_code, false, &shader_src.varyings);
    push_uniform_collection(&mut shader_code, &shader_src.common_uniforms);
    push_uniform_collection(
        &mut shader_code,
        &shader_src.vertex_shader.uniform_collection,
    );
    push_imported_function(
        &mut shader_code,
        &shader_src.vertex_shader.import_fn,
        imported_functions,
    );
    push_main_function(&mut shader_code, &shader_src.vertex_shader.main_fn);

    shader_code
}

pub fn generate_shader_str_from_single_source(
    source: &ShaderSource,
) -> Result<(String, String), ()> {
    let imported_fn = {
        let mut imported_fn_vec = Vec::new();
        for i_fn in &source.imported_functions {
            match i_fn.definition {
                FunctionDefinitionType::InlineFn { .. } => imported_fn_vec.push(i_fn.clone()),
                FunctionDefinitionType::ImportFn { .. } => return Err(()),
            }
        }
        imported_fn_vec
    };
    Ok(generate_vertex_fragment_shader(source, &imported_fn))
}

pub fn generate_vertex_fragment_shader(
    source: &ShaderSource,
    imported_fn: &Vec<FunctionDefinition>,
) -> (String, String) {
    let vertex_shader_code = generate_vertex_stage_str(&source, &imported_fn);
    let fragment_shader_code = generate_fragment_stage_str(&source, &imported_fn);

    (vertex_shader_code, fragment_shader_code)
}
