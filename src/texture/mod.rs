use std::rc::Rc;

use glam::*;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext as gl, WebGlTexture};

mod constants;
pub use constants::*;

use crate::Graphics;

#[derive(Debug, Clone, Copy)]
pub enum GlTextureError {
    CreateObject,
}

pub fn tex_wrap(
    context: &gl,
    target: TextureBindTarget,
    wrap_axis: TextureWrapSelect,
    value: TextureWrap,
) {
    let value: u32 = value.into();
    context.tex_parameteri(target.into(), wrap_axis.into(), value as i32);
}

pub fn set_mag_filter(context: &gl, target: TextureBindTarget, value: MagFilter) {
    let value: u32 = value.into();
    context.tex_parameteri(target.into(), gl::TEXTURE_MAG_FILTER, value as i32);
}

pub fn set_min_filter(context: &gl, target: TextureBindTarget, value: MinFilter) {
    let value: u32 = value.into();
    context.tex_parameteri(target.into(), gl::TEXTURE_MIN_FILTER, value as i32);
}

pub fn set_base_level(context: &gl, target: TextureBindTarget, value: i32) {
    context.tex_parameteri(target.into(), gl::TEXTURE_BASE_LEVEL, value);
}

pub fn set_max_level(context: &gl, target: TextureBindTarget, value: i32) {
    context.tex_parameteri(target.into(), gl::TEXTURE_MAX_LEVEL, value);
}

pub fn set_min_max_lod(context: &gl, target: TextureBindTarget, min_max_value: (f32, f32)) {
    context.tex_parameterf(target.into(), gl::TEXTURE_MIN_LOD, min_max_value.0);
    context.tex_parameterf(target.into(), gl::TEXTURE_MAX_LOD, min_max_value.1);
}

#[derive(Clone, Copy)]
pub struct Texture2DProps {
    pub wrap_x: TextureWrap,
    pub wrap_y: TextureWrap,
    pub mag_filter: MagFilter,
    pub min_filter: MinFilter,
    pub base_level: i32,
    pub max_level: i32,
    pub min_max_lod: (f32, f32),
}

impl Texture2DProps {
    pub fn set_all_props(&self, context: &Graphics) {
        let context = &context.get_gl_context_clone();
        let target = TextureBindTarget::TEXTURE_2D;

        tex_wrap(
            context,
            target.into(),
            TextureWrapSelect::TEXTURE_WRAP_X,
            self.wrap_x,
        );
        tex_wrap(
            context,
            target.into(),
            TextureWrapSelect::TEXTURE_WRAP_Y,
            self.wrap_y,
        );

        set_mag_filter(context, target.into(), self.mag_filter);
        set_min_filter(context, target.into(), self.min_filter);

        set_base_level(context, target.into(), self.base_level);
        set_max_level(context, target.into(), self.max_level);

        set_min_max_lod(context, target.into(), self.min_max_lod);
    }
}

pub struct GlTexture2D {
    context: Rc<gl>,
    pub props: Texture2DProps,
    pub texture: WebGlTexture,
    pub format: TextureInternalFormat,
    pub size: UVec2,
}

impl GlTexture2D {
    pub fn new(
        graphics: &Graphics,
        props: Texture2DProps,
        size: UVec2,
        format: TextureInternalFormat,
    ) -> Result<Self, GlTextureError> {
        let gl = graphics.get_gl_context_clone();
        let texture = match gl.create_texture() {
            Some(texture) => texture,
            None => return Err(GlTextureError::CreateObject),
        };

        //Bind
        gl.bind_texture(TextureBindTarget::TEXTURE_2D.into(), Some(&texture));

        gl.tex_storage_2d(
            TextureBindTarget::TEXTURE_2D.into(),
            0,
            format.into(),
            size.x as i32,
            size.y as i32,
        );
        props.set_all_props(graphics);

        //Unbind
        gl.bind_texture(TextureBindTarget::TEXTURE_2D.into(), None);

        Ok(Self {
            props,
            texture,
            context: graphics.gl_context.clone(),
            format,
            size,
        })
    }

    pub fn bind(&self) {
        self.context
            .bind_texture(TextureBindTarget::TEXTURE_2D.into(), Some(&self.texture));
    }

    pub fn unbind(&self) {
        self.context
            .bind_texture(TextureBindTarget::TEXTURE_2D.into(), None);
    }

    pub fn set_texture_data<T: bytemuck::Pod>(
        &mut self,
        level: u32,
        src_data: &[T],
        src_offset: u32,
    ) -> Result<(), JsValue> {
        let internal_format: u32 = self.format.into();
        let format: TextureFormat = self.format.into();
        let type_: TextureType = self.format.into();
        self.context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
                TextureBindTarget::TEXTURE_2D.into(),
                level as i32,
                internal_format as i32,
                self.size.x as i32,
                self.size.y as i32,
                0,
                format.into(),
                type_.into(),
                bytemuck::cast_slice(src_data),
                src_offset,
            )
    }

    pub fn set_sub_texture_data<T: bytemuck::Pod>(
        &mut self,
        level: u32,
        src_data: &[T],
        src_offset: u32,
        offset: UVec2,
    ) -> Result<(), JsValue> {
        let format: TextureFormat = self.format.into();
        let type_: TextureType = self.format.into();
        self.context
            .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_u8_array_and_src_offset(
                TextureBindTarget::TEXTURE_2D.into(),
                level as i32,
                offset.x as i32,
                offset.y as i32,
                self.size.x as i32,
                self.size.y as i32,
                format.into(),
                type_.into(),
                bytemuck::cast_slice(src_data),
                src_offset,
            )
    }
}

impl Drop for GlTexture2D{
    fn drop(&mut self) {
        self.unbind();
		self.context.delete_texture(Some(&self.texture))
    }
}