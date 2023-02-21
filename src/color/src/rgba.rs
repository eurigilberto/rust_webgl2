use std::ops::Mul;

use crate::hsla::HSLA;

fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

pub enum RGBElems {
    R,
    G,
    B,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<RGBA> for RGB {
    fn from(color: RGBA) -> Self {
        RGB {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}

impl Into<RGBA> for RGB {
    fn into(self) -> RGBA {
        RGBA {
            r: self.r,
            g: self.g,
            b: self.b,
            a: 1.0,
        }
    }
}

impl Into<[f32; 3]> for RGB{
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

impl RGBA {
    pub fn rand_rgb() -> Self {
        Self {
            r: rand::random(),
            g: rand::random(),
            b: rand::random(),
            a: 1.0,
        }
    }
    pub fn lerp_rgba(&self, other: &Self, param: f32) -> Self {
        Self {
            r: lerp_f32(self.r, other.r, param),
            g: lerp_f32(self.g, other.g, param),
            b: lerp_f32(self.b, other.b, param),
            a: lerp_f32(self.a, other.a, param),
        }
    }
    pub fn gamma_corrected(self) -> Self {
        Self {
            r: f32::powf(self.r, 2.2),
            g: f32::powf(self.g, 2.2),
            b: f32::powf(self.b, 2.2),
            a: self.a,
        }
    }
    pub fn new_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: (r as f32) / 255.0,
            g: (g as f32) / 255.0,
            b: (b as f32) / 255.0,
            a: (a as f32) / 255.0,
        }
    }
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    pub const fn rrr1(r: f32) -> Self {
        Self {
            r,
            g: r,
            b: r,
            a: 1.0,
        }
    }
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const GREY: Self = Self {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    pub fn set_alpha(mut self, alpha: f32) -> Self {
        self.a = alpha;
        self
    }
}

impl Mul<f32> for RGBA {
    type Output = RGBA;

    /// The mutliplication does not change the alpha
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }
    }
}

impl From<[f32; 4]> for RGBA {
    fn from(value: [f32; 4]) -> Self {
        RGBA {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

impl From<RGBA> for [f32; 4] {
    fn from(rgba: RGBA) -> Self {
        [rgba.r, rgba.g, rgba.b, rgba.a]
    }
}

impl From<RGBA> for [f32; 3] {
    fn from(rgba: RGBA) -> Self {
        [rgba.r, rgba.g, rgba.b]
    }
}

impl From<RGBA> for HSLA {
    fn from(rgba: RGBA) -> Self {
        let mut cmax = (rgba.r, RGBElems::R);
        if cmax.0 < rgba.g {
            cmax = (rgba.g, RGBElems::G);
        }
        if cmax.0 < rgba.b {
            cmax = (rgba.b, RGBElems::B);
        }

        let cmin = f32::min(f32::min(rgba.r, rgba.g), rgba.b);
        let delta = cmax.0 - cmin;
        let hue;
        let lightness = (cmax.0 - cmin) / 2.0;
        let mut sat = 0.0;
        if f32::abs(delta) > f32::EPSILON {
            sat = delta / (1.0 - f32::abs(2.0 * lightness - 1.0));
        }
        let c_div_delta = |a: f32, b: f32| (a - b) / delta;

        match cmax.1 {
            RGBElems::R => {
                hue = 60.0 * (c_div_delta(rgba.g, rgba.b).rem_euclid(6.0));
            }
            RGBElems::G => {
                hue = 60.0 * (c_div_delta(rgba.b, rgba.r) + 2.0);
            }
            RGBElems::B => {
                hue = 60.0 * (c_div_delta(rgba.r, rgba.g) + 4.0);
            }
        };

        Self {
            h: hue,
            s: sat,
            l: lightness,
            a: rgba.a,
        }
    }
}
