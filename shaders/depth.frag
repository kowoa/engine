#version 330 core

struct Material {
    sampler2D diffuse_tex1;
    sampler2D specular_tex1;
    float shininess;
};

struct DirLight {
    vec3 direction; // should be in view space

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position; // should be in view space
    
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    // attenuation
    float att_constant;
    float att_linear;
    float att_quadratic;
};

struct SpotLight {
    vec3 position; // should be in view space
    vec3 direction; // should be in view space
    float cutoff_angle_cos; // should be the cosine of an angle
    float outer_cutoff_angle_cos; // should be the cosine of an angle
    
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    
    // attenuation
    float att_constant;
    float att_linear;
    float att_quadratic;
};

uniform Material material;
uniform float time;

// ---------- lights ---------- //
uniform DirLight dir_light;

#define NR_POINT_LIGHTS 4
uniform PointLight point_lights[NR_POINT_LIGHTS];

uniform SpotLight spot_light;
// ---------------------------- //

in vec3 f_pos;
in vec3 f_normal;
in vec2 f_tex_coords;

out vec4 color;

vec3 calc_dir_light(DirLight light, vec3 normal, vec3 frag_to_cam);
vec3 calc_point_light(PointLight light, vec3 normal, vec3 frag_pos, vec3 frag_to_cam);
vec3 calc_spot_light(SpotLight light, vec3 normal, vec3 frag_pos, vec3 frag_to_cam);

float near = 0.1;
float far = 100.0;

float linearize_depth(float depth) {
    float ndc = depth * 2.0 - 1.0;
    return (2.0 * near * far) / (far + near - ndc * (far - near));
}

void main() {
    /*
    // light properties
    vec3 normal = normalize(f_normal);
    vec3 frag_to_cam = normalize(-f_pos);
    
    // directional light
    vec3 result = calc_dir_light(dir_light, normal, frag_to_cam);
    
    // point lights
    for (int i = 0; i < NR_POINT_LIGHTS; i++) {
        result += calc_point_light(point_lights[i], normal, f_pos, frag_to_cam);
    }
    
    // spot light
    result += calc_spot_light(spot_light, normal, f_pos, frag_to_cam);
    */
    
    float depth = linearize_depth(gl_FragCoord.z) / far;
    color = vec4(vec3(depth), 1.0);
}

vec3 calc_dir_light(DirLight light, vec3 normal, vec3 frag_to_cam) {
    vec3 light_dir = normalize(light.direction);

    // diffuse shading
    float diff_strength = max(dot(normal, -light_dir), 0.0);

    // specular shading
    vec3 reflect_dir = reflect(light_dir, normal);
    float spec_strength = pow(max(dot(frag_to_cam, reflect_dir), 0.0), material.shininess);

    // combine results
    vec3 ambient = light.ambient * vec3(texture(material.diffuse_tex1, f_tex_coords));
    vec3 diffuse = light.diffuse * diff_strength * vec3(texture(material.diffuse_tex1, f_tex_coords));
    vec3 specular = light.specular * spec_strength * vec3(texture(material.specular_tex1, f_tex_coords));

    return ambient + diffuse + specular;
}

vec3 calc_point_light(PointLight light, vec3 normal, vec3 frag_pos, vec3 frag_to_cam) {
    vec3 frag_to_light = normalize(light.position - frag_pos);
    
    // diffuse shading
    float diff_strength = max(dot(normal, frag_to_light), 0.0);
    
    // specular shading
    vec3 reflect_dir = reflect(-frag_to_light, normal);
    float spec_strength = pow(max(dot(frag_to_cam, reflect_dir), 0.0), material.shininess);
    
    // attenuation
    float dist = distance(light.position, frag_pos);
    float attenuation = 1.0 / (
        light.att_constant
        + light.att_linear * dist
        + light.att_quadratic * dist * dist
    );
    
    // combine results
    vec3 ambient = light.ambient * vec3(texture(material.diffuse_tex1, f_tex_coords)) * attenuation;
    vec3 diffuse = light.diffuse * diff_strength * vec3(texture(material.diffuse_tex1, f_tex_coords)) * attenuation;
    vec3 specular = light.specular * spec_strength * vec3(texture(material.specular_tex1, f_tex_coords)) * attenuation;
    
    return ambient + diffuse + specular;
}

vec3 calc_spot_light(SpotLight light, vec3 normal, vec3 frag_pos, vec3 frag) {
    // ambient lighting
    vec3 ambient = light.ambient * vec3(
        texture(material.diffuse_tex1, f_tex_coords)
    );

    // diffuse lighting
    vec3 frag_to_light = normalize(light.position - frag_pos);
    float diff_strength = max(dot(normal, frag_to_light), 0.0);
    vec3 diffuse = light.diffuse * diff_strength * vec3(
        texture(material.diffuse_tex1, f_tex_coords)
    );

    // specular lighting
    vec3 frag_to_cam = normalize(-f_pos);
    vec3 reflect_dir = reflect(-frag_to_light, normal);
    float spec_strength = pow(max(dot(frag_to_cam, reflect_dir), 0.0), material.shininess);
    vec3 specular = light.specular * spec_strength * vec3(
        texture(material.specular_tex1, f_tex_coords)
    );
    
    // attenuation (decrease intensity of light over distance)
    float dist = distance(light.position, frag_pos);
    float attenuation = 1.0 / (
        light.att_constant
        + light.att_linear * dist
        + light.att_quadratic * dist * dist
    );

    // apply cutoff (for spotlight)
    float theta_cos = dot(frag_to_light, normalize(-light.direction));
    float epsilon = light.cutoff_angle_cos - light.outer_cutoff_angle_cos;
    float intensity = clamp((theta_cos - light.outer_cutoff_angle_cos) / epsilon, 0.0, 1.0);
    
    ambient *= attenuation;
    diffuse *= attenuation * intensity;
    specular *= attenuation * intensity;
    
    return ambient + diffuse + specular;
}