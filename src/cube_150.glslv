#version 150 core
in vec3 a_pos;
in vec3 a_norm;
in vec2 a_tex_coord;
out vec2 v_TexCoord;
out vec3 v_Pos;
out vec3 v_Norm;
uniform mat4 u_model_view_proj;
uniform mat4 u_model;
uniform mat3 u_model_norm;

void main() {
    v_TexCoord = a_tex_coord;
    v_Pos = vec3(u_model * vec4(a_pos, 1.0));
    v_Norm = u_model_norm * a_norm;

    gl_Position = u_model_view_proj * u_model * vec4(a_pos, 1.0);
}