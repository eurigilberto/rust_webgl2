use color::{RGBA, RGB};
use glam::*;
use crate::{GlUniform, FloatUniform, UIntUniform, IntUniform};

pub trait IntoGlUniform{
	fn uniform(&self)->GlUniform;
}

impl IntoGlUniform for f32{
    fn uniform(&self)->GlUniform {
        FloatUniform::Scalar(*self).into()
    }
}

impl IntoGlUniform for u32{
    fn uniform(&self)->GlUniform {
        UIntUniform::Scalar(*self).into()
    }
}

impl IntoGlUniform for i32{
    fn uniform(&self)->GlUniform {
        IntUniform::Scalar(*self).into()
    }
}

impl IntoGlUniform for Vec4{
    fn uniform(&self)->GlUniform {
        FloatUniform::Vec4(*self).into()
    }
}

impl IntoGlUniform for Vec3{
    fn uniform(&self)->GlUniform {
        FloatUniform::Vec3(*self).into()
    }
}

impl IntoGlUniform for Mat4{
    fn uniform(&self)->GlUniform {
        FloatUniform::Mat4(*self).into()
    }
}

impl IntoGlUniform for Vec2{
    fn uniform(&self)->GlUniform {
        FloatUniform::Vec2(*self).into()
    }
}

impl IntoGlUniform for RGBA{
	fn uniform(&self)->GlUniform {
		FloatUniform::Vec4(Vec4::from_array((*self).into())).into()
	}
}

impl IntoGlUniform for RGB{
	fn uniform(&self)->GlUniform {
		FloatUniform::Vec3(Vec3::from_array((*self).into())).into()
	}
}