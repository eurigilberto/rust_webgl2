use webgl2_shader_definition::*;

pub fn get_pbr_functions() -> Vec<FunctionDefinition> {
    // GLSL functions taken from https://learnopengl.com/PBR/Lighting
    
    let mut functions = Vec::new();

    let fresnel_schlick = FunctionDefinition {
        name: "fresnelSchlick".into(),
        definition: FunctionDefinitionType::InlineFn {
            return_type: WebGLDataType::Vec3,
            parameters: "float cosTheta, vec3 F0".into(),
            body: r#"return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);"#.into(),
        },
    };

    functions.push(fresnel_schlick);

    let distribution_ggx = FunctionDefinition {
        name: "DistributionGGX".into(),
        definition: FunctionDefinitionType::InlineFn {
            return_type: WebGLDataType::Float,
            parameters: "vec3 N, vec3 H, float _roughness".into(),
            body: r#"
float a      = _roughness*_roughness;
float a2     = a*a;
float NdotH  = max(dot(N, H), 0.0);
float NdotH2 = NdotH*NdotH;

float num   = a2;
float denom = (NdotH2 * (a2 - 1.0) + 1.0);
denom = 3.141592 * denom * denom;

return num / denom;"#
                .into(),
        },
    };

    functions.push(distribution_ggx);

    let geometry_schlick_ggx = FunctionDefinition {
        name: "GeometrySchlickGGX".into(),
        definition: FunctionDefinitionType::InlineFn {
            return_type: WebGLDataType::Float,
            parameters: "float NdotV, float _roughness".into(),
            body: r#"
float r = (_roughness + 1.0);
float k = (r*r) / 8.0;

float num   = NdotV;
float denom = NdotV * (1.0 - k) + k;

return num / denom;
			"#
            .into(),
        },
    };

    functions.push(geometry_schlick_ggx);

    let geometry_smith = FunctionDefinition {
        name: "GeometrySmith".into(),
        definition: FunctionDefinitionType::InlineFn {
            return_type: WebGLDataType::Float,
            parameters: "vec3 N, vec3 V, vec3 L, float _roughness".into(),
            body: r#"
float NdotV = max(dot(N, V), 0.0);
float NdotL = max(dot(N, L), 0.0);
float ggx2  = GeometrySchlickGGX(NdotV, _roughness);
float ggx1  = GeometrySchlickGGX(NdotL, _roughness);

return ggx1 * ggx2;"#
                .into(),
        },
    };

    functions.push(geometry_smith);

    let light_function = FunctionDefinition {
        name: "PBRLighting".into(),
        definition: FunctionDefinitionType::InlineFn {
            return_type: WebGLDataType::Vec3,
            parameters: "vec3 normal, vec3 cam_pos, vec3 world_pos, vec3 w_pos_to_light, vec3 _albedo, float _metallic, float _roughness, vec3 _radiance".into(),
            // cook-torrance brdf
            // add to outgoing radiance Lo
            body: r#"
vec3 N = normal;
vec3 V = normalize(cam_pos - world_pos);

vec3 F0 = vec3(0.04); 
F0 = mix(F0, _albedo, _metallic);

vec3 L = normalize(w_pos_to_light);
vec3 H = normalize(V + L);      

float NDF = DistributionGGX(N, H, _roughness);        
float G   = GeometrySmith(N, V, L, _roughness);      
vec3 F    = fresnelSchlick(max(dot(H, V), 0.0), F0);       

vec3 kS = F;
vec3 kD = vec3(1.0) - kS;
kD *= 1.0 - _metallic;	  

vec3 numerator    = NDF * G * F;
float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001;
vec3 specular     = numerator / denominator;  

float NdotL = max(dot(N, L), 0.0);                
vec3 Lo = (kD * _albedo / 3.141592 + specular) * _radiance * NdotL;
return Lo;"#
                .into(),
        },
    };

    functions.push(light_function);

    functions
}
