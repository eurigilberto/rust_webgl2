use std::rc::Rc;

use web_sys::{WebGl2RenderingContext as gl, WebGlFramebuffer, WebGlTexture};
mod constants;
use crate::{Graphics};
pub use constants::*;

#[derive(Debug, Clone, Copy)]
pub enum FramebufferError {
    CreateBuffer,
}

pub struct Framebuffer {
    context: Rc<gl>,
    pub framebuffer: WebGlFramebuffer,
    pub target: Option<FramebufferBinding>,
}

impl Framebuffer {
    pub fn new(graphics: &Graphics) -> Result<Self, FramebufferError> {
        let framebuffer = match graphics.gl_context.create_framebuffer() {
            Some(framebuffer) => framebuffer,
            None => return Err(FramebufferError::CreateBuffer),
        };

        Ok(Self {
            context: graphics.gl_context.clone(),
            framebuffer,
            target: None,
        })
    }

    pub fn bind(&mut self, target: FramebufferBinding) {
        self.unbind();
        self.context
            .bind_framebuffer(target.into(), Some(&self.framebuffer));
        self.target = Some(target);
    }

    pub fn unbind(&mut self) {
        if let Some(target) = self.target {
            self.context.bind_framebuffer(target.into(), None);
            self.target = None;
        }
    }

    pub fn set_color_attachment(
        &mut self,
        color_attachment: u32,
        texture: Option<&WebGlTexture>,
        mipmap_level: u32,
        layer: u32,
    ) {
        self.context.framebuffer_texture_layer(
            FramebufferBinding::DRAW_FRAMEBUFFER.into(),
            gl::COLOR_ATTACHMENT0 + color_attachment,
            texture,
            mipmap_level as i32,
            layer as i32,
        )
    }

    pub fn set_depth_attachment(&mut self, texture: Option<&WebGlTexture>, layer: u32) {
        self.context.framebuffer_texture_layer(
            FramebufferBinding::DRAW_FRAMEBUFFER.into(),
            gl::DEPTH_ATTACHMENT,
            texture,
            0,
            layer as i32,
        );
    }

    pub fn set_depth_stencil_attachment(&mut self, texture: Option<&WebGlTexture>, layer: u32) {
        self.context.framebuffer_texture_layer(
            FramebufferBinding::DRAW_FRAMEBUFFER.into(),
            gl::DEPTH_STENCIL_ATTACHMENT,
            texture,
            0,
            layer as i32,
        )
    }
}


impl Drop for Framebuffer{
    fn drop(&mut self) {
        self.unbind();
		self.context.delete_framebuffer(Some(&self.framebuffer))
    }
}