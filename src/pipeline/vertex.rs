use gfx;
use gfx::{gfx_impl_struct_meta, gfx_vertex_struct, gfx_vertex_struct_meta};

gfx_vertex_struct!(Vertex {
    a_pos: [f32; 4] = "a_pos",
    a_norm: [f32; 4] = "a_norm",
    a_tex_coord: [f32; 2] = "a_tex_coord",
});

impl Vertex {
    pub fn new(pos: [f32; 3], norm: [f32; 3], tc: [f32; 2]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1.0],
            a_norm: [norm[0], norm[1], norm[2], 1.0],
            a_tex_coord: tc,
        }
    }
}
