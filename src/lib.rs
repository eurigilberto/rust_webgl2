use std::ops::Deref;
use std::rc::Rc;

pub use color::*;
use glam::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as wgl_context};
use web_sys::{
    WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlShader, WebGlUniformLocation,
    WebGlVertexArrayObject,
};

mod shader_program;
pub use shader_program::*;
mod limits;
pub use limits::*;
mod constants;
pub use constants::*;
mod webgl_objects;
pub use shader_def::*;
use webgl2_shader_definition as shader_def;
pub use webgl_objects::*;
mod material;
pub mod shader_functions;
pub use shader_functions::*;
pub use material::*;
pub use webgl2_shader_definition;
pub use webgl2_shader_generation;

#[allow(dead_code)]
pub struct GLViewport {
    pub position: IVec2,
    pub size: IVec2,
}

pub fn get_canvas() -> Result<HtmlCanvasElement, ()> {
    let document = web_sys::window().unwrap().document().unwrap();
    match document.get_element_by_id("view3d") {
        Some(canvas_element) => match canvas_element.dyn_into() {
            Ok(canvas) => Ok(canvas),
            Err(_) => Err(()),
        },
        None => Err(()),
    }
} 

pub fn create_webgl_context(
) -> Result<(HtmlCanvasElement, web_sys::WebGl2RenderingContext), JsValue> {
    let canvas = get_canvas().expect("Could not get the HTMLCanvasElement");

    match js_sys::JSON::parse(&format!("{{\"xrCompatible\":{}, \"antialias\": true}}", "true")) {
        Ok(attributes) => {
            match canvas
                .get_context_with_context_options("webgl2", &attributes)?
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
            {
                Ok(context) => Ok((canvas, context)),
                Err(_) => Err(JsValue::from("Web gl context could not be created")),
            }
        }
        Err(_) => {
            panic!("Attribute object could not be created");
        }
    }
}
///////////GRAPHICS
pub struct Graphics {
    gl_context: Rc<wgl_context>,
    canvas: HtmlCanvasElement,
    pub screen_size: glam::UVec2,
    pub viewport_size: Vec2,
}

impl Graphics {
    pub fn new(
        render_context: wgl_context,
        canvas: web_sys::HtmlCanvasElement,
        size: Option<UVec2>,
        start_background_color: RGBA
    ) -> Result<Self, ()> {
        Self::_clear_framebuffer(&render_context, None, Some(&start_background_color), None, None);

        let mut _size = uvec2(0, 0);
        if let Some(size) = size {
            canvas.set_width(size.x);
            canvas.set_height(size.y);
            _size = size;
        }

        Ok(Self {
            canvas,
            gl_context: Rc::new(render_context),
            screen_size: _size,
            viewport_size: Vec2::ZERO,
        })
    }
    #[allow(dead_code)]
    pub fn resize(&mut self, new_size: UVec2) {
        self.canvas.set_width(new_size.x);
        self.canvas.set_height(new_size.y);
        self.screen_size = new_size;
    }
    pub fn clear_main_frameburffer(
        &self,
        color: Option<&RGBA>,
        depth: Option<f32>,
        stencil: Option<u32>,
    ) {
        self.clear_framebuffer(None, color, depth, stencil);
    }

    pub fn clear_framebuffer(
        &self,
        framebuffer: Option<&WebGlFramebuffer>,
        color: Option<&RGBA>,
        depth: Option<f32>,
        stencil: Option<u32>,
    ) {
        Self::_clear_framebuffer(&self.gl_context, framebuffer, color, depth, stencil)
    }

    pub fn clear_current_framebuffer(
        &self,
        color: Option<&RGBA>,
        depth: Option<f32>,
        stencil: Option<u32>,
    ) {
        Self::_clear_current_framebuffer(&self.gl_context, color, depth, stencil)
    }

    pub fn _clear_current_framebuffer(
        context: &wgl_context,
        color: Option<&RGBA>,
        depth: Option<f32>,
        stencil: Option<u32>,
    ) {
        let mut clear_mask = 0;
        if let Some(c) = color {
            context.clear_color(c.r, c.g, c.b, c.a);
            clear_mask |= ClearMaskBits::COLOR_BUFFER_BIT.value();
        }
        if let Some(d) = depth {
            context.clear_depth(d);
            clear_mask |= ClearMaskBits::DEPTH_BUFFER_BIT.value();
        }
        if let Some(s) = stencil {
            context.clear_stencil(s as i32);
            clear_mask |= ClearMaskBits::STENCIL_BUFFER_BIT.value();
        }
        if clear_mask != 0 {
            context.clear(clear_mask);
        }
    }
    pub fn get_gl_context_clone(&self) -> wgl_context {
        self.gl_context.as_ref().clone()
    }
    fn _clear_framebuffer(
        context: &wgl_context,
        framebuffer: Option<&WebGlFramebuffer>,
        color: Option<&RGBA>,
        depth: Option<f32>,
        stencil: Option<u32>,
    ) {
        context.bind_framebuffer(FramebufferBinding::FRAMEBUFFER.into(), framebuffer);
        Self::_clear_current_framebuffer(context, color, depth, stencil)
    }

    pub fn bind_default_vertex_array(&self) {
        self.gl_context.bind_vertex_array(None);
    }
}

impl From<WebGLDataType> for GlUniform {
    fn from(uniform: WebGLDataType) -> Self {
        match uniform {
            shader_def::WebGLDataType::Void => panic!("Cannot transform void"),

            shader_def::WebGLDataType::Float => GlUniform::Float(FloatUniform::Scalar(0.0)),
            shader_def::WebGLDataType::Vec2 => GlUniform::Float(FloatUniform::Vec2(Vec2::ZERO)),
            shader_def::WebGLDataType::Vec3 => GlUniform::Float(FloatUniform::Vec3(Vec3::ZERO)),
            shader_def::WebGLDataType::Vec4 => GlUniform::Float(FloatUniform::Vec4(Vec4::ZERO)),
            shader_def::WebGLDataType::Mat2 => GlUniform::Float(FloatUniform::Mat2(Mat2::IDENTITY)),
            shader_def::WebGLDataType::Mat3 => GlUniform::Float(FloatUniform::Mat3(Mat3::IDENTITY)),
            shader_def::WebGLDataType::Mat4 => GlUniform::Float(FloatUniform::Mat4(Mat4::IDENTITY)),

            shader_def::WebGLDataType::Bool => todo!("I am not sure if boolean are useful"),
            shader_def::WebGLDataType::BVec2 => todo!("I am not sure if boolean are useful"),
            shader_def::WebGLDataType::BVec3 => todo!("I am not sure if boolean are useful"),
            shader_def::WebGLDataType::BVec4 => todo!("I am not sure if boolean are useful"),

            shader_def::WebGLDataType::Int => GlUniform::Int(IntUniform::Scalar(0)),
            shader_def::WebGLDataType::IVec2 => GlUniform::Int(IntUniform::Vec2(IVec2::ZERO)),
            shader_def::WebGLDataType::IVec3 => GlUniform::Int(IntUniform::Vec3(IVec3::ZERO)),
            shader_def::WebGLDataType::IVec4 => GlUniform::Int(IntUniform::Vec4(IVec4::ZERO)),

            shader_def::WebGLDataType::Uint => GlUniform::UInt(UIntUniform::Scalar(0)),
            shader_def::WebGLDataType::UVec2 => GlUniform::UInt(UIntUniform::Vec2(UVec2::ZERO)),
            shader_def::WebGLDataType::UVec3 => GlUniform::UInt(UIntUniform::Vec3(UVec3::ZERO)),
            shader_def::WebGLDataType::UVec4 => GlUniform::UInt(UIntUniform::Vec4(UVec4::ZERO)),
        }
    }
}
//Shader object creation
impl Graphics {
    pub fn create_shader_from_str(
        &self,
        shader: &str,
        shader_type: ShaderType,
    ) -> Result<WebGlShader, JsValue> {
        let gl_shader = self.gl_context.create_shader(shader_type.into()).unwrap();
        self.gl_context.shader_source(&gl_shader, shader);
        self.gl_context.compile_shader(&gl_shader);
        let compilation_status = self
            .gl_context
            .get_shader_parameter(&gl_shader, wgl_context::COMPILE_STATUS);
        if !compilation_status
            .as_bool()
            .expect("Compilation status has to be bool")
        {
            //self.gl_context.draw_elements_with_f64(mode, count, type_, offset)
            match self.gl_context.get_shader_info_log(&gl_shader) {
                Some(shader_log) => Err(JsValue::from(shader_log + "/n " + shader)),
                None => Err(JsValue::from("Shader compilation failed")),
            }
        } else {
            Ok(gl_shader)
        }
    }

    pub fn create_gl_program(
        &self,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, JsValue> {
        match self.gl_context.create_program() {
            Some(shader_program) => {
                self.gl_context
                    .attach_shader(&shader_program, vertex_shader);
                self.gl_context
                    .attach_shader(&shader_program, fragment_shader);
                self.gl_context.link_program(&shader_program);

                let link_param = self
                    .gl_context
                    .get_program_parameter(&shader_program, ProgramParamerter::LINK_STATUS.into());

                if link_param.as_bool().unwrap() {
                    self.gl_context
                        .detach_shader(&shader_program, vertex_shader);
                    self.gl_context
                        .detach_shader(&shader_program, fragment_shader);
                    Ok(shader_program)
                } else {
                    match self.gl_context.get_program_info_log(&shader_program) {
                        Some(info_log) => Err(JsValue::from(info_log)),
                        None => Err(JsValue::from("Program link status false")),
                    }
                }
            }
            None => Err(JsValue::from("Could not create webgl program")),
        }
    }
}

//Redefinition of gl function signatures
impl Graphics {
    pub fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        self.gl_context.color_mask(red, green, blue, alpha);
    }
    pub fn depth_mask(&self, depth: bool) {
        self.gl_context.depth_mask(depth);
    }
    #[allow(dead_code)]
    pub fn create_shader(&self, shader_type: ShaderType) -> Option<WebGlShader> {
        self.gl_context.create_shader(shader_type.into())
    }
    #[allow(dead_code)]
    pub fn bind_buffer(&self, bind_target: BindingPoint, buffer: Option<&WebGlBuffer>) {
        self.gl_context.bind_buffer(bind_target.into(), buffer);
    }

    pub fn bind_framebuffer(
        &self,
        framebuffer_binding: FramebufferBinding,
        framebuffer: Option<&WebGlFramebuffer>,
    ) {
        self.gl_context
            .bind_framebuffer(framebuffer_binding.into(), framebuffer);
    }

    #[allow(dead_code)]
    pub fn enable_capability(&self, capability: WebGlCapability) {
        self.gl_context.enable(capability.into());
    }
    #[allow(dead_code)]
    pub fn disable_capability(&self, capability: WebGlCapability) {
        self.gl_context.disable(capability.into());
    }

    pub fn set_cull_face(&self, mode: CullMode) {
        self.gl_context.cull_face(mode.into());
    }

    pub fn set_depth_func(&self, depth_fn: DepthFunction) {
        self.gl_context.depth_func(depth_fn.into());
    }
    pub fn set_depth_write(&self, value: bool) {
        self.gl_context.depth_mask(value);
    }
    pub fn set_depth_range(&self, near: f32, far: f32) {
        self.gl_context.depth_range(near, far);
    }

    pub fn set_viewport(&mut self, position: IVec2, size: UVec2) {
        self.viewport_size = size.as_vec2();
        self.gl_context
            .viewport(position.x, position.y, size.x as i32, size.y as i32);
    }

    pub fn set_scissor(&self, pos: IVec2, size: IVec2) {
        self.gl_context.scissor(pos.x, pos.y, size.x, size.y)
    }

    pub fn set_blend_equation(&self, equation: BlendEquationState) {
        match equation {
            BlendEquationState::Same(eq) => {
                self.gl_context.blend_equation(eq.into());
            }
            BlendEquationState::Separate { rgb, alpha } => self
                .gl_context
                .blend_equation_separate(rgb.into(), alpha.into()),
        }
    }

    pub fn set_blend_factor(&self, factor: BlendFactorState) {
        match factor {
            BlendFactorState::Same { src, dst } => {
                self.gl_context.blend_func(src.into(), dst.into())
            }
            BlendFactorState::Separate {
                src_rgb,
                dst_rgb,
                src_alpha,
                dst_alpha,
            } => self.gl_context.blend_func_separate(
                src_rgb.into(),
                dst_rgb.into(),
                src_alpha.into(),
                dst_alpha.into(),
            ),
        }
    }

    pub fn set_blend_color(&self, color: RGBA) {
        self.gl_context
            .blend_color(color.r, color.g, color.b, color.a)
    }

    pub fn set_stencil_state(&self, state: StencilData) {
        self.gl_context.stencil_func(
            state.func.into(),
            state.ref_.value as i32,
            state.ref_.mask as u32,
        );
        self.gl_context
            .stencil_op(state.fail.into(), state.zfail.into(), state.zpass.into())
    }

    pub fn set_stencil_state_separate(&self, face: CullMode, state: StencilData) {
        self.gl_context.stencil_func_separate(
            face.into(),
            state.func.into(),
            state.ref_.value as i32,
            state.ref_.mask as u32,
        );
        self.gl_context.stencil_op_separate(
            face.into(),
            state.fail.into(),
            state.zfail.into(),
            state.zpass.into(),
        );
    }
    //pub fn set_blend
}

#[derive(Debug)]
pub enum ProgramCreationError {
    SourceParsing,
    VertexShader(JsValue),
    FragmentShader(JsValue),
    ShaderGeneration { vertex: JsValue, fragment: JsValue },
    Program(JsValue),
}
pub fn create_program_from_single_shader_source(
    graphics: &Graphics,
    source: &ShaderSource,
) -> Result<GlProgram, ProgramCreationError> {
    match generate_shader_str_from_single_source(source) {
        Ok((vs_shader, fs_shader)) => {
            /*web_sys::console::log_1(&JsValue::from(format!(
                "Vertex shader: {:?}
            Fragment shader: {:?}",
                vs_shader, fs_shader
            )));*/
            match (
                GlShader::new(graphics, &vs_shader, ShaderType::VERTEX_SHADER),
                GlShader::new(graphics, &fs_shader, ShaderType::FRAGMENT_SHADER),
            ) {
                (Err(vertex_error), Err(fragment_error)) => {
                    Err(ProgramCreationError::ShaderGeneration {
                        vertex: vertex_error,
                        fragment: fragment_error,
                    })
                }
                (Ok(_), Err(frag_error)) => Err(ProgramCreationError::FragmentShader(frag_error)),
                (Err(vert_error), Ok(_)) => Err(ProgramCreationError::VertexShader(vert_error)),
                (Ok(vert_shader), Ok(frag_shader)) => {
                    match GlProgram::new(graphics, &vert_shader, &frag_shader) {
                        Ok(program) => Ok(program),
                        Err(error) => Err(ProgramCreationError::Program(error)),
                    }
                }
            }
        }
        Err(_) => Err(ProgramCreationError::SourceParsing),
    }
}

#[derive(Clone, Copy)]
pub struct StencilData {
    pub ref_: StencilRef,
    pub func: StencilFunc,
    pub fail: StencilOp,
    pub zfail: StencilOp,
    pub zpass: StencilOp,
}

impl StencilData {
    pub fn set_ref(&mut self, ref_: StencilRef) {
        self.ref_ = ref_;
    }
}

#[derive(Clone, Copy)]
pub struct StencilRef {
    pub value: u8,
    pub mask: u8,
}

#[derive(Clone, Copy)]
pub enum StencilTestState {
    Same(StencilData),
    Separate {
        pub front: StencilData,
        pub back: StencilData,
    },
}

#[derive(Clone, Copy)]
pub enum BlendEquationState {
    Same(BlendEquation),
    Separate {
        pub rgb: BlendEquation,
        pub alpha: BlendEquation,
    },
}

#[derive(Clone, Copy)]
pub enum BlendFactorState {
    Same {
        pub src: BlendFuncFactor,
        pub dst: BlendFuncFactor,
    },
    Separate {
        pub src_rgb: BlendFuncFactor,
        pub dst_rgb: BlendFuncFactor,

        pub src_alpha: BlendFuncFactor,
        pub dst_alpha: BlendFuncFactor,
    },
}

#[derive(Clone, Copy)]
pub struct BlendState {
    pub equation: BlendEquationState,
    pub factor: BlendFactorState,
    pub blend_color: Option<RGBA>,
}

#[derive(Clone, Copy)]
pub struct Scissor {
    pub position: IVec2,
    pub size: IVec2,
}

pub struct DrawCapabilities {
    pub blend_state: Option<BlendState>,
    pub cull_face: Option<CullMode>,
    pub depth_test: Option<DepthFunction>,
    pub stencil_test: Option<StencilTestState>,
    pub scissor_test: Option<Scissor>,
    pub color_draw_mask: (bool, bool, bool, bool),
    pub depth_draw_mask: bool,
}

impl DrawCapabilities {
    pub fn new(
        blend_state: Option<BlendState>,
        cull_face: Option<CullMode>,
        depth_test: Option<DepthFunction>,
        stencil_test: Option<StencilTestState>,
        scissor_test: Option<Scissor>,
        color_draw_mask: Option<(bool, bool, bool, bool)>,
        depth_draw_mask: Option<bool>,
    ) -> Self {
        Self {
            blend_state,
            cull_face,
            depth_test,
            stencil_test,
            scissor_test,
            color_draw_mask: color_draw_mask.unwrap_or((true, true, true, true)),
            depth_draw_mask: depth_draw_mask.unwrap_or(true),
        }
    }

    pub fn default_opaque_no_cull() -> Self {
        DrawCapabilities::new(
            None,
            None,
            Some(DepthFunction::LEQUAL),
            None,
            None,
            None,
            None,
        )
    }

    pub fn default_opaque() -> Self {
        DrawCapabilities::new(
            None,
            Some(CullMode::BACK),
            Some(DepthFunction::LEQUAL),
            None,
            None,
            None,
            None,
        )
    }

    pub fn always_render() -> Self {
        DrawCapabilities::new(None, None, None, None, None, None, None)
    }

    pub fn get_mut_stencil_state(&mut self) -> Option<&mut StencilTestState> {
        match &mut self.stencil_test {
            Some(stencil_test) => Some(stencil_test),
            None => None,
        }
    }

    pub fn set_capabilities(&self, graphics: &Graphics) {
        match self.blend_state {
            Some(state) => {
                graphics.enable_capability(WebGlCapability::BLEND);
                graphics.set_blend_equation(state.equation);
                graphics.set_blend_factor(state.factor);
                let blend_color = state.blend_color.unwrap_or(RGBA::BLACK);
                graphics.set_blend_color(blend_color);
            }
            None => graphics.disable_capability(WebGlCapability::BLEND),
        }

        match self.cull_face {
            Some(mode) => {
                graphics.enable_capability(WebGlCapability::CULL_FACE);
                graphics.set_cull_face(mode);
            }
            None => graphics.disable_capability(WebGlCapability::CULL_FACE),
        }

        match self.depth_test {
            Some(depth_fn) => {
                graphics.enable_capability(WebGlCapability::DEPTH_TEST);
                graphics.set_depth_func(depth_fn);
                graphics.set_depth_write(true);
            }
            None => {
                graphics.disable_capability(WebGlCapability::DEPTH_TEST);
            }
        }

        match self.stencil_test {
            Some(stencil_state) => {
                graphics.enable_capability(WebGlCapability::STENCIL_TEST);
                match stencil_state {
                    StencilTestState::Same(state) => {
                        graphics.set_stencil_state(state);
                    }
                    StencilTestState::Separate { front, back } => {
                        graphics.set_stencil_state_separate(CullMode::FRONT, front);
                        graphics.set_stencil_state_separate(CullMode::BACK, back);
                    }
                }
            }
            None => graphics.disable_capability(WebGlCapability::STENCIL_TEST),
        }

        match self.scissor_test {
            Some(scissor) => {
                graphics.enable_capability(WebGlCapability::SCISSOR_TEST);
                graphics.set_scissor(scissor.position, scissor.size);
            }
            None => graphics.disable_capability(WebGlCapability::SCISSOR_TEST),
        }

        graphics.color_mask(
            self.color_draw_mask.0,
            self.color_draw_mask.1,
            self.color_draw_mask.2,
            self.color_draw_mask.3,
        );

        graphics.depth_mask(
            self.depth_draw_mask,
        );
    }
}
