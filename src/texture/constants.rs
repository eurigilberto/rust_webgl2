use crate::Graphics;
use web_sys::WebGl2RenderingContext as gl;

#[derive(Clone, Copy)]
pub struct TextureBindTarget(u32);
impl TextureBindTarget {
    pub const TEXTURE_2D: TextureBindTarget = TextureBindTarget(gl::TEXTURE_2D);
    pub const TEXTURE_3D: TextureBindTarget = TextureBindTarget(gl::TEXTURE_3D);
    pub const TEXTURE_CUBE_MAP: TextureBindTarget = TextureBindTarget(gl::TEXTURE_CUBE_MAP);
    pub const TEXTURE_2D_ARRAY: TextureBindTarget = TextureBindTarget(gl::TEXTURE_2D_ARRAY);
}
impl Into<u32> for TextureBindTarget {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct MagFilter(u32);
impl MagFilter {
    pub const LINEAR: MagFilter = MagFilter(gl::LINEAR);
    pub const NEAREST: MagFilter = MagFilter(gl::NEAREST);
}
impl Into<u32> for MagFilter {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct MinFilter(u32);
impl MinFilter {
    pub const LINEAR: MinFilter = MinFilter(gl::LINEAR);
    pub const NEAREST: MinFilter = MinFilter(gl::NEAREST);
    pub const NEAREST_MIPMAP_NEAREST: MinFilter = MinFilter(gl::NEAREST_MIPMAP_NEAREST);
    pub const LINEAR_MIPMAP_NEAREST: MinFilter = MinFilter(gl::LINEAR_MIPMAP_NEAREST);
    pub const NEAREST_MIPMAP_LINEAR: MinFilter = MinFilter(gl::NEAREST_MIPMAP_LINEAR);
    pub const LINEAR_MIPMAP_LINEAR: MinFilter = MinFilter(gl::LINEAR_MIPMAP_LINEAR);
}
impl Into<u32> for MinFilter {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct TextureWrap(u32);
impl TextureWrap {
    pub const REPEAT: TextureWrap = TextureWrap(gl::REPEAT);
    pub const CLAMP_TO_EDGE: TextureWrap = TextureWrap(gl::CLAMP_TO_EDGE);
    pub const MIRRORED_REPEAT: TextureWrap = TextureWrap(gl::MIRRORED_REPEAT);
}
impl Into<u32> for TextureWrap {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct TextureWrapSelect(u32);
impl TextureWrapSelect {
    pub const TEXTURE_WRAP_X: TextureWrapSelect = TextureWrapSelect(gl::TEXTURE_WRAP_S);
    pub const TEXTURE_WRAP_Y: TextureWrapSelect = TextureWrapSelect(gl::TEXTURE_WRAP_T);
    pub const TEXTURE_WRAP_Z: TextureWrapSelect = TextureWrapSelect(gl::TEXTURE_WRAP_R);
}
impl Into<u32> for TextureWrapSelect {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct TextureCompareFunc(u32);
impl TextureCompareFunc {
    pub const LEQUAL: TextureCompareFunc = TextureCompareFunc(gl::LEQUAL);
    pub const GEQUAL: TextureCompareFunc = TextureCompareFunc(gl::GEQUAL);
    pub const LESS: TextureCompareFunc = TextureCompareFunc(gl::LESS);
    pub const GREATER: TextureCompareFunc = TextureCompareFunc(gl::GREATER);
    pub const EQUAL: TextureCompareFunc = TextureCompareFunc(gl::EQUAL);
    pub const NOTEQUAL: TextureCompareFunc = TextureCompareFunc(gl::NOTEQUAL);
    pub const ALWAYS: TextureCompareFunc = TextureCompareFunc(gl::ALWAYS);
    pub const NEVER: TextureCompareFunc = TextureCompareFunc(gl::NEVER);
}
impl Into<u32> for TextureCompareFunc {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TextureInternalFormat(u32);
impl TextureInternalFormat {
    pub const R8: TextureInternalFormat = TextureInternalFormat(gl::R8);
    pub const R8_SNORM: TextureInternalFormat = TextureInternalFormat(gl::R8_SNORM);
    pub const RG8: TextureInternalFormat = TextureInternalFormat(gl::RG8);
    pub const RG8_SNORM: TextureInternalFormat = TextureInternalFormat(gl::RG8_SNORM);
    pub const RGB8: TextureInternalFormat = TextureInternalFormat(gl::RGB8);
    pub const RGB8_SNORM: TextureInternalFormat = TextureInternalFormat(gl::RGB8_SNORM);
    pub const RGB565: TextureInternalFormat = TextureInternalFormat(gl::RGB565);
    pub const RGBA4: TextureInternalFormat = TextureInternalFormat(gl::RGBA4);
    pub const RGB5_A1: TextureInternalFormat = TextureInternalFormat(gl::RGB5_A1);
    pub const RGBA8: TextureInternalFormat = TextureInternalFormat(gl::RGBA8);
    pub const RGBA8_SNORM: TextureInternalFormat = TextureInternalFormat(gl::RGBA8_SNORM);
    pub const RGB10_A2: TextureInternalFormat = TextureInternalFormat(gl::RGB10_A2);
    pub const RGB10_A2UI: TextureInternalFormat = TextureInternalFormat(gl::RGB10_A2UI);
    pub const SRGB8: TextureInternalFormat = TextureInternalFormat(gl::SRGB8);
    pub const SRGB8_ALPHA8: TextureInternalFormat = TextureInternalFormat(gl::SRGB8_ALPHA8);
    pub const R16F: TextureInternalFormat = TextureInternalFormat(gl::R16F);
    pub const RG16F: TextureInternalFormat = TextureInternalFormat(gl::RG16F);
    pub const RGB16F: TextureInternalFormat = TextureInternalFormat(gl::RGB16F);
    pub const RGBA16F: TextureInternalFormat = TextureInternalFormat(gl::RGBA16F);
    pub const R32F: TextureInternalFormat = TextureInternalFormat(gl::R32F);
    pub const RG32F: TextureInternalFormat = TextureInternalFormat(gl::RG32F);
    pub const RGB32F: TextureInternalFormat = TextureInternalFormat(gl::RGB32F);
    pub const RGBA32F: TextureInternalFormat = TextureInternalFormat(gl::RGBA32F);
    pub const R11F_G11F_B10F: TextureInternalFormat = TextureInternalFormat(gl::R11F_G11F_B10F);
    pub const RGB9_E5: TextureInternalFormat = TextureInternalFormat(gl::RGB9_E5);
    pub const R8I: TextureInternalFormat = TextureInternalFormat(gl::R8I);
    pub const R8UI: TextureInternalFormat = TextureInternalFormat(gl::R8UI);
    pub const R16I: TextureInternalFormat = TextureInternalFormat(gl::R16I);
    pub const R16UI: TextureInternalFormat = TextureInternalFormat(gl::R16UI);
    pub const R32I: TextureInternalFormat = TextureInternalFormat(gl::R32I);
    pub const R32UI: TextureInternalFormat = TextureInternalFormat(gl::R32UI);
    pub const RG8I: TextureInternalFormat = TextureInternalFormat(gl::RG8I);
    pub const RG8UI: TextureInternalFormat = TextureInternalFormat(gl::RG8UI);
    pub const RG16I: TextureInternalFormat = TextureInternalFormat(gl::RG16I);
    pub const RG16UI: TextureInternalFormat = TextureInternalFormat(gl::RG16UI);
    pub const RG32I: TextureInternalFormat = TextureInternalFormat(gl::RG32I);
    pub const RG32UI: TextureInternalFormat = TextureInternalFormat(gl::RG32UI);
    pub const RGB8I: TextureInternalFormat = TextureInternalFormat(gl::RGB8I);
    pub const RGB8UI: TextureInternalFormat = TextureInternalFormat(gl::RGB8UI);
    pub const RGB16I: TextureInternalFormat = TextureInternalFormat(gl::RGB16I);
    pub const RGB16UI: TextureInternalFormat = TextureInternalFormat(gl::RGB16UI);
    pub const RGB32I: TextureInternalFormat = TextureInternalFormat(gl::RGB32I);
    pub const RGB32UI: TextureInternalFormat = TextureInternalFormat(gl::RGB32UI);
    pub const RGBA8I: TextureInternalFormat = TextureInternalFormat(gl::RGBA8I);
    pub const RGBA8UI: TextureInternalFormat = TextureInternalFormat(gl::RGBA8UI);
    pub const RGBA16I: TextureInternalFormat = TextureInternalFormat(gl::RGBA16I);
    pub const RGBA16UI: TextureInternalFormat = TextureInternalFormat(gl::RGBA16UI);
    pub const RGBA32I: TextureInternalFormat = TextureInternalFormat(gl::RGBA32I);
    pub const RGBA32UI: TextureInternalFormat = TextureInternalFormat(gl::RGBA32UI);
    //----
    pub const DEPTH_COMPONENT16: TextureInternalFormat =
        TextureInternalFormat(gl::DEPTH_COMPONENT16);
    pub const DEPTH_COMPONENT24: TextureInternalFormat =
        TextureInternalFormat(gl::DEPTH_COMPONENT24);
    pub const DEPTH_COMPONENT32F: TextureInternalFormat =
        TextureInternalFormat(gl::DEPTH_COMPONENT32F);
    pub const DEPTH24_STENCIL8: TextureInternalFormat = TextureInternalFormat(gl::DEPTH24_STENCIL8);
}
impl Into<u32> for TextureInternalFormat {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct TextureFormat(u32);
impl Into<TextureFormat> for TextureInternalFormat {
    fn into(self) -> TextureFormat {
        match self {
            TextureInternalFormat::R8 => TextureFormat(gl::RED),
            TextureInternalFormat::R16F => TextureFormat(gl::RED),
            TextureInternalFormat::R32F => TextureFormat(gl::RED),
            TextureInternalFormat::R8UI => TextureFormat(gl::RED_INTEGER),
            //---
            TextureInternalFormat::RG8 => TextureFormat(gl::RG),
            TextureInternalFormat::RG16F => TextureFormat(gl::RG),
            TextureInternalFormat::RG32F => TextureFormat(gl::RG),
            TextureInternalFormat::RG8UI => TextureFormat(gl::RG_INTEGER),
            //---
            TextureInternalFormat::RGB8 => TextureFormat(gl::RGB),
            TextureInternalFormat::SRGB8 => TextureFormat(gl::RGB),
            TextureInternalFormat::RGB565 => TextureFormat(gl::RGB),
            TextureInternalFormat::R11F_G11F_B10F => TextureFormat(gl::RGB),
            TextureInternalFormat::RGB9_E5 => TextureFormat(gl::RGB),
            TextureInternalFormat::RGB16F => TextureFormat(gl::RGB),
            TextureInternalFormat::RGB32F => TextureFormat(gl::RGB),
            TextureInternalFormat::RGB8UI => TextureFormat(gl::RGB_INTEGER),
			//----
			TextureInternalFormat::RGBA8 => TextureFormat(gl::RGBA),
			TextureInternalFormat::SRGB8_ALPHA8 => TextureFormat(gl::RGBA),
			TextureInternalFormat::RGB5_A1 => TextureFormat(gl::RGBA),
			TextureInternalFormat::RGB10_A2 => TextureFormat(gl::RGBA),
			TextureInternalFormat::RGBA4 => TextureFormat(gl::RGBA),
    		TextureInternalFormat::RGBA16F => TextureFormat(gl::RGBA),
    		TextureInternalFormat::RGBA32F => TextureFormat(gl::RGBA),
    		TextureInternalFormat::RGBA8UI => TextureFormat(gl::RGBA_INTEGER),
			//----
			TextureInternalFormat::DEPTH_COMPONENT16 => TextureFormat(gl::DEPTH_COMPONENT),
			TextureInternalFormat::DEPTH_COMPONENT24 => TextureFormat(gl::DEPTH_COMPONENT),
			TextureInternalFormat::DEPTH_COMPONENT32F => TextureFormat(gl::DEPTH_COMPONENT),
			TextureInternalFormat::DEPTH24_STENCIL8 => TextureFormat(gl::DEPTH_STENCIL),
            _ => {
                panic!("Invalid internal format")
            }
        }
    }
}
impl Into<u32> for TextureFormat {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct TextureType(u32);
impl Into<TextureType> for TextureInternalFormat {
    fn into(self) -> TextureType {
        match self {
            TextureInternalFormat::R8 => TextureType(gl::UNSIGNED_BYTE),
    		TextureInternalFormat::R16F => TextureType(gl::HALF_FLOAT),
			TextureInternalFormat::R32F => TextureType(gl::FLOAT),
    		TextureInternalFormat::R8UI => TextureType(gl::UNSIGNED_BYTE),
    		//----
    		TextureInternalFormat::RG8 => TextureType(gl::UNSIGNED_BYTE),
    		TextureInternalFormat::RG16F => TextureType(gl::HALF_FLOAT),
    		TextureInternalFormat::RG32F => TextureType(gl::FLOAT),
    		TextureInternalFormat::RG8UI => TextureType(gl::UNSIGNED_BYTE),
    		//----
			TextureInternalFormat::RGB8 => TextureType(gl::UNSIGNED_BYTE),
			TextureInternalFormat::SRGB8 => TextureType(gl::UNSIGNED_BYTE),
			TextureInternalFormat::RGB565 => TextureType(gl::UNSIGNED_SHORT_5_6_5),
			TextureInternalFormat::R11F_G11F_B10F => TextureType(gl::HALF_FLOAT),
    		TextureInternalFormat::RGB9_E5 => TextureType(gl::HALF_FLOAT),
    		TextureInternalFormat::RGB16F => TextureType(gl::HALF_FLOAT),
    		TextureInternalFormat::RGB32F => TextureType(gl::FLOAT),
    		TextureInternalFormat::RGB8UI => TextureType(gl::UNSIGNED_BYTE),
    		//----
    		TextureInternalFormat::RGBA8 => TextureType(gl::UNSIGNED_BYTE),
    		TextureInternalFormat::SRGB8_ALPHA8 => TextureType(gl::UNSIGNED_BYTE),
    		TextureInternalFormat::RGB5_A1 => TextureType(gl::UNSIGNED_SHORT_5_5_5_1),
    		TextureInternalFormat::RGB10_A2 => TextureType(gl::UNSIGNED_INT_2_10_10_10_REV),
    		TextureInternalFormat::RGBA4 => TextureType(gl::UNSIGNED_SHORT_4_4_4_4),
    		TextureInternalFormat::RGBA16F => TextureType(gl::HALF_FLOAT),
    		TextureInternalFormat::RGBA32F => TextureType(gl::FLOAT),
    		TextureInternalFormat::RGBA8UI => TextureType(gl::UNSIGNED_BYTE),
			//----
			TextureInternalFormat::DEPTH_COMPONENT16 => TextureType(gl::UNSIGNED_SHORT),
			TextureInternalFormat::DEPTH_COMPONENT24 => TextureType(gl::UNSIGNED_INT),
			TextureInternalFormat::DEPTH_COMPONENT32F => TextureType(gl::FLOAT),
			TextureInternalFormat::DEPTH24_STENCIL8 => TextureType(gl::UNSIGNED_INT_24_8),
            _ => {
                panic!("Invalid internal format")
            }
        }
    }
}
impl Into<u32> for TextureType {
    fn into(self) -> u32 {
        self.0
    }
}