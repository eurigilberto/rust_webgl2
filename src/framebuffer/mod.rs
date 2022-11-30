use std::{rc::Rc, cell::RefCell};

use glam::UVec2;
use web_sys::{WebGl2RenderingContext as gl, WebGlFramebuffer};
mod constants;
use crate::{FramebufferMaskBits, GlTexture2D, Graphics, MagFilter, TextureBindTarget};
pub use constants::*;

#[derive(Debug, Clone, Copy)]
pub enum FramebufferError {
    CreateBuffer,
}

pub struct Framebuffer {
    context: Rc<gl>,
    pub framebuffer: WebGlFramebuffer,
    pub target: RefCell<Option<FramebufferBinding>>,
}

pub struct Viewport {
    position: UVec2,
    size: UVec2,
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
            target: RefCell::new(None),
        })
    }

    pub fn bind(&self, target: FramebufferBinding) {
        self.unbind();
        self.context.bind_framebuffer(
            target.into(),
            Some(&self.framebuffer),
        );
        self.target.replace(Some(target));
    }

    pub fn bind_none(context: &gl, target: FramebufferBinding){
        context.bind_framebuffer(target.into(), None);
    }

    pub fn unbind(&self) {
        if let Some(target) = *self.target.borrow() {
            self.context.bind_framebuffer(target.into(), None);
            self.target.replace(None);
        }
    }

    fn set_attachment(&mut self, attachment_index: u32, texture: Option<&GlTexture2D>) {
        self.bind(FramebufferBinding::DRAW_FRAMEBUFFER);
        let mut tx = None;
        if let Some(texture) = texture {
            texture.bind();
            tx = Some(&texture.texture);
        }

        self.context.framebuffer_texture_2d(
            FramebufferBinding::DRAW_FRAMEBUFFER.into(),
            attachment_index,
            TextureBindTarget::TEXTURE_2D.into(),
            tx,
            0,
        );

        if let Some(texture) = texture {
            texture.unbind();
        }
        self.unbind();
    }

    pub fn set_color_attachment(&mut self, color_attachment: u32, texture: Option<&GlTexture2D>) {
        self.set_attachment(gl::COLOR_ATTACHMENT0 + color_attachment, texture);
    }

    pub fn set_depth_attachment(&mut self, texture: Option<&GlTexture2D>) {
        self.set_attachment(gl::DEPTH_ATTACHMENT, texture);
    }

    pub fn set_depth_stencil_attachment(&mut self, texture: Option<&GlTexture2D>) {
        self.set_attachment(gl::DEPTH_STENCIL_ATTACHMENT, texture);
    }

    pub fn blit_framebuffer(
        graphics: &Graphics,
        src: Option<&Framebuffer>,
        src_viewport: Viewport,
        dst: Option<&mut Framebuffer>,
        dst_viewport: Viewport,
        copy_color: bool,
        copy_depth: bool,
        copy_stencil: bool,
        filter: MagFilter,
    ) {
        match src {
            Some(src) => src.bind(FramebufferBinding::READ_FRAMEBUFFER),
            None => Self::bind_none(&graphics.gl_context, FramebufferBinding::READ_FRAMEBUFFER),
        }

        match dst{
            Some(dst) => dst.bind(FramebufferBinding::DRAW_FRAMEBUFFER),
            None => Self::bind_none(&graphics.gl_context, FramebufferBinding::DRAW_FRAMEBUFFER),
        }

        let mut mask = 0;
        if copy_color {
            mask = mask | FramebufferMaskBits::COLOR_BUFFER_BIT.value()
        };
        if copy_depth {
            mask = mask | FramebufferMaskBits::DEPTH_BUFFER_BIT.value()
        };
        if copy_stencil {
            mask = mask | FramebufferMaskBits::STENCIL_BUFFER_BIT.value()
        };

        graphics.gl_context.blit_framebuffer(
            src_viewport.position.x as i32,
            src_viewport.position.y as i32,
            src_viewport.size.x as i32,
            src_viewport.size.y as i32,
            dst_viewport.position.x as i32,
            dst_viewport.position.y as i32,
            dst_viewport.size.x as i32,
            dst_viewport.size.y as i32,
            mask,
            filter.into(),
        )
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        self.unbind();
        self.context.delete_framebuffer(Some(&self.framebuffer))
    }
}
