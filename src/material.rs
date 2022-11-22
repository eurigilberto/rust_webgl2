use crate::{
    create_program_from_single_shader_source, shader_program::GlProgram, DrawCapabilities, Graphics, GlUniform, UniformIndex,
};
use std::rc::Rc;
use web_sys::WebGl2RenderingContext as wgl_context;
use webgl2_shader_definition::ShaderSource;

/// A material is going to have some implicit lifetime entanglement with the buffers
/// bound in the vertex array object. Having said so, the buffers could be removed from the VAO
/// and others coudl be bound instead, so it is not a hard coupling.
///
/// Uniforms definitions are still missing form the material
#[allow(dead_code)]
pub struct GlMaterial {
    context: Rc<wgl_context>,
    pub program: GlProgram,
    draw_capabilities: Vec<DrawCapabilities>,
}

impl GlMaterial {
    pub fn with_source(
        graphics: &Graphics,
        draw_capabilities: Vec<DrawCapabilities>,
        source: &ShaderSource,
    ) -> Self {
        let program = create_program_from_single_shader_source(graphics, source)
            .expect("Create Program Error");
        Self{
            context: graphics.gl_context.clone(),
            program,
            draw_capabilities
        }
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
        }
    }

    pub fn set_capabilities(&self, graphics: &Graphics, index: usize) {
        self.draw_capabilities[index].set_capabilities(graphics)
    }

    pub fn set_uniform(&mut self, uniform_index: UniformIndex, uniform_value: GlUniform){
        self.program.uniforms.set_uniform(uniform_index, uniform_value);
    }
}
