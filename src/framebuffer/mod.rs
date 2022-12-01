use std::{cell::RefCell, rc::Rc};

use glam::UVec2;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext as gl, WebGlFramebuffer};
mod constants;
use crate::{
    FramebufferMaskBits, GlTexture2D, Graphics, MagFilter, Renderbuffer, TextureBindTarget,
};
pub use constants::*;

#[derive(Debug, Clone, Copy)]
pub enum FramebufferError {
    CreateBuffer,
}

#[derive(Clone, Copy)]
pub struct Viewport {
    pub position: UVec2,
    pub size: UVec2,
}

#[derive(Clone, Copy)]
pub enum FramebufferAttachment {
    Color(u32),
    Depth,
    DepthStencil,
}

pub struct Framebuffer {
    context: Rc<gl>,
    pub framebuffer: WebGlFramebuffer,
    target: RefCell<Option<FramebufferBinding>>,
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
        self.context
            .bind_framebuffer(target.into(), Some(&self.framebuffer));
        self.target.replace(Some(target));
    }

    pub fn bind_none(context: &gl, target: FramebufferBinding) {
        context.bind_framebuffer(target.into(), None);
    }

    pub fn unbind(&self) {
        let target = *self.target.borrow_mut();
        if let Some(target) = target {
            self.context.bind_framebuffer(target.into(), None);
        }
        self.target.replace(None);
    }

    pub fn set_attachment_texture2d(
        &mut self,
        attachment: FramebufferAttachment,
        texture: Option<&GlTexture2D>,
    ) {
        self.bind(FramebufferBinding::DRAW_FRAMEBUFFER);
        let mut tx = None;
        if let Some(texture) = texture {
            texture.bind();
            tx = Some(&texture.texture);
        }

        self.context.framebuffer_texture_2d(
            FramebufferBinding::DRAW_FRAMEBUFFER.into(),
            match attachment {
                FramebufferAttachment::Color(index) => gl::COLOR_ATTACHMENT0 + index,
                FramebufferAttachment::Depth => gl::DEPTH_ATTACHMENT,
                FramebufferAttachment::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
            },
            TextureBindTarget::TEXTURE_2D.into(),
            tx,
            0,
        );

        if let Some(texture) = texture {
            texture.unbind();
        }
        self.unbind();
    }

    pub fn set_attachment_renderbuffer(
        &mut self,
        attachment: FramebufferAttachment,
        renderbuffer: Option<&Renderbuffer>,
    ) {
        self.bind(FramebufferBinding::DRAW_FRAMEBUFFER);
        {
            let renderbuffer = if let Some(renderbuffer) = renderbuffer {
                renderbuffer.bind();
                Some(&renderbuffer.renderbuffer)
            } else {
                None
            };
            self.context.framebuffer_renderbuffer(
                FramebufferBinding::DRAW_FRAMEBUFFER.into(),
                match attachment {
                    FramebufferAttachment::Color(index) => gl::COLOR_ATTACHMENT0 + index,
                    FramebufferAttachment::Depth => gl::DEPTH_ATTACHMENT,
                    FramebufferAttachment::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
                },
                gl::RENDERBUFFER,
                renderbuffer,
            );
        }
        self.unbind();
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

        match dst {
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

        let src_min = src_viewport.position.as_ivec2();
        let src_max = src_min + src_viewport.size.as_ivec2();

        let dst_min = dst_viewport.position.as_ivec2();
        let dst_max = dst_min + dst_viewport.size.as_ivec2();

        graphics.gl_context.blit_framebuffer(
            src_min.x,
            src_min.y,
            src_max.x,
            src_max.y,
            //----
            dst_min.x,
            dst_min.y,
            dst_max.x,
            dst_max.y,
            //----
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
