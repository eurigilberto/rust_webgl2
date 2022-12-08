use std::{borrow::Borrow, cell::Ref, rc::Rc};

use glam::*;
use web_sys::{WebGl2RenderingContext as gl, WebGlTexture};

mod texture2d;
pub use texture2d::*;

mod constants;
pub use constants::*;

use crate::Graphics;

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

pub enum TextureRef {
    Texture2D(Rc<GlTexture2D>),
}

impl TextureRef {
    pub fn bind(&self) {
        match self {
            TextureRef::Texture2D(texture) => texture.bind(),
        }
    }

    pub fn ref_eq(&self, texture: &TextureRef) -> bool {
        match (self, texture) {
            (TextureRef::Texture2D(tx1), TextureRef::Texture2D(tx2)) => Rc::ptr_eq(tx1, tx2),
        }
    }

    pub fn clone(&self) -> Self {
        match self {
            TextureRef::Texture2D(tx_ref) => TextureRef::Texture2D(Rc::clone(tx_ref)),
        }
    }
}

pub struct TextureUnits {
    pub active_textures: [Option<TextureRef>; 16],
}

impl TextureUnits {
    pub fn new() -> Self {
        Self {
            active_textures: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
        }
    }

    pub fn get_unit_binding_from_texture(&self, texture: &TextureRef) -> Option<usize> {
        for (index, active_texture) in self.active_textures.iter().enumerate() {
            if active_texture.is_none() {
                continue;
            }
            let active_texture = active_texture.as_ref().unwrap();
            if TextureRef::ref_eq(active_texture, &texture) {
                return Some(index);
            }
        }
        None
    }

    pub fn generate_already_bound_units(
        &self,
        textures: &Vec<TextureRef>,
    ) -> BoundTextureUnitsStatus {
        let mut available_units: Vec<u32> = (0..16).map(|unit| unit).collect();
        let mut bound_units = Vec::new();
        let mut missing_bindins = Vec::new();
        //Remove bounds textures from available
        for (texture_index, tx) in textures.iter().enumerate() {
            let unit_index = self.get_unit_binding_from_texture(tx);
            if let Some(u_index) = unit_index {
                available_units.remove(u_index);
                bound_units.push(TextureBindData {
                    texture_index,
                    texture_unit: u_index,
                })
            } else {
                missing_bindins.push(texture_index);
            }
        }

        BoundTextureUnitsStatus {
            available_units,
            bound_units,
            missing_bindins,
        }
    } 
}

pub struct BoundTextureUnitsStatus {
    pub available_units: Vec<u32>,
    pub bound_units: Vec<TextureBindData>,
    pub missing_bindins: Vec<usize>,
}

pub struct TextureBindData {
    pub texture_index: usize,
    pub texture_unit: usize,
}

impl Graphics {
    pub fn bind_missing_textures(
        &self,
        textures: &Vec<TextureRef>,
        mut bound_units: BoundTextureUnitsStatus,
    ) -> BoundTextureUnitsStatus {
        for texture_index in bound_units.missing_bindins.iter() {
            if bound_units.available_units.len() == 0 {
                panic!("No more available texture units");
            }
            let available_unit = bound_units.available_units.remove(0);
            self.bind_texture_to_unit(available_unit, textures[*texture_index].clone());
            bound_units.bound_units.push(TextureBindData {
                texture_index: *texture_index,
                texture_unit: available_unit as usize,
            });
        }
        bound_units
    }

    pub fn bind_textures_to_units(&self, textures: Vec<TextureRef>) -> Vec<TextureBindData> {
        let bound_units = self.texture_units.borrow().generate_already_bound_units(&textures);
        let bound_units = self.bind_missing_textures(&textures, bound_units);
        bound_units.bound_units
    }

    pub fn bind_texture_to_unit(&self, unit: u32, texture: TextureRef) {
        if unit >= 16 {
            panic!("Binding a texture to an out of bounds unit");
        }
        self.gl_context.active_texture(gl::TEXTURE0 + unit);
        texture.bind();
        self.texture_units.borrow_mut().active_textures[unit as usize] = Some(texture);
    }
}
