use std::cell::RefCell;

use super::*;
use crate::shader_program::UniformSetter;

///////////GL-BUFFER
pub struct GlBuffer {
    context: Rc<wgl_context>,
    current_binding: RefCell<Option<BindingPoint>>,
    pub buffer: WebGlBuffer,
    pub binding_point: BindingPoint,
    pub usage: BufferUsage,
}

impl Deref for GlBuffer {
    type Target = WebGlBuffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl GlBuffer {
    pub fn with_size(
        graphics: &Graphics,
        binding_point: BindingPoint,
        size: u32,
        usage: BufferUsage,
    ) -> Result<Self, ()> {
        graphics.gl_context.bind_vertex_array(None);
        match graphics.gl_context.create_buffer() {
            Some(buffer) => {
                graphics
                    .gl_context
                    .bind_buffer(binding_point.into(), Some(&buffer));
                graphics.gl_context.buffer_data_with_i32(
                    binding_point.into(),
                    size as i32,
                    usage.into(),
                );
                graphics.gl_context.bind_buffer(binding_point.into(), None);
                Ok(Self {
                    context: graphics.gl_context.clone(),
                    current_binding: RefCell::new(None),
                    buffer,
                    binding_point,
                    usage,
                })
            }
            None => Err(()),
        }
    }
    pub fn with_data<T: bytemuck::Pod>(
        graphics: &Graphics,
        binding_point: BindingPoint,
        src_data: &[T],
        usage: BufferUsage,
    ) -> Result<Self, ()> {
        graphics.gl_context.bind_vertex_array(None);
        match graphics.gl_context.create_buffer() {
            Some(buffer) => {
                graphics
                    .gl_context
                    .bind_buffer(binding_point.into(), Some(&buffer));

                graphics.gl_context.buffer_data_with_u8_array(
                    binding_point.into(),
                    bytemuck::cast_slice(src_data),
                    usage.into(),
                );

                graphics.gl_context.bind_buffer(binding_point.into(), None);
                Ok(Self {
                    context: graphics.gl_context.clone(),
                    current_binding: RefCell::new(None),
                    buffer,
                    binding_point,
                    usage,
                })
            }
            None => Err(()),
        }
    }

    pub fn array_buffer_with_data<T: bytemuck::Pod>(
        graphics: &Graphics,
        src_data: &[T],
        usage: BufferUsage,
    ) -> Result<Self, ()> {
        Self::with_data(graphics, BindingPoint::ARRAY_BUFFER, src_data, usage)
    }

    pub fn with_data_static_array_buffer<T: bytemuck::Pod>(
        graphics: &Graphics,
        src_data: &[T],
    ) -> Result<Self, ()> {
        Self::array_buffer_with_data(graphics, src_data, BufferUsage::STATIC_DRAW)
    }

    pub fn bind_to(&self, bind_point: BindingPoint) {
        self.current_binding.replace(Some(bind_point));
        self.context.bind_buffer(bind_point.into(), Some(&self.buffer));
    }

    pub fn bind(&self) {
        self.bind_to(self.binding_point);
    }
    pub fn unbind(&self) {
        let current_binding = self.current_binding.take();
        if let Some(bind_point) = current_binding {
            self.context.bind_buffer((bind_point).into(), None);
        }
    }
    pub fn buffer_data<T: bytemuck::Pod>(&self, src_data: &[T]) {
        self.bind();
        self.context.buffer_data_with_u8_array(
            self.binding_point.into(),
            bytemuck::cast_slice(src_data),
            self.usage.into(),
        );
        self.unbind();
    }
    pub fn partial_buffer_data<T: bytemuck::Pod>(&self, src_data: &[T], copy_length: u32) {
        self.partial_buffer_data_offset(src_data, copy_length, 0);
    }
    pub fn partial_buffer_data_offset<T: bytemuck::Pod>(
        &self,
        src_data: &[T],
        copy_length: u32,
        src_offset: u32,
    ) {
        let src_data_slice: &[u8] = bytemuck::cast_slice(src_data);
        if src_data_slice.len() < copy_length as usize {
            panic!("Out of bounds copy")
        }
        self.bind();
        self.context
            .buffer_data_with_u8_array_and_src_offset_and_length(
                self.binding_point.into(),
                bytemuck::cast_slice(src_data),
                self.usage.into(),
                src_offset,
                copy_length,
            );
        self.unbind();
    }
}

impl Drop for GlBuffer {
    fn drop(&mut self) {
        let context = self.context.clone();
        context.delete_buffer(Some(&self.buffer));
    }
}
///////////GL-INDEX-BUFFER
pub struct GlIndexBuffer {
    pub index_type: IndexType,
    pub buffer: GlBuffer,
}
impl Deref for GlIndexBuffer {
    type Target = GlBuffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
impl GlIndexBuffer {
    pub fn with_size(
        graphics: &Graphics,
        index_unit_type: IndexType,
        capacity: u32,
        usage: BufferUsage,
    ) -> Result<Self, ()> {
        match GlBuffer::with_size(graphics, BindingPoint::INDEX_BUFFER, capacity, usage) {
            Ok(buffer) => Ok(Self {
                index_type: index_unit_type,
                buffer,
            }),
            Err(_) => Err(()),
        }
    }

    pub fn with_data<T: bytemuck::Pod>(
        graphics: &Graphics,
        index_unit_type: IndexType,
        src_data: &[T],
        usage: BufferUsage,
    ) -> Result<Self, ()> {
        match GlBuffer::with_data(graphics, BindingPoint::INDEX_BUFFER, src_data, usage) {
            Ok(buffer) => Ok(Self {
                index_type: index_unit_type,
                buffer,
            }),
            Err(_) => Err(()),
        }
    }
}
///////////GL-UNIFORM-BUFFER
pub struct GlUniformBuffer {
    pub gl_buffer: GlBuffer,
    pub binding_point_location: u32,
}

impl Deref for GlUniformBuffer {
    type Target = GlBuffer;

    fn deref(&self) -> &Self::Target {
        &self.gl_buffer
    }
}

impl GlUniformBuffer {
    /// The data layout for 'src_data' has the std140 requirements
    /// I am still unsure on how to enforce them form this API
    pub fn with_data<T: bytemuck::Pod>(
        graphics: &Graphics,
        src_data: &[T],
        usage: BufferUsage,
        binding_point_location: u32,
    ) -> Result<Self, ()> {
        match GlBuffer::with_data(graphics, BindingPoint::UNIFORM_BUFFER, src_data, usage) {
            Ok(gl_buffer) => {
                graphics.gl_context.bind_buffer_base(
                    BindingPoint::UNIFORM_BUFFER.into(),
                    binding_point_location,
                    Some(&gl_buffer),
                );
                Ok(Self {
                    gl_buffer,
                    binding_point_location,
                })
            }
            Err(_) => Err(()),
        }
    }
    pub fn with_capacity(
        graphics: &Graphics,
        capacity: u16,
        usage: BufferUsage,
        binding_point_location: u32,
    ) -> Result<Self, ()> {
        match GlBuffer::with_size(
            graphics,
            BindingPoint::UNIFORM_BUFFER,
            capacity as u32,
            usage,
        ) {
            Ok(gl_buffer) => {
                graphics.gl_context.bind_buffer_base(
                    BindingPoint::UNIFORM_BUFFER.into(),
                    binding_point_location,
                    Some(&gl_buffer),
                );
                Ok(Self {
                    gl_buffer,
                    binding_point_location,
                })
            }
            Err(_) => Err(()),
        }
    }
}
///////////GL-SHADER
pub struct GlShader {
    context: Rc<wgl_context>,
    pub shader: WebGlShader,
    pub shader_type: ShaderType,
}

impl Deref for GlShader {
    type Target = WebGlShader;

    fn deref(&self) -> &Self::Target {
        &self.shader
    }
}

impl Drop for GlShader {
    fn drop(&mut self) {
        self.context.delete_shader(Some(&self.shader))
    }
}

impl GlShader {
    pub fn new(
        graphics: &Graphics,
        shader: &str,
        shader_type: ShaderType,
    ) -> Result<Self, JsValue> {
        match graphics.create_shader_from_str(shader, shader_type) {
            Ok(shader) => Ok(Self {
                context: graphics.gl_context.clone(),
                shader,
                shader_type,
            }),
            Err(err) => Err(err),
        }
    }
}

#[derive(Clone, Copy)]
pub enum FloatUniform {
    Scalar(f32),
    Vec2(glam::Vec2),
    Vec3(glam::Vec3),
    Vec4(glam::Vec4),

    Mat2(glam::Mat2),
    Mat3(glam::Mat3),
    Mat4(glam::Mat4),
}

impl FloatUniform {
    pub fn set_uniform(&self, uniform_setter: &UniformSetter, location: &WebGlUniformLocation) {
        match self {
            FloatUniform::Scalar(value) => uniform_setter.set_uniform_f32(location, *value),
            FloatUniform::Vec2(value) => uniform_setter.set_uniform_vec2(location, value),
            FloatUniform::Vec3(value) => uniform_setter.set_uniform_vec3(location, value),
            FloatUniform::Vec4(value) => uniform_setter.set_uniform_vec4(location, value),
            FloatUniform::Mat2(value) => uniform_setter.set_uniform_mat2(location, value),
            FloatUniform::Mat3(value) => uniform_setter.set_uniform_mat3(location, value),
            FloatUniform::Mat4(value) => uniform_setter.set_uniform_mat4(location, value),
        }
    }

    pub fn equal_variant(lhs: &Self, rhs: &Self) -> bool {
        match (lhs, rhs) {
            (FloatUniform::Scalar(_), FloatUniform::Scalar(_)) => true,
            (FloatUniform::Vec2(_), FloatUniform::Vec2(_)) => true,
            (FloatUniform::Vec3(_), FloatUniform::Vec3(_)) => true,
            (FloatUniform::Vec4(_), FloatUniform::Vec4(_)) => true,
            (FloatUniform::Mat2(_), FloatUniform::Mat2(_)) => true,
            (FloatUniform::Mat3(_), FloatUniform::Mat3(_)) => true,
            (FloatUniform::Mat4(_), FloatUniform::Mat4(_)) => true,
            _ => false,
        }
    }
}

impl From<FloatUniform> for GlUniform {
    fn from(uni: FloatUniform) -> Self {
        GlUniform::Float(uni)
    }
}

#[derive(Clone, Copy)]
pub enum IntUniform {
    Scalar(i32),
    Vec2(glam::IVec2),
    Vec3(glam::IVec3),
    Vec4(glam::IVec4),
}

impl IntUniform {
    pub fn set_uniform(&self, uniform_setter: &UniformSetter, location: &WebGlUniformLocation) {
        match self {
            IntUniform::Scalar(value) => uniform_setter.set_uniform_i32(location, *value),
            IntUniform::Vec2(value) => uniform_setter.set_uniform_ivec2(location, value),
            IntUniform::Vec3(value) => uniform_setter.set_uniform_ivec3(location, value),
            IntUniform::Vec4(value) => uniform_setter.set_uniform_ivec4(location, value),
        }
    }
    pub fn equal_variant(lhs: &Self, rhs: &Self) -> bool {
        match (lhs, rhs) {
            (IntUniform::Scalar(_), IntUniform::Scalar(_)) => true,
            (IntUniform::Vec2(_), IntUniform::Vec2(_)) => true,
            (IntUniform::Vec3(_), IntUniform::Vec3(_)) => true,
            (IntUniform::Vec4(_), IntUniform::Vec4(_)) => true,
            _ => false,
        }
    }
}

impl From<IntUniform> for GlUniform {
    fn from(uni: IntUniform) -> Self {
        GlUniform::Int(uni)
    }
}

#[derive(Clone, Copy)]
pub enum UIntUniform {
    Scalar(u32),
    Vec2(glam::UVec2),
    Vec3(glam::UVec3),
    Vec4(glam::UVec4),
}

impl UIntUniform {
    pub fn set_uniform(&self, uniform_setter: &UniformSetter, location: &WebGlUniformLocation) {
        match self {
            UIntUniform::Scalar(value) => uniform_setter.set_uniform_u32(location, *value),
            UIntUniform::Vec2(value) => uniform_setter.set_uniform_uvec2(location, value),
            UIntUniform::Vec3(value) => uniform_setter.set_uniform_uvec3(location, value),
            UIntUniform::Vec4(value) => uniform_setter.set_uniform_uvec4(location, value),
        }
    }
    pub fn equal_variant(lhs: &Self, rhs: &Self) -> bool {
        match (lhs, rhs) {
            (UIntUniform::Scalar(_), UIntUniform::Scalar(_)) => true,
            (UIntUniform::Vec2(_), UIntUniform::Vec2(_)) => true,
            (UIntUniform::Vec3(_), UIntUniform::Vec3(_)) => true,
            (UIntUniform::Vec4(_), UIntUniform::Vec4(_)) => true,
            _ => false,
        }
    }
}

impl From<UIntUniform> for GlUniform {
    fn from(uni: UIntUniform) -> Self {
        GlUniform::UInt(uni)
    }
}

#[derive(Clone, Copy)]
pub enum GlUniform {
    Float(FloatUniform),
    Int(IntUniform),
    UInt(UIntUniform),
}

impl GlUniform {
    pub fn set_uniform(&self, uniform_setter: &UniformSetter, location: &WebGlUniformLocation) {
        match self {
            GlUniform::Float(value) => value.set_uniform(uniform_setter, location),
            GlUniform::Int(value) => value.set_uniform(uniform_setter, location),
            GlUniform::UInt(value) => value.set_uniform(uniform_setter, location),
        }
    }
    pub fn equal_variant(&self, other: &Self) -> bool {
        match (self, other) {
            (GlUniform::Float(lhs), GlUniform::Float(rhs)) => FloatUniform::equal_variant(lhs, rhs),
            (GlUniform::Int(lhs), GlUniform::Int(rhs)) => IntUniform::equal_variant(lhs, rhs),
            (GlUniform::UInt(lhs), GlUniform::UInt(rhs)) => UIntUniform::equal_variant(lhs, rhs),
            (_, _) => {
                return false;
            }
        }
    }
}

///////////VERTEX-ATTRIBUTE-OBJECT
#[derive(Debug)]
pub struct InterleavedOffset {
    pub stride: u8,
    pub offset: u8,
}

pub struct AttributeDescription<'a> {
    pub location: u32,
    pub unit_type: NumberType,
    pub size: AttributeSize,
    pub buffer: &'a GlBuffer,
    pub normalize: bool,
    //Required for Interleaved attributes
    pub interleaved_offset: Option<InterleavedOffset>,
    pub per_instance_divisor: Option<u32>,
}

pub struct GlVertexArrayObject {
    context: Rc<wgl_context>,
    pub vao: WebGlVertexArrayObject,
}

impl GlVertexArrayObject {
    pub fn new(
        graphics: &Graphics,
        attribute_descriptors: Vec<AttributeDescription>,
        index_buffer: Option<&GlIndexBuffer>,
    ) -> Result<Self, JsValue> {
        match graphics.gl_context.create_vertex_array() {
            Some(vertex_array) => {
                let mut bound_points = std::collections::HashSet::new();
                graphics.gl_context.bind_vertex_array(Some(&vertex_array));
                for attribute in attribute_descriptors {
                    let buffer = attribute.buffer;
                    buffer.bind();

                    graphics
                        .gl_context
                        .enable_vertex_attrib_array(attribute.location);

                    let mut stride = 0;
                    let mut offset = 0;
                    if let Some(interleaved) = attribute.interleaved_offset {
                        stride = interleaved.stride as i32;
                        offset = interleaved.offset as i32;
                    }

                    if let Some(divisor) = attribute.per_instance_divisor {
                        if stride == 0 && divisor != 0 {
                            return Err(JsValue::from(
                                "The attribute is instanced but the stride was set to 0",
                            ));
                        }
                    }

                    if attribute.unit_type.is_integer_type() && !attribute.normalize {
                        graphics.gl_context.vertex_attrib_i_pointer_with_i32(
                            attribute.location,
                            attribute.size.into(),
                            attribute.unit_type.into(),
                            stride,
                            offset,
                        );
                    } else {
                        graphics.gl_context.vertex_attrib_pointer_with_i32(
                            attribute.location,
                            attribute.size.into(),
                            attribute.unit_type.into(),
                            attribute.normalize,
                            stride,
                            offset,
                        );
                    }

                    if let Some(divisor) = attribute.per_instance_divisor {
                        graphics
                            .gl_context
                            .vertex_attrib_divisor(attribute.location, divisor)
                    }

                    bound_points.insert(buffer.binding_point);
                }

                if let Some(index_buffer) = index_buffer {
                    if index_buffer.binding_point == BindingPoint::INDEX_BUFFER {
                        index_buffer.bind();

                        bound_points.insert(index_buffer.binding_point);
                    } else {
                        return Err(JsValue::from(
                            "Index buffer set but the index does not point to an Index buffer",
                        ));
                    }
                }

                graphics.gl_context.bind_vertex_array(None);
                for bp in bound_points.drain() {
                    graphics.gl_context.bind_buffer(bp.into(), None);
                }

                Ok(Self {
                    context: graphics.gl_context.clone(),
                    vao: vertex_array,
                })
            }
            None => Err(JsValue::from("Could not create vertex array object")),
        }
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
