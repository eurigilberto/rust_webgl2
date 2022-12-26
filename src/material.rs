use crate::{
    create_program_from_single_shader_source, shader_program::GlProgram, DrawCapabilities, Graphics, GlUniform, UniformIndex, GlTexture2D, TextureRef, IntoGlUniform, ProgramCreationError,
};
use std::rc::Rc;
use web_sys::WebGl2RenderingContext as wgl_context;
use webgl2_shader_definition::ShaderSource;

#[allow(dead_code)]
pub struct GlMaterial {
    context: Rc<wgl_context>,
    pub program: GlProgram,
    pub draw_capabilities: Vec<DrawCapabilities>,
    pub sampled_textures: Vec<(Rc<GlTexture2D>, UniformIndex)>
}

impl GlMaterial {
    pub fn with_source(
        graphics: &Graphics,
        draw_capabilities: Vec<DrawCapabilities>,
        source: &ShaderSource,
    ) -> Result<Self, ProgramCreationError> {
        let program = match create_program_from_single_shader_source(graphics, source){
            Ok(program) => program,
            Err(error) => return Err(error),
        };
        Ok(Self{
            context: graphics.gl_context.clone(),
            program,
            draw_capabilities,
            sampled_textures:Vec::new()
        })
    }

    /// There should be a parameter descriptor and parameter upload functions accompaniying this
    pub fn new(
        graphics: &Graphics,
        program: GlProgram,
        draw_capabilities: Vec<DrawCapabilities>,
    ) -> Self {
        Self {
            context: graphics.gl_context.clone(),
            program,
            draw_capabilities,
            sampled_textures:Vec::new()
        }
    }

    pub fn set_capabilities(&self, graphics: &Graphics, index: usize) {
        self.draw_capabilities[index].set_capabilities(graphics)
    }

    pub fn push_texture_samplers(&mut self, graphics: &Graphics){
        let mut texture_refs = Vec::new();
        for (texture, _) in self.sampled_textures.iter(){
            texture_refs.push(TextureRef::Texture2D(Rc::clone(texture)))
        }
        let bind_data = graphics.bind_textures_to_units(texture_refs);
        for data in bind_data{
            let (_, uniform_index) = self.sampled_textures[data.texture_index];
            self.set_uniform(uniform_index, (data.texture_unit as i32).uniform());
        }
    }

    pub fn set_uniform(&mut self, uniform_index: UniformIndex, uniform_value: GlUniform){
        self.program.uniforms.set_uniform(uniform_index, uniform_value);
    }
}
