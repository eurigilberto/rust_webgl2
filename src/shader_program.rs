use glam::*;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::JsValue;
use web_sys::{WebGlProgram, WebGlUniformLocation};
use webgl2_shader_definition::ShaderUniform;

use crate::{GlShader, GlUniform, Graphics, IndexType, PrimitiveType};
use web_sys::WebGl2RenderingContext as wgl_context;

impl Drop for GlProgram {
    fn drop(&mut self) {
        self.context.delete_program(Some(&self.program));
    }
}

pub struct GlProgram {
    context: Rc<wgl_context>,
    pub program: WebGlProgram,
    pub uniforms: ProgramUniforms,
}

impl GlProgram {
    pub fn new(
        graphics: &Graphics,
        vertex_shader: &GlShader,
        fragment_shader: &GlShader,
    ) -> Result<Self, JsValue> {
        match graphics.create_gl_program(&vertex_shader.shader, &fragment_shader.shader) {
            Ok(program) => Ok(Self {
                context: graphics.gl_context.clone(),
                program,
                uniforms: ProgramUniforms::new(),
            }),
            Err(error) => Err(error),
        }
    }
    pub fn get_uniform_block_index(&self, uniform_block_name: &str) -> Result<u32, JsValue> {
        let index = self
            .context
            .get_uniform_block_index(&self.program, uniform_block_name);

        if index == wgl_context::INVALID_INDEX {
            Err(JsValue::from("Invalid Index"))
        } else {
            Ok(index)
        }
    }
    pub fn set_uniform_block_binding_str(
        &self,
        uniform_block_name: &str,
        block_binding_number: u32,
    ) {
        let index = self
            .get_uniform_block_index(uniform_block_name)
            .expect("Unable to get uniform block index");
        self.set_uniform_block_binding(index, block_binding_number);
    }
    pub fn set_uniform_block_binding(&self, uniform_block_index: u32, block_binding_number: u32) {
        self.context
            .uniform_block_binding(&self.program, uniform_block_index, block_binding_number)
    }
    pub fn get_uniform_location(&self, uniform_name: &str) -> Option<WebGlUniformLocation> {
        self.context
            .get_uniform_location(&self.program, uniform_name)
    }

    pub fn use_program(&mut self) -> ProgramInUse {
        self.context.use_program(Some(&self.program));
		ProgramInUse::new(self)
    }
    fn unuse_program(&self) {
        self.context.use_program(None);
    }

    pub fn insert_uniform(&mut self, uniform_name: &str, uniform: GlUniform) -> Result<UniformIndex, ()> {
        match self.get_uniform_location(uniform_name) {
            Some(location) => {
                let uniform_data = UniformData {
                    value: uniform,
                    location,
                    in_program: false,
                };
                Ok(self.uniforms.insert(uniform_name, uniform_data))
            }
            None => {
                panic!("Uniform - {} does not exist", uniform_name)
            }
        }
    }

    pub fn insert_shader_uniforms(&mut self, uniforms: &Vec<ShaderUniform>) -> Result<(), ()> {
        for uniform in uniforms {
            match self.insert_uniform(&uniform.name, GlUniform::from(uniform.kind)) {
                Ok(_) => {}
                Err(_) => return Err(()),
            }
        }
        Ok(())
    }
}

pub struct UniformSetter {
    context: Rc<wgl_context>,
}

impl UniformSetter {
    //float 32
    pub fn set_uniform_f32(&self, location: &WebGlUniformLocation, value: f32) {
        self.context.uniform1f(Some(&location), value);
    }
    pub fn set_uniform_vec2(&self, location: &WebGlUniformLocation, value: &Vec2) {
        self.context.uniform2f(Some(&location), value.x, value.y);
    }
    pub fn set_uniform_vec3(&self, location: &WebGlUniformLocation, value: &Vec3) {
        self.context
            .uniform3f(Some(&location), value.x, value.y, value.z);
    }
    pub fn set_uniform_vec4(&self, location: &WebGlUniformLocation, value: &Vec4) {
        self.context
            .uniform4f(Some(&location), value.x, value.y, value.z, value.w);
    }
    //int 32
    pub fn set_uniform_i32(&self, location: &WebGlUniformLocation, value: i32) {
        self.context.uniform1i(Some(&location), value);
    }
    pub fn set_uniform_ivec2(&self, location: &WebGlUniformLocation, value: &IVec2) {
        self.context.uniform2i(Some(&location), value.x, value.y);
    }
    pub fn set_uniform_ivec3(&self, location: &WebGlUniformLocation, value: &IVec3) {
        self.context
            .uniform3i(Some(&location), value.x, value.y, value.z);
    }
    pub fn set_uniform_ivec4(&self, location: &WebGlUniformLocation, value: &IVec4) {
        self.context
            .uniform4i(Some(&location), value.x, value.y, value.z, value.w);
    }
    //unsigned int 32
    pub fn set_uniform_u32(&self, location: &WebGlUniformLocation, value: u32) {
        self.context.uniform1ui(Some(&location), value);
    }
    pub fn set_uniform_uvec2(&self, location: &WebGlUniformLocation, value: &UVec2) {
        self.context.uniform2ui(Some(&location), value.x, value.y);
    }
    pub fn set_uniform_uvec3(&self, location: &WebGlUniformLocation, value: &UVec3) {
        self.context
            .uniform3ui(Some(&location), value.x, value.y, value.z);
    }
    pub fn set_uniform_uvec4(&self, location: &WebGlUniformLocation, value: &UVec4) {
        self.context
            .uniform4ui(Some(&location), value.x, value.y, value.z, value.w);
    }
    //float 32 matrix
    pub fn set_uniform_mat2(&self, location: &WebGlUniformLocation, value: &Mat2) {
        self.context.uniform_matrix2fv_with_f32_array(
            Some(&location),
            false,
            &value.to_cols_array(),
        );
    }
    pub fn set_uniform_mat3(&self, location: &WebGlUniformLocation, value: &Mat3) {
        self.context.uniform_matrix3fv_with_f32_array(
            Some(&location),
            false,
            &value.to_cols_array(),
        );
    }
    pub fn set_uniform_mat4(&self, location: &WebGlUniformLocation, value: &Mat4) {
        self.context.uniform_matrix4fv_with_f32_array(
            Some(&location),
            false,
            &value.to_cols_array(),
        );
    }
}

pub struct ProgramInUse<'a> {
    program: &'a mut GlProgram,
    uniform_setter: UniformSetter,
}

impl<'a> ProgramInUse<'a> {
    fn new(program: &'a mut GlProgram) -> Self {
        let uniform_setter = UniformSetter {
            context: Rc::clone(&program.context),
        };
        Self {
            program,
            uniform_setter,
        }
    }
}

impl ProgramInUse<'_> {
    pub fn draw_arrays(&self, primitive_type: PrimitiveType, first: u32, count: u32) {
        self.program
            .context
            .draw_arrays(primitive_type.into(), first as i32, count as i32);
    }
    pub fn draw_arrays_instanced(
        &self,
        primitive_type: PrimitiveType,
        first: u32,
        count: u32,
        instance_count: u32,
    ) {
        self.program.context.draw_arrays_instanced(
            primitive_type.into(),
            first as i32,
            count as i32,
            instance_count as i32,
        );
    }
    pub fn draw_elements_with_i32(
        &self,
        primitive_type: PrimitiveType,
        count: u32,
        index_type: IndexType,
        offset: u32,
    ) {
        self.program.context.draw_elements_with_i32(
            primitive_type.into(),
            count as i32,
            index_type.into(),
            offset as i32,
        );
    }
    pub fn draw_elements_instanced_with_i32(
        &self,
        primitive_type: PrimitiveType,
        count: u32,
        index_type: IndexType,
        offset: u32,
        instance_count: u32,
    ) {
        self.program.context.draw_elements_instanced_with_i32(
            primitive_type.into(),
            count as i32,
            index_type.into(),
            offset as i32,
            instance_count as i32,
        );
    }
    pub fn draw_range_elements_with_i32(
        &self,
        primitive_type: PrimitiveType,
        start: u32,
        end: u32,
        count: u32,
        index_type: IndexType,
        offset: u32,
    ) {
        self.program.context.draw_range_elements_with_i32(
            primitive_type.into(),
            start,
            end,
            count as i32,
            index_type.into(),
            offset as i32,
        )
    }
}

impl ProgramInUse<'_> {
    pub fn push_uniform(&mut self, uniform_index: UniformIndex) {
        self.program
            .uniforms
            .push_uniform(uniform_index, &self.uniform_setter);
    }
    pub fn push_all_uniforms(&mut self) {
        self.program
            .uniforms
            .push_all_uniforms(&self.uniform_setter);
    }
    pub fn push_uniform_by_name(&mut self, uniform_name: &str) {
        self.program
            .uniforms
            .push_uniform_by_name(uniform_name, &self.uniform_setter);
    }
    pub fn set_uniform(&mut self, uniform_index: UniformIndex, uniform_value: GlUniform) {
        self.program
            .uniforms
            .set_uniform(uniform_index, uniform_value);
        self.push_uniform(uniform_index);
    }
    pub fn set_uniform_by_name(&mut self, uniform_name: &str, uniform_value: GlUniform) {
        if let Some(uniform_index) = self.program.uniforms.get_uniform_index(uniform_name) {
            self.set_uniform(uniform_index, uniform_value);
        }
    }
}

impl Drop for ProgramInUse<'_> {
    fn drop(&mut self) {
        self.program.unuse_program();
    }
}

pub struct UniformData {
    value: GlUniform,
    location: WebGlUniformLocation,
    in_program: bool,
}

#[derive(Clone, Copy)]
pub struct UniformIndex(usize);

pub struct ProgramUniforms {
    uniforms: Vec<UniformData>,
    keys: HashMap<String, UniformIndex>,
}

impl ProgramUniforms {
    fn new() -> Self {
        Self {
            uniforms: Vec::new(),
            keys: HashMap::new(),
        }
    }

    fn insert(&mut self, uniform_name: &str, uniform_data: UniformData)->UniformIndex {
        let index = UniformIndex(self.uniforms.len());
        self.keys
            .insert(uniform_name.to_string(), index);
        self.uniforms.push(uniform_data);
        index
    }

    pub fn get_uniform_location_by_name(&self, name: &str) -> WebGlUniformLocation {
        let uniform_index = *self.keys.get(name).expect("Uniform does not exist");
        self.get_uniform_location(uniform_index)
    }

    pub fn get_uniform_location(&self, index: UniformIndex) -> WebGlUniformLocation {
        self.uniforms[index.0].location.clone()
    }

    pub fn get_uniform_index(&self, uniform_name: &str) -> Option<UniformIndex> {
        match self.keys.get(uniform_name) {
            Some(uniform_index) => Some(*uniform_index),
            None => None,
        }
    }

    fn push_all_uniforms(&mut self, uniform_setter: &UniformSetter) {
        for uniform in self.uniforms.iter_mut() {
            if !uniform.in_program {
                uniform.value.set_uniform(uniform_setter, &uniform.location);
                uniform.in_program = true;
            }
        }
    }

    fn push_uniform(&mut self, uniform_index: UniformIndex, uniform_setter: &UniformSetter) {
        let uniform = &mut self.uniforms[uniform_index.0];
        if !uniform.in_program {
            uniform.value.set_uniform(uniform_setter, &uniform.location);
            uniform.in_program = true;
        }
    }

    fn push_uniform_by_name(&mut self, uniform_name: &str, uniform_setter: &UniformSetter) {
        let uniform_index = *self.keys.get(uniform_name).expect("Cannot get name");
        self.push_uniform(uniform_index, uniform_setter)
    }

    pub fn set_uniform(&mut self, uniform_index: UniformIndex, uniform_value: GlUniform) {
        let uniform_data = &mut self.uniforms[uniform_index.0];
        if uniform_data.value.equal_variant(&uniform_value) {
            uniform_data.value = uniform_value;
            uniform_data.in_program = false;
        }
    }

    pub fn set_uniform_by_name(&mut self, uniform_name: &str, uniform_value: GlUniform) {
        let uniform_index = *self.keys.get(uniform_name).expect("Uniform does not exist");
        self.set_uniform(uniform_index, uniform_value)
    }
}
