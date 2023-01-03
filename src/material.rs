use crate::{
    create_program_from_single_shader_source, shader_program::GlProgram, DeviceLimit,
    DrawCapabilities, GlTexture2D, GlUniform, Graphics, IntoGlUniform, ProgramCreationError,
    TextureRef, UniformIndex, TextureBindTarget,
};
use std::rc::Rc;
use web_sys::WebGl2RenderingContext as wgl_context;
use webgl2_shader_definition::ShaderSource;

#[allow(dead_code)]
pub struct GlMaterial {
    context: Rc<wgl_context>,
    pub program: GlProgram,
    pub draw_capabilities: Vec<DrawCapabilities>,
    max_texture_units: u32,
    sampled_textures: Vec<(Rc<GlTexture2D>, UniformIndex)>,
}

impl GlMaterial {
    pub fn with_source(
        graphics: &Graphics,
        draw_capabilities: Vec<DrawCapabilities>,
        source: &ShaderSource,
    ) -> Result<Self, ProgramCreationError> {
        let program = match create_program_from_single_shader_source(graphics, source) {
            Ok(program) => program,
            Err(error) => return Err(error),
        };
        let context = graphics.gl_context.clone();
        let max_texture_units =
            match context.get_parameter(DeviceLimit::MAX_VERTEX_TEXTURE_IMAGE_UNITS.value()) {
                Ok(max_count) => max_count.as_f64().unwrap() as u32,
                Err(_) => panic!("Cannot get the max textire unit"),
            };
        Ok(Self {
            context: context,
            program,
            draw_capabilities,
            sampled_textures: Vec::new(),
            max_texture_units,
        })
    }
    pub fn insert_uniform<T: IntoGlUniform>(&mut self, uniform: T, name: &str)->UniformIndex{
        let uniform = uniform.uniform();
        self.program.insert_uniform(name, uniform).unwrap()
    }

    /// There should be a parameter descriptor and parameter upload functions accompaniying this
    pub fn new(
        graphics: &Graphics,
        program: GlProgram,
        draw_capabilities: Vec<DrawCapabilities>,
    ) -> Self {
        let context = graphics.gl_context.clone();
        let max_texture_units =
            match context.get_parameter(DeviceLimit::MAX_VERTEX_TEXTURE_IMAGE_UNITS.value()) {
                Ok(max_count) => max_count.as_f64().unwrap() as u32,
                Err(_) => panic!("Cannot get the max textire unit"),
            };
        Self {
            context: context,
            program,
            draw_capabilities,
            sampled_textures: Vec::new(),
            max_texture_units,
        }
    }

    pub fn set_capabilities(&self, graphics: &Graphics, index: usize) {
        self.draw_capabilities[index].set_capabilities(graphics)
    }

    pub fn push_texture_samplers(&mut self, graphics: &Graphics) {
        let mut texture_refs = Vec::new();
        for (texture, _) in self.sampled_textures.iter() {
            texture_refs.push(TextureRef::Texture2D(Rc::clone(texture)))
        }
        let bind_data = graphics.bind_textures_to_units(texture_refs);
        for data in bind_data {
            let (_, uniform_index) = self.sampled_textures[data.texture_index];
            self.set_uniform(uniform_index, (data.texture_unit as i32).uniform());
        }
        self.context.active_texture(wgl_context::TEXTURE16);
        self.context.bind_texture(TextureBindTarget::TEXTURE_2D.into(), None);
        self.context.bind_texture(TextureBindTarget::TEXTURE_3D.into(), None);
        self.context.bind_texture(TextureBindTarget::TEXTURE_2D_ARRAY.into(), None);
        self.context.bind_texture(TextureBindTarget::TEXTURE_CUBE_MAP.into(), None);
    }

    /// Gets the uniform location with the passed uniform name
    /// adds the result to the GlProgram struct and adds the texture
    /// reference to the material sampled textures array for later use
    pub fn set_texture_sampler_uniform(
        &mut self,
        uniform_name: &str,
        texture_ref: Rc<GlTexture2D>,
    ) -> Result<(), String> {
        if self.sampled_textures.len() as u32 >= self.max_texture_units {
            return Err(format!("Trying to push more texture units than what is supported by this hardware. Current supported count {}", self.max_texture_units));
        }
        let texture_uniform_index = self
            .program
            .insert_uniform(uniform_name, (0 as i32).uniform())?;
        self.sampled_textures
            .push((texture_ref, texture_uniform_index));
        Ok(())
    }

    /// Swaps a texture reference for another texture using the name of the uniform
    /// It expects that the all the textures in the sampled_textures vector have a corresponding entry
    /// in the GlProgram uniform list, it is going to panic if this is not the case
    pub fn swap_texture_sampler_uniform(
        &mut self,
        uniform_name: &str,
        texture_ref: Rc<GlTexture2D>,
    ) -> Result<(), ()> {
        match self.program.uniforms.get_uniform_index(uniform_name) {
            Some(uniform_index) => {
                //Get the index of the sampled texture with the same uniform index
                let texture_index = self
                    .sampled_textures
                    .iter()
                    .position(|(_, uni_index)| *uni_index == uniform_index)
                    .unwrap();
                self.sampled_textures[texture_index] = (texture_ref, uniform_index);
                return Ok(());
            }
            None => todo!(),
        }
    }

    pub fn set_uniform(&mut self, uniform_index: UniformIndex, uniform_value: GlUniform) {
        self.program
            .uniforms
            .set_uniform(uniform_index, uniform_value);
    }
    pub fn set_uniform_value<T: IntoGlUniform>(&mut self, uniform_index: UniformIndex, uniform: T){
        self.set_uniform(uniform_index, uniform.uniform());
    }
}
