#version 330 core

layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec3 v_normal;
layout (location = 2) in vec2 v_tex_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;
uniform mat3 normal_mat;

out vec3 f_pos;
out vec3 f_normal;
out vec2 f_tex_coords;

void main() {
    gl_Position = proj * view * model * vec4(v_pos, 1.0);
    // fragment position should be in view space
    f_pos = vec3(view * model * vec4(v_pos, 1.0));
    // fragment normal should be in view space
    f_normal = normal_mat * v_normal;
    // pass along tex coords
    f_tex_coords = v_tex_coords;
}