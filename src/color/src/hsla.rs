use crate::rgba::RGBA;
pub struct HSLA{
	pub h: f32,
	pub s: f32,
	pub l: f32,
	pub a: f32
}

impl From<HSLA> for [f32;4]{
	fn from(hsla: HSLA)->Self{
		return [hsla.h, hsla.s, hsla.l, hsla.a]
	}
}

impl From<HSLA> for RGBA{
	fn from(hsla: HSLA)->Self{
		let c = (1.0 - f32::abs(2.0 * hsla.l - 1.0)) * hsla.s;
		let x = c * (1.0 - f32::abs((hsla.h / 60.0).rem_euclid(2.0) - 1.0));
		let m = hsla.l - c / 2.0;

		let mut rgba = RGBA{r:0.0, g:0.0, b:0.0, a: hsla.a};

		if hsla.h >= 0.0 && hsla.h < 60.0 {
			rgba.r = c;
			rgba.g = x;
		} else if hsla.h >= 60.0 && hsla.h < 120.0 {
			rgba.r = x;
			rgba.g = c;
		} else if hsla.h >= 120.0 && hsla.h < 180.0 {
			rgba.g = c;
			rgba.b = x;
		} else if hsla.h >= 180.0 && hsla.h < 240.0 {
			rgba.g = x;
			rgba.b = c;
		} else if hsla.h >= 240.0 && hsla.h < 300.0 {
			rgba.r = x;
			rgba.b = c;
		} else if hsla.h >= 300.0 && hsla.h < 360.0 {
			rgba.r = c;
			rgba.b = x;
		}

		rgba.r += m;
		rgba.g += m;
		rgba.b += m;

		return rgba;
	}
}