use std::rc::Rc;

use glam::*;
use web_sys::{WebGl2RenderingContext as gl, WebGlRenderbuffer};

use crate::{limits, Graphics, TextureInternalFormat};

pub struct Renderbuffer {
    context: Rc<gl>,
    pub renderbuffer: WebGlRenderbuffer,
}

impl Renderbuffer {
    pub fn new(
        graphics: &Graphics,
        desired_sample_count: u32,
        size: UVec2,
        format: TextureInternalFormat,
    ) -> Result<Self, ()> {
        let context = graphics.get_gl_context_clone();
        let renderbuffer = match context.create_renderbuffer() {
            Some(renderbuffer) => renderbuffer,
            None => return Err(()),
        };
        let max_samples = context
            .get_parameter(limits::DeviceLimit::MAX_SAMPLES.into())
            .expect("Cannot get max sampels")
            .as_f64()
            .unwrap() as i32;
        let samples = if (desired_sample_count as i32) < max_samples {
            desired_sample_count as i32
        } else {
            max_samples
        };

        context.bind_renderbuffer(gl::RENDERBUFFER, Some(&renderbuffer));
        context.renderbuffer_storage_multisample(
            gl::RENDERBUFFER,
            samples,
            format.into(),
            size.x as i32,
            size.y as i32,
        );
        context.bind_renderbuffer(gl::RENDERBUFFER, None);

        Ok(Self {
            context,
            renderbuffer,
        })
    }

    pub fn bind(&self) {
        self.context
            .bind_renderbuffer(gl::RENDERBUFFER, Some(&self.renderbuffer));
    }

    pub fn unbind(&self) {
        self.context.bind_framebuffer(gl::RENDERBUFFER, None);
    }
}
