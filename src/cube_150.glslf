#version 150 core
in vec2 v_TexCoord;
in vec3 v_Pos;
in vec3 v_Norm;
out vec4 o_Color;
uniform sampler2D t_color;
uniform vec3 u_camera;
uniform vec3 u_light;

const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
    float diffuse = max(dot(normalize(v_Norm), normalize(u_light)), 0.0);

    vec3 camera_dir = normalize(u_camera - v_Pos);
    vec3 half_direction = normalize(normalize(u_light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_Norm)), 0.0), 16.0);
    
    vec3 tex = vec3(texture(t_color, v_TexCoord));
    vec3 ambient_color = 0.2 * tex;
    vec3 diffuse_color = tex;

    o_Color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}