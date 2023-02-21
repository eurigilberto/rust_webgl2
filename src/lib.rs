use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub use color::*;
use glam::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as gl};
use web_sys::{
    WebGlBuffer, WebGlFramebuffer, WebGlShader, WebGlUniformLocation, WebGlVertexArrayObject,
};

mod renderbuffer;
pub use renderbuffer::*;
mod shader_program;
pub use shader_program::*;
mod limits;
pub use limits::*;
mod constants;
pub use constants::*;
mod webgl_objects;
pub use webgl2_shader_definition::*;
pub use webgl2_shader_generation::*;
pub use webgl_objects::*;
mod material;
pub mod shader_functions;
pub use material::*;
pub use shader_functions::*;
mod draw_capabilities;
pub use draw_capabilities::*;
mod texture;
pub use texture::*;
mod vertex_array_object;
pub use vertex_array_object::*;
mod framebuffer;
pub use framebuffer::*;
mod util;
pub use util::*;

pub fn get_canvas(canvas_id: &str) -> Result<HtmlCanvasElement, ()> {
    let document = web_sys::window().unwrap().document().unwrap();
    match document.get_element_by_id(canvas_id) {
        Some(canvas_element) => match canvas_element.dyn_into() {
            Ok(canvas) => Ok(canvas),
            Err(_) => Err(()),
        },
        None => Err(()),
    }
}

pub fn create_webgl2_context(
    xr_compatible: bool,
    antialias: bool,
    depth: bool,
    stencil: bool,
    canvas_id: &str,
) -> Result<(HtmlCanvasElement, web_sys::WebGl2RenderingContext), JsValue> {
    let canvas =
        get_canvas(canvas_id).expect(&format!("Could not get the HTMLCanvasElement {canvas_id}"));

    match js_sys::JSON::parse(&format!(
        "{{\"xrCompatible\":{xr_compatible}, \"antialias\": {antialias}, \"depth\": {depth}, \"stencil\": {stencil}}}",
    )) {
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
    gl_context: Rc<gl>,
    canvas: HtmlCanvasElement,
    texture_units: RefCell<TextureUnits>,
}

impl Graphics {
    pub fn new(render_context: gl, canvas: web_sys::HtmlCanvasElement) -> Result<Self, ()> {
        Ok(Self {
            canvas,
            gl_context: Rc::new(render_context),
            texture_units: RefCell::new(TextureUnits::new()),
        })
    }

    pub fn get_canvas_size(&self) -> UVec2 {
        UVec2::new(self.canvas.width(), self.canvas.height())
    }

    #[allow(dead_code)]
    pub fn resize(&mut self, new_size: UVec2) {
        self.canvas.set_width(new_size.x);
        self.canvas.set_height(new_size.y);
    }
    pub fn clear_main_frameburffer(
        &self,
        color: Option<RGBA>,
        depth: Option<f32>,
        stencil: Option<u32>,
    ) {
        self.clear_framebuffer(None, color, depth, stencil);
    }

    pub fn clear_framebuffer(
        &self,
        framebuffer: Option<&WebGlFramebuffer>,
        color: Option<RGBA>,
        depth: Option<f32>,
        stencil: Option<u32>,
    ) {
        Self::_clear_framebuffer(&self.gl_context, framebuffer, color, depth, stencil)
    }

    pub fn clear_current_framebuffer(
        &self,
        color: Option<RGBA>,
        depth: Option<f32>,
        stencil: Option<u32>,
    ) {
        Self::_clear_current_framebuffer(&self.gl_context, color, depth, stencil)
    }

    pub fn _clear_current_framebuffer(
        context: &gl,
        color: Option<RGBA>,
        depth: Option<f32>,
        stencil: Option<u32>,
    ) {
        let mut clear_mask = 0;
        if let Some(c) = color {
            context.clear_color(c.r, c.g, c.b, c.a);
            clear_mask |= FramebufferMaskBits::COLOR_BUFFER_BIT.value();
        }
        if let Some(d) = depth {
            context.clear_depth(d);
            clear_mask |= FramebufferMaskBits::DEPTH_BUFFER_BIT.value();
        }
        if let Some(s) = stencil {
            context.clear_stencil(s as i32);
            clear_mask |= FramebufferMaskBits::STENCIL_BUFFER_BIT.value();
        }
        if clear_mask != 0 {
            context.clear(clear_mask);
        }
    }
    pub fn get_gl_context_clone(&self) -> Rc<gl> {
        Rc::clone(&self.gl_context)
    }
    fn _clear_framebuffer(
        context: &gl,
        framebuffer: Option<&WebGlFramebuffer>,
        color: Option<RGBA>,
        depth: Option<f32>,
        stencil: Option<u32>,
    ) {
        context.bind_framebuffer(FramebufferBinding::DRAW_FRAMEBUFFER.into(), framebuffer);
        Self::_clear_current_framebuffer(context, color, depth, stencil)
    }

    pub fn bind_default_vertex_array(&self) {
        self.gl_context.bind_vertex_array(None);
    }

    pub fn finish(&self){
        self.gl_context.flush();
        self.gl_context.finish();
    }
}

impl From<WebGLDataType> for GlUniform {
    fn from(uniform: WebGLDataType) -> Self {
        use WebGLDataType as Type;
        match uniform {
            Type::Void => panic!("Cannot transform void"),

            Type::Float => GlUniform::Float(FloatUniform::Scalar(0.0)),
            Type::Vec2 => GlUniform::Float(FloatUniform::Vec2(Vec2::ZERO)),
            Type::Vec3 => GlUniform::Float(FloatUniform::Vec3(Vec3::ZERO)),
            Type::Vec4 => GlUniform::Float(FloatUniform::Vec4(Vec4::ZERO)),
            Type::Mat2 => GlUniform::Float(FloatUniform::Mat2(Mat2::IDENTITY)),
            Type::Mat3 => GlUniform::Float(FloatUniform::Mat3(Mat3::IDENTITY)),
            Type::Mat4 => GlUniform::Float(FloatUniform::Mat4(Mat4::IDENTITY)),
            Type::Bool => todo!("I am not sure if boolean are useful"),
            Type::BVec2 => todo!("I am not sure if boolean are useful"),
            Type::BVec3 => todo!("I am not sure if boolean are useful"),
            Type::BVec4 => todo!("I am not sure if boolean are useful"),
            Type::Int => GlUniform::Int(IntUniform::Scalar(0)),
            Type::IVec2 => GlUniform::Int(IntUniform::Vec2(IVec2::ZERO)),
            Type::IVec3 => GlUniform::Int(IntUniform::Vec3(IVec3::ZERO)),
            Type::IVec4 => GlUniform::Int(IntUniform::Vec4(IVec4::ZERO)),
            Type::Uint => GlUniform::UInt(UIntUniform::Scalar(0)),
            Type::UVec2 => GlUniform::UInt(UIntUniform::Vec2(UVec2::ZERO)),
            Type::UVec3 => GlUniform::UInt(UIntUniform::Vec3(UVec3::ZERO)),
            Type::UVec4 => GlUniform::UInt(UIntUniform::Vec4(UVec4::ZERO)),
            Type::Sampler2D
            | Type::Sampler3D
            | Type::SamplerCube
            | Type::SamplerCubeShadow
            | Type::Sampler2DShadow
            | Type::Sampler2DArray
            | Type::Sampler2DArrayShadow
            | Type::ISampler2D
            | Type::ISampler3D
            | Type::ISamplerCube
            | Type::ISampler2DArray
            | Type::USampler2D
            | Type::USampler3D
            | Type::USamplerCube
            | Type::USampler2DArray => GlUniform::Int(IntUniform::Scalar(0)),
        }
    }
}
//Shader object creation

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

    pub fn set_viewport(&self, position: IVec2, size: UVec2) {
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
}
