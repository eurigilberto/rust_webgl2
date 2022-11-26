pub mod lighting_functions;

pub mod dither{
    use webgl2_shader_definition::*;
    pub const NAME: &str = "dither_fn";
    pub const PARAMS: &str = "vec2 screen_position_px, float in_value";
    pub const FN: &str = r#"
vec2 uv = screen_position_px;
float DITHER_THRESHOLDS[16] =
float[16](
    1.0 / 17.0,  9.0 / 17.0,  3.0 / 17.0, 11.0 / 17.0,
    13.0 / 17.0,  5.0 / 17.0, 15.0 / 17.0,  7.0 / 17.0,
    4.0 / 17.0, 12.0 / 17.0,  2.0 / 17.0, 10.0 / 17.0,
    16.0 / 17.0,  8.0 / 17.0, 14.0 / 17.0,  6.0 / 17.0
);
uint index = uint(mod(uv.x, 4.0)) * 4u + uint(mod(uv.y, 4.0));
return in_value - DITHER_THRESHOLDS[index];"#;

    // Code taken from https://docs.unity3d.com/Packages/com.unity.shadergraph@13.1/manual/Dither-Node.html
    pub fn definition() -> FunctionDefinition{
        FunctionDefinition {
            name: NAME.into(),
            definition: FunctionDefinitionType::InlineFn {
                return_type: WebGLDataType::Float,
                parameters: PARAMS.into(),
                body: FN.into(),
            },
        }
    }
}

pub mod fast_hash {
    use webgl2_shader_definition::*;
    pub const NAME: &str = "fast_hash";
    pub const PARAMS: &str = "vec2 v";
    pub const FN: &str = r#"
v = (1./4320.) * v + vec2(0.25,0.);
float state = fract( dot( v * v, vec2(3571)));
return fract( state * state * (3571. * 2.));
"#;
    pub const RETURN_TYPE: WebGLDataType = WebGLDataType::Float;
    // UE4's RandFast function
    // https://github.com/EpicGames/UnrealEngine/blob/release/Engine/Shaders/Private/Random.ush
        
    pub fn definition() -> FunctionDefinition {
        FunctionDefinition {
            name: NAME.into(),
            definition: FunctionDefinitionType::InlineFn {
                return_type: RETURN_TYPE,
                parameters: PARAMS.into(),
                body: FN.into(),
            },
        }
    }
}

pub mod gerstner_waves {
    use webgl2_shader_definition::*;

    pub const NAME: &str = "gerstner_waves";
    pub const PARAMS: &str =
        "vec3 position, vec2 wave_axis, float wavelength, float time, float steepness";
    pub const FN: &str = r#"
float w_pos = dot(position, vec3(wave_axis.x, 0.0, wave_axis.y));
float k = 2.0 * 3.1416 / wavelength;
float f = k * (w_pos - time);
float a = steepness / k;
return vec2(a * cos(f), a * sin(f));
"#;
    pub const RETURN_TYPE: WebGLDataType = WebGLDataType::Vec2;
    pub fn definition() -> FunctionDefinition {
        FunctionDefinition {
            name: NAME.into(),
            definition: FunctionDefinitionType::InlineFn {
                return_type: RETURN_TYPE,
                parameters: PARAMS.into(),
                body: FN.into(),
            },
        }
    }
}
