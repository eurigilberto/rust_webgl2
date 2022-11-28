use web_sys::WebGl2RenderingContext as wgl_context;

#[derive(Clone, Copy)]
pub struct BlendEquation(u32);
impl BlendEquation {
    pub const FUNC_ADD: BlendEquation = BlendEquation(wgl_context::FUNC_ADD);
    pub const FUNC_SUBTRACT: BlendEquation = BlendEquation(wgl_context::FUNC_SUBTRACT);
    pub const FUNC_REVERSE_SUBTRACT: BlendEquation =
        BlendEquation(wgl_context::FUNC_REVERSE_SUBTRACT);
    pub const MIN: BlendEquation = BlendEquation(wgl_context::MIN);
    pub const MAX: BlendEquation = BlendEquation(wgl_context::MAX);
}
impl Into<u32> for BlendEquation {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct BlendFuncFactor(u32);
impl BlendFuncFactor {
    pub const ZERO: BlendFuncFactor = BlendFuncFactor(wgl_context::ZERO);
    pub const ONE: BlendFuncFactor = BlendFuncFactor(wgl_context::ONE);
    pub const SRC_COLOR: BlendFuncFactor = BlendFuncFactor(wgl_context::SRC_COLOR);
    pub const ONE_MINUS_SRC_COLOR: BlendFuncFactor =
        BlendFuncFactor(wgl_context::ONE_MINUS_SRC_COLOR);
    pub const DST_COLOR: BlendFuncFactor = BlendFuncFactor(wgl_context::DST_COLOR);
    pub const ONE_MINUS_DST_COLOR: BlendFuncFactor =
        BlendFuncFactor(wgl_context::ONE_MINUS_DST_COLOR);
    pub const SRC_ALPHA: BlendFuncFactor = BlendFuncFactor(wgl_context::SRC_ALPHA);
    pub const ONE_MINUS_SRC_ALPHA: BlendFuncFactor =
        BlendFuncFactor(wgl_context::ONE_MINUS_SRC_ALPHA);
    pub const DST_ALPHA: BlendFuncFactor = BlendFuncFactor(wgl_context::DST_ALPHA);
    pub const ONE_MINUS_DST_ALPHA: BlendFuncFactor =
        BlendFuncFactor(wgl_context::ONE_MINUS_DST_ALPHA);
    pub const CONSTANT_COLOR: BlendFuncFactor = BlendFuncFactor(wgl_context::CONSTANT_COLOR);
    pub const ONE_MINUS_CONSTANT_COLOR: BlendFuncFactor =
        BlendFuncFactor(wgl_context::ONE_MINUS_CONSTANT_COLOR);
    pub const CONSTANT_ALPHA: BlendFuncFactor = BlendFuncFactor(wgl_context::CONSTANT_ALPHA);
    pub const ONE_MINUS_CONSTANT_ALPHA: BlendFuncFactor =
        BlendFuncFactor(wgl_context::ONE_MINUS_CONSTANT_ALPHA);
    pub const SRC_ALPHA_SATURATE: BlendFuncFactor =
        BlendFuncFactor(wgl_context::SRC_ALPHA_SATURATE);
}
impl Into<u32> for BlendFuncFactor {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct StencilFunc(u32);
impl StencilFunc {
    pub const NEVER: StencilFunc = StencilFunc(wgl_context::NEVER);
    pub const LESS: StencilFunc = StencilFunc(wgl_context::LESS);
    pub const EQUAL: StencilFunc = StencilFunc(wgl_context::EQUAL);
    pub const LEQUAL: StencilFunc = StencilFunc(wgl_context::LEQUAL);
    pub const GREATER: StencilFunc = StencilFunc(wgl_context::GREATER);
    pub const NOTEQUAL: StencilFunc = StencilFunc(wgl_context::NOTEQUAL);
    pub const GEQUAL: StencilFunc = StencilFunc(wgl_context::GEQUAL);
    pub const ALWAYS: StencilFunc = StencilFunc(wgl_context::ALWAYS);
}
impl Into<u32> for StencilFunc {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct StencilOp(u32);
impl StencilOp {
    pub const KEEP: StencilOp = StencilOp(wgl_context::KEEP);
    pub const ZERO: StencilOp = StencilOp(wgl_context::ZERO);
    pub const REPLACE: StencilOp = StencilOp(wgl_context::REPLACE);
    pub const INCR: StencilOp = StencilOp(wgl_context::INCR);
    pub const INCR_WRAP: StencilOp = StencilOp(wgl_context::INCR_WRAP);
    pub const DECR: StencilOp = StencilOp(wgl_context::DECR);
    pub const DECR_WRAP: StencilOp = StencilOp(wgl_context::DECR_WRAP);
    pub const INVERT: StencilOp = StencilOp(wgl_context::INVERT);
}
impl Into<u32> for StencilOp {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct ShaderType(u32);
#[allow(dead_code)]
impl ShaderType {
    pub const VERTEX_SHADER: ShaderType = ShaderType(wgl_context::VERTEX_SHADER);
    pub const FRAGMENT_SHADER: ShaderType = ShaderType(wgl_context::FRAGMENT_SHADER);
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: ShaderType =
        ShaderType(wgl_context::FRAGMENT_SHADER_DERIVATIVE_HINT);
}
impl Into<u32> for ShaderType {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebGlCapability(u32);
#[allow(dead_code)]
impl WebGlCapability {
    pub const BLEND: WebGlCapability = WebGlCapability(wgl_context::BLEND);
    pub const CULL_FACE: WebGlCapability = WebGlCapability(wgl_context::CULL_FACE);
    pub const DEPTH_TEST: WebGlCapability = WebGlCapability(wgl_context::DEPTH_TEST);
    pub const DITHER: WebGlCapability = WebGlCapability(wgl_context::DITHER);
    pub const POLYGON_OFFSET_FILL: WebGlCapability =
        WebGlCapability(wgl_context::POLYGON_OFFSET_FILL);
    pub const SAMPLE_ALPHA_TO_COVERAGE: WebGlCapability =
        WebGlCapability(wgl_context::SAMPLE_ALPHA_TO_COVERAGE);
    pub const SAMPLE_COVERAGE: WebGlCapability = WebGlCapability(wgl_context::SAMPLE_COVERAGE);
    pub const SCISSOR_TEST: WebGlCapability = WebGlCapability(wgl_context::SCISSOR_TEST);
    pub const STENCIL_TEST: WebGlCapability = WebGlCapability(wgl_context::STENCIL_TEST);
    pub const RASTERIZER_DISCARD: WebGlCapability =
        WebGlCapability(wgl_context::RASTERIZER_DISCARD);
}
impl Into<u32> for WebGlCapability {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindingPoint(u32);
#[allow(dead_code)]
impl BindingPoint {
    pub const ARRAY_BUFFER: BindingPoint = BindingPoint(wgl_context::ARRAY_BUFFER);
    pub const ELEMENT_ARRAY_BUFFER: BindingPoint = BindingPoint(wgl_context::ELEMENT_ARRAY_BUFFER);
    pub const INDEX_BUFFER: BindingPoint = BindingPoint::ELEMENT_ARRAY_BUFFER;
    pub const COPY_READ_BUFFER: BindingPoint = BindingPoint(wgl_context::COPY_READ_BUFFER);
    pub const COPY_WRITE_BUFFER: BindingPoint = BindingPoint(wgl_context::COPY_WRITE_BUFFER);
    pub const TRANSFORM_FEEDBACK_BUFFER: BindingPoint =
        BindingPoint(wgl_context::TRANSFORM_FEEDBACK_BUFFER);
    pub const UNIFORM_BUFFER: BindingPoint = BindingPoint(wgl_context::UNIFORM_BUFFER);
    pub const PIXEL_PACK_BUFFER: BindingPoint = BindingPoint(wgl_context::PIXEL_PACK_BUFFER);
    pub const PIXEL_UNPACK_BUFFER: BindingPoint = BindingPoint(wgl_context::PIXEL_UNPACK_BUFFER);
}
impl Into<u32> for BindingPoint {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct BufferUsage(u32);
#[allow(dead_code)]
impl BufferUsage {
    pub const STATIC_DRAW: BufferUsage = BufferUsage(wgl_context::STATIC_DRAW);
    pub const DYNAMIC_DRAW: BufferUsage = BufferUsage(wgl_context::DYNAMIC_DRAW);
    pub const STREAM_DRAW: BufferUsage = BufferUsage(wgl_context::STREAM_DRAW);
    pub const STATIC_READ: BufferUsage = BufferUsage(wgl_context::STATIC_READ);
    pub const DYNAMIC_READ: BufferUsage = BufferUsage(wgl_context::DYNAMIC_READ);
    pub const STREAM_READ: BufferUsage = BufferUsage(wgl_context::STREAM_READ);
    pub const STATIC_COPY: BufferUsage = BufferUsage(wgl_context::STATIC_COPY);
    pub const DYNAMIC_COPY: BufferUsage = BufferUsage(wgl_context::DYNAMIC_COPY);
    pub const STREAM_COPY: BufferUsage = BufferUsage(wgl_context::STREAM_COPY);
}
impl Into<u32> for BufferUsage {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NumberType(u32);
#[allow(dead_code)]
impl NumberType {
    pub const BYTE: NumberType = NumberType(wgl_context::BYTE);
    pub const SHORT: NumberType = NumberType(wgl_context::SHORT);
    pub const UNSIGNED_BYTE: NumberType = NumberType(wgl_context::UNSIGNED_BYTE);
    pub const UNSIGNED_SHORT: NumberType = NumberType(wgl_context::UNSIGNED_SHORT);
    pub const INT: NumberType = NumberType(wgl_context::INT);
    pub const UNSIGNED_INT: NumberType = NumberType(wgl_context::UNSIGNED_INT);

    pub const FLOAT: NumberType = NumberType(wgl_context::FLOAT);
    pub const HALF_FLOAT: NumberType = NumberType(wgl_context::HALF_FLOAT);

    pub fn is_integer_type(self) -> bool {
        match self {
            Self::BYTE
            | Self::SHORT
            | Self::UNSIGNED_BYTE
            | Self::UNSIGNED_SHORT
            | Self::INT
            | Self::UNSIGNED_INT => {
                return true;
            }
            Self::FLOAT | Self::HALF_FLOAT => {
                return false;
            }
            _ => {
                panic!("This should be unreachable")
            }
        }
    }
}
impl Into<u32> for NumberType {
    fn into(self) -> u32 {
        self.0
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct ProgramParamerter(pub u32);
#[allow(dead_code)]
impl ProgramParamerter {
    pub const DELETE_STATUS: ProgramParamerter = ProgramParamerter(wgl_context::DELETE_STATUS);
    pub const LINK_STATUS: ProgramParamerter = ProgramParamerter(wgl_context::LINK_STATUS);
    pub const VALIDATE_STATUS: ProgramParamerter = ProgramParamerter(wgl_context::VALIDATE_STATUS);
    pub const ATTACHED_SHADERS: ProgramParamerter =
        ProgramParamerter(wgl_context::ATTACHED_SHADERS);
    pub const ACTIVE_ATTRIBUTES: ProgramParamerter =
        ProgramParamerter(wgl_context::ACTIVE_ATTRIBUTES);
    pub const ACTIVE_UNIFORMS: ProgramParamerter = ProgramParamerter(wgl_context::ACTIVE_UNIFORMS);
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: ProgramParamerter =
        ProgramParamerter(wgl_context::TRANSFORM_FEEDBACK_BUFFER_MODE);
    pub const TRANSFORM_FEEDBACK_VARYINGS: ProgramParamerter =
        ProgramParamerter(wgl_context::TRANSFORM_FEEDBACK_VARYINGS);
    pub const ACTIVE_UNIFORM_BLOCKS: ProgramParamerter =
        ProgramParamerter(wgl_context::ACTIVE_UNIFORM_BLOCKS);
}
impl Into<u32> for ProgramParamerter {
    fn into(self) -> u32 {
        self.0
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct DepthFunction(u32);
#[allow(dead_code)]
impl DepthFunction {
    pub const NEVER: DepthFunction = DepthFunction(wgl_context::NEVER);
    pub const LESS: DepthFunction = DepthFunction(wgl_context::LESS);
    pub const EQUAL: DepthFunction = DepthFunction(wgl_context::EQUAL);
    pub const LEQUAL: DepthFunction = DepthFunction(wgl_context::LEQUAL);
    pub const GREATER: DepthFunction = DepthFunction(wgl_context::GREATER);
    pub const NOTEQUAL: DepthFunction = DepthFunction(wgl_context::NOTEQUAL);
    pub const GEQUAL: DepthFunction = DepthFunction(wgl_context::GEQUAL);
    pub const ALWAYS: DepthFunction = DepthFunction(wgl_context::ALWAYS);
}
impl Into<u32> for DepthFunction{
    fn into(self) -> u32 {
        self.0
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct AttributeSize(i32);
#[allow(dead_code)]
impl AttributeSize {
    pub const ONE: AttributeSize = AttributeSize(1);
    pub const TWO: AttributeSize = AttributeSize(2);
    pub const THREE: AttributeSize = AttributeSize(3);
    pub const FOUR: AttributeSize = AttributeSize(4);
}
impl Into<i32> for AttributeSize {
    fn into(self) -> i32 {
        self.0
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct PrimitiveType(u32);
#[allow(dead_code)]
impl PrimitiveType{
	pub const POINTS: PrimitiveType = PrimitiveType(wgl_context::POINTS);
	pub const LINE_STRIP: PrimitiveType = PrimitiveType(wgl_context::LINE_STRIP);
	pub const LINE_LOOP: PrimitiveType = PrimitiveType(wgl_context::LINE_LOOP);
	pub const LINES: PrimitiveType = PrimitiveType(wgl_context::LINES);
	pub const TRIANGLE_STRIP: PrimitiveType = PrimitiveType(wgl_context::TRIANGLE_STRIP);
	pub const TRIANGLE_FAN: PrimitiveType = PrimitiveType(wgl_context::TRIANGLE_FAN);
	pub const TRIANGLES: PrimitiveType = PrimitiveType(wgl_context::TRIANGLES);
}
impl Into<u32> for PrimitiveType{
    fn into(self) -> u32 {
        self.0
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct IndexType(u32);
#[allow(dead_code)]
impl IndexType{
	pub const U8: IndexType = IndexType(wgl_context::UNSIGNED_BYTE);
	pub const U16: IndexType = IndexType(wgl_context::UNSIGNED_SHORT);
	pub const U32: IndexType = IndexType(wgl_context::UNSIGNED_INT);
}
impl Into<u32> for IndexType{
    fn into(self) -> u32 {
        self.0
    }
}



#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct CullMode(u32);
#[allow(dead_code)]
impl CullMode{
	pub const FRONT: CullMode = CullMode(wgl_context::FRONT);
	pub const BACK: CullMode = CullMode(wgl_context::BACK);
	pub const FRONT_AND_BACK: CullMode = CullMode(wgl_context::FRONT_AND_BACK);
}
impl Into<u32> for CullMode{
    fn into(self) -> u32 {
        self.0
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct FrontFaceWinding(u32);
#[allow(dead_code)]
impl FrontFaceWinding{
	pub const CLOCK_WISE: FrontFaceWinding = FrontFaceWinding(wgl_context::CW);
	pub const COUNTER_CLOCK_WISE: FrontFaceWinding = FrontFaceWinding(wgl_context::CCW);
}
impl Into<u32> for FrontFaceWinding{
    fn into(self) -> u32 {
        self.0
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct ClearMaskBits(u32);
#[allow(dead_code)]
impl ClearMaskBits{
    pub const COLOR_BUFFER_BIT: ClearMaskBits = ClearMaskBits(wgl_context::COLOR_BUFFER_BIT);
    pub const DEPTH_BUFFER_BIT: ClearMaskBits = ClearMaskBits(wgl_context::DEPTH_BUFFER_BIT);
    pub const STENCIL_BUFFER_BIT: ClearMaskBits = ClearMaskBits(wgl_context::STENCIL_BUFFER_BIT);
    pub const fn value(self)->u32{
        self.0
    }
}
impl Into<u32> for ClearMaskBits{
    fn into(self) -> u32 {
        self.0
    }
}