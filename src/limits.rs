use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as wgl_context;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct DeviceLimit(u32);
#[allow(dead_code)]
impl DeviceLimit{
    pub const MAX_3D_TEXTURE_SIZE: DeviceLimit = DeviceLimit(wgl_context::MAX_3D_TEXTURE_SIZE);
    pub const MAX_ARRAY_TEXTURE_LAYERS: DeviceLimit = DeviceLimit(wgl_context::MAX_ARRAY_TEXTURE_LAYERS);
    pub const MAX_CLIENT_WAIT_TIMEOUT_WEBGL: DeviceLimit = DeviceLimit(wgl_context::MAX_CLIENT_WAIT_TIMEOUT_WEBGL);
    pub const MAX_COLOR_ATTACHMENTS: DeviceLimit = DeviceLimit(wgl_context::MAX_COLOR_ATTACHMENTS);
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: DeviceLimit = DeviceLimit(wgl_context::MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS);
    pub const MAX_COMBINED_UNIFORM_BLOCKS: DeviceLimit = DeviceLimit(wgl_context::MAX_COMBINED_UNIFORM_BLOCKS);
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: DeviceLimit = DeviceLimit(wgl_context::MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS);
    pub const MAX_DRAW_BUFFERS: DeviceLimit = DeviceLimit(wgl_context::MAX_DRAW_BUFFERS);
    pub const MAX_ELEMENT_INDEX: DeviceLimit = DeviceLimit(wgl_context::MAX_ELEMENT_INDEX);
    pub const MAX_ELEMENTS_INDICES: DeviceLimit = DeviceLimit(wgl_context::MAX_ELEMENTS_INDICES);
    pub const MAX_ELEMENTS_VERTICES: DeviceLimit = DeviceLimit(wgl_context::MAX_ELEMENTS_VERTICES);
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: DeviceLimit = DeviceLimit(wgl_context::MAX_FRAGMENT_INPUT_COMPONENTS);
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: DeviceLimit = DeviceLimit(wgl_context::MAX_FRAGMENT_UNIFORM_BLOCKS);
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: DeviceLimit = DeviceLimit(wgl_context::MAX_FRAGMENT_UNIFORM_COMPONENTS);
    pub const MAX_PROGRAM_TEXEL_OFFSET: DeviceLimit = DeviceLimit(wgl_context::MAX_PROGRAM_TEXEL_OFFSET);
    pub const MAX_SAMPLES: DeviceLimit = DeviceLimit(wgl_context::MAX_SAMPLES);
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: DeviceLimit = DeviceLimit(wgl_context::MAX_VERTEX_TEXTURE_IMAGE_UNITS);
    pub const MAX_SERVER_WAIT_TIMEOUT: DeviceLimit = DeviceLimit(wgl_context::MAX_SERVER_WAIT_TIMEOUT);
    pub const MAX_TEXTURE_LOD_BIAS: DeviceLimit = DeviceLimit(wgl_context::MAX_TEXTURE_LOD_BIAS);
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: DeviceLimit = DeviceLimit(wgl_context::MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS);
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: DeviceLimit = DeviceLimit(wgl_context::MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS);
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: DeviceLimit = DeviceLimit(wgl_context::MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS);
    pub const MAX_UNIFORM_BLOCK_SIZE: DeviceLimit = DeviceLimit(wgl_context::MAX_UNIFORM_BLOCK_SIZE);
    pub const MAX_UNIFORM_BUFFER_BINDINGS: DeviceLimit = DeviceLimit(wgl_context::MAX_UNIFORM_BUFFER_BINDINGS);
    pub const MAX_VARYING_COMPONENTS: DeviceLimit = DeviceLimit(wgl_context::MAX_VARYING_COMPONENTS);
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: DeviceLimit = DeviceLimit(wgl_context::MAX_VERTEX_OUTPUT_COMPONENTS);
    pub const MAX_VERTEX_UNIFORM_BLOCKS: DeviceLimit = DeviceLimit(wgl_context::MAX_VERTEX_UNIFORM_BLOCKS);
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: DeviceLimit = DeviceLimit(wgl_context::MAX_VERTEX_UNIFORM_COMPONENTS);
    pub const MIN_PROGRAM_TEXEL_OFFSET: DeviceLimit = DeviceLimit(wgl_context::MIN_PROGRAM_TEXEL_OFFSET);

    pub const fn value(self)->u32{
        self.0
    }
}

impl Into<u32> for DeviceLimit{
    fn into(self) -> u32 {
        self.0
    }
}

#[allow(dead_code)]
pub fn get_limit_parameter(context: &wgl_context, param: DeviceLimit)->Result<i64, JsValue>{
    match context.get_parameter(param.into()){
        Ok(value) => {
            match value.as_f64() {
                Some(val) => Ok(val as i64),
                None => Err(JsValue::from("Value is not a number")),
            }
        },
        Err(error) => {
            Err(error)
        },
    }
}