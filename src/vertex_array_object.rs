use super::*;
use std::rc::{Rc, Weak};
use wasm_bindgen::JsValue;

use crate::{AttributeSize, BindingPoint, GlBuffer, GlIndexBuffer, Graphics, NumberType};

#[derive(Debug, Clone, Copy)]
pub enum AttributeType {
    Single,
    Interleaved { stride: u8, offset: u32 },
    PerInstance { stride: u8, divisor: u8 },
    PerInstanceInterleaved { stride: u8, offset: u32, divisor: u8 },
}

pub struct AttributeDescription {
    pub location: u32,
    pub unit_type: NumberType,
    pub size: AttributeSize,
    pub buffer: usize,
    pub normalize: bool,
    pub kind: AttributeType,
}

impl AttributeDescription {
    pub fn get_stride_and_offset(&self) -> (u8, u32) {
        match self.kind {
            AttributeType::Single => (0, 0),
            AttributeType::PerInstance { stride, .. } => (stride, 0),
            AttributeType::Interleaved { stride, offset }
            | AttributeType::PerInstanceInterleaved { stride, offset, .. } => (stride, offset),
        }
    }
}

/// Wrapper around the WebGlVertexArrayObject
pub struct GlVertexArrayObject {
    context: Rc<gl>,
    pub vao: WebGlVertexArrayObject,
    index_buffer: RefCell<Option<Rc<GlIndexBuffer>>>,
    buffers: RefCell<Vec<Rc<GlBuffer>>>,
    attribute_descriptors: Vec<AttributeDescription>,

}

impl GlVertexArrayObject {
    /// Sets the vertex atribute pointer
    /// it is set to the currently bound ArrayBuffer
    /// with the provided attribute description.
    /// This properly selects the webgl2 attrib pointer function to use
    /// depending on the value type and the normalization property
    fn vertex_attrib_pointer(graphics: &Graphics, attribute: &AttributeDescription, instance_offset: u32) {
        let (stride, mut offset) = attribute.get_stride_and_offset();
        offset = offset + instance_offset * stride as u32; //Set the starting pointer for the attribute to the correct instance offset
        if attribute.unit_type.is_integer_type() && !attribute.normalize {
            graphics.gl_context.vertex_attrib_i_pointer_with_i32(
                attribute.location,
                attribute.size.into(),
                attribute.unit_type.into(),
                stride as i32,
                offset as i32,
            );
        } else {
            graphics.gl_context.vertex_attrib_pointer_with_i32(
                attribute.location,
                attribute.size.into(),
                attribute.unit_type.into(),
                attribute.normalize,
                stride as i32,
                offset as i32,
            );
        }
    }

    pub fn get_index_type(&self) -> Option<IndexType> {
        let index_buffer = self.index_buffer.borrow();
        match index_buffer.as_ref() {
            Some(index_buffer) => Some(index_buffer.index_type),
            None => None,
        }
    }

    pub fn get_index_buffer(&self) -> Option<Weak<GlIndexBuffer>> {
        let index_buffer = self.index_buffer.borrow();
        match index_buffer.as_ref() {
            Some(index_buffer) => return Some(Rc::downgrade(index_buffer)),
            None => None,
        }
    }

    pub fn get_array_buffers(&self) -> Vec<Weak<GlBuffer>> {
        let vertex_buffer = self.buffers.borrow();
        let mut buffers = Vec::new();
        for buffer in vertex_buffer.iter() {
            buffers.push(Rc::downgrade(buffer));
        }
        buffers
    }

    fn enable_vertex_attrib(graphics: &Graphics, attribute: &AttributeDescription) {
        graphics
            .gl_context
            .enable_vertex_attrib_array(attribute.location);
    }

    fn vertex_attrib_divisor(graphics: &Graphics, attribute: &AttributeDescription) {
        match attribute.kind {
            AttributeType::PerInstance { divisor, .. }
            | AttributeType::PerInstanceInterleaved { divisor, .. } => graphics
                .gl_context
                .vertex_attrib_divisor(attribute.location, divisor as u32),
            _ => {}
        }
    }

    /// Panics if 2 attributes have the same location
    fn check_no_overlapping_locations(attributes: &[AttributeDescription]) {
        for (current_index, current_attribute) in attributes.iter().enumerate() {
            for (other_index, other_attribute) in attributes.iter().enumerate() {
                if current_index != other_index
                    && current_attribute.location == other_attribute.location
                {
                    panic!("Attribute {current_index} has the same location as attribute {other_index}");
                }
            }
        }
    }

    /// Panics if 2 attributes have the same location
    pub fn new(
        graphics: &Graphics,
        attribute_descriptors: Vec<AttributeDescription>,
        buffers: &[&Rc<GlBuffer>],
        index_buffer: Option<Rc<GlIndexBuffer>>,
    ) -> Result<Self, JsValue> {
        match graphics.gl_context.create_vertex_array() {
            Some(vertex_array) => {
                let mut bound_points = std::collections::HashSet::new();
                graphics.gl_context.bind_vertex_array(Some(&vertex_array));
                Self::check_no_overlapping_locations(&attribute_descriptors);

                let vao_buffers = buffers.iter().map(|buff| Rc::clone(buff)).collect::<Vec<_>>();
                for attribute in attribute_descriptors.iter() {
                    let buffer = &vao_buffers[attribute.buffer];
                    buffer.bind();

                    Self::enable_vertex_attrib(graphics, attribute);
                    Self::vertex_attrib_pointer(graphics, attribute, 0);
                    Self::vertex_attrib_divisor(graphics, attribute);

                    bound_points.insert(buffer.binding_point);
                }

                let index_buffer = match index_buffer {
                    Some(index_buffer) => {
                        index_buffer.bind();
                        bound_points.insert(index_buffer.binding_point);
                        Some(Rc::clone(&index_buffer))
                    }
                    None => None,
                };

                graphics.gl_context.bind_vertex_array(None);
                for bp in bound_points.drain() {
                    graphics.gl_context.bind_buffer(bp.into(), None);
                }

                Ok(Self {
                    context: graphics.gl_context.clone(),
                    vao: vertex_array,
                    index_buffer: RefCell::new(index_buffer),
                    buffers: RefCell::new(vao_buffers),
                    attribute_descriptors,
                })
            }
            None => Err(JsValue::from("Could not create vertex array object")),
        }
    }

    pub fn swap_buffer(&self, graphics: &Graphics, index: usize, buffer: Rc<GlBuffer>) {
        if index >= self.buffers.borrow().len() {
            panic!("Trying to swap out of bounds buffers")
        }
        self.bind();
        for attribute in self.attribute_descriptors.iter() {
            if attribute.buffer == index {
                buffer.bind();
                Self::vertex_attrib_pointer(graphics, attribute, 0);
            }
        }
        self.unbind();
        graphics.bind_buffer(BindingPoint::ARRAY_BUFFER, None);
        self.buffers.borrow_mut()[index] = buffer;
    }

    pub fn set_instance_offset(&self, graphics: &Graphics, instance_offset: u32) {
        self.bind();
        for attribute in self.attribute_descriptors.iter() {
            match attribute.kind {
                AttributeType::PerInstance { .. } | AttributeType::PerInstanceInterleaved { .. } => {
                    let buffer = &self.buffers.borrow()[attribute.buffer];
                    buffer.bind();
                    Self::vertex_attrib_pointer(graphics, attribute, instance_offset);
                }
                _ => {}
            }
        }
        self.unbind();
        graphics.bind_buffer(BindingPoint::ARRAY_BUFFER, None);
    }

    pub fn swap_index_buffer(&self, graphics: &Graphics, index_buffer: Option<Rc<GlIndexBuffer>>) {
        self.bind();
        match index_buffer {
            Some(index_buffer) => index_buffer.bind(),
            None => {
                graphics.bind_buffer(BindingPoint::INDEX_BUFFER, None);
            }
        }
        self.unbind();
    }

    pub fn bind(&self) {
        self.context.bind_vertex_array(Some(&self.vao));
    }

    pub fn unbind(&self) {
        self.context.bind_vertex_array(None);
    }
}

impl Drop for GlVertexArrayObject {
    fn drop(&mut self) {
        self.context.delete_vertex_array(Some(&self.vao));
    }
}

impl Deref for GlVertexArrayObject {
    type Target = WebGlVertexArrayObject;

    fn deref(&self) -> &Self::Target {
        &self.vao
    }
}
