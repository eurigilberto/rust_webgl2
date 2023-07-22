use std::rc::Rc;

use glam::*;
use web_sys::{WebGl2RenderingContext as gl, WebGlRenderbuffer};

use crate::{limits, Graphics, TextureInternalFormat};

pub struct Renderbuffer {
    context: Rc<gl>,
    pub name: Option<String>,
    pub renderbuffer: WebGlRenderbuffer,
    pub size: UVec2, 
    pub format: TextureInternalFormat,
}

impl Renderbuffer {
    pub fn new(
        graphics: &Graphics,
        name: Option<String>,
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
        if samples == 0 || samples == 1 {
            context.renderbuffer_storage(gl::RENDERBUFFER,
                format.into(),
                size.x as i32,
                size.y as i32,)
        }else{
            context.renderbuffer_storage_multisample(
                gl::RENDERBUFFER,
                samples,
                format.into(),
                size.x as i32,
                size.y as i32,
            );
        }
        context.bind_renderbuffer(gl::RENDERBUFFER, None);

        Ok(Self {
            context,
            name,
            renderbuffer,
            size,
            format
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

impl Drop for Renderbuffer{
    fn drop(&mut self) {
        //self.unbind();
        self.context.delete_renderbuffer(Some(&self.renderbuffer));
        /*match &self.name {
            Some(name) => {
                web_sys::console::log_1(&JsValue::from_str(&format!("Dropped renderbuffer -- {}", name)))
            },
            None => {},
        }*/
    }
}