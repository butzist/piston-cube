use camera_controllers::{model_view_projection, Camera, CameraPerspective};
use cgmath::prelude::One;
use gfx;
use gfx::traits::*;
use gfx::{gfx_impl_struct_meta, gfx_pipeline_inner, gfx_vertex_struct, gfx_vertex_struct_meta};
use piston_window::*;
use shader_version::glsl::GLSL;
use shader_version::Shaders;

mod cube;
pub type Cube = cube::Cube;

gfx_vertex_struct!(Vertex {
    a_pos: [f32; 4] = "a_pos",
    a_norm: [f32; 4] = "a_norm",
    a_tex_coord: [f32; 2] = "a_tex_coord",
});

gfx::gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    u_model: gfx::Global<[[f32; 4]; 4]> = "u_model",
    u_model_norm: gfx::Global<[[f32; 3]; 3]> = "u_model_norm",
    u_camera: gfx::Global<[f32; 3]> = "u_camera",
    u_light: gfx::Global<[f32; 3]> = "u_light",
    t_color: gfx::TextureSampler<[f32; 4]> = "t_color",
    out_color: gfx::RenderTarget<::gfx::format::Srgba8> = "o_Color",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

impl Vertex {
    fn new(pos: [f32; 3], norm: [f32; 3], tc: [f32; 2]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1.0],
            a_norm: [norm[0], norm[1], norm[2], 1.0],
            a_tex_coord: tc,
        }
    }
}

#[derive(Clone)]
pub struct ModelData {
    vertices: gfx_core::handle::Buffer<gfx_device_gl::Resources, Vertex>,
    indices: gfx::Slice<gfx_device_gl::Resources>,
    texture: gfx_core::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>,
    sampler: gfx_core::handle::Sampler<gfx_device_gl::Resources>,
    matrix: cgmath::Matrix4<f32>,
    matrix_normal: cgmath::Matrix3<f32>,
    pipeline: gfx::pso::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
}

impl ModelData {
    fn new(
        factory: &mut gfx_device_gl::Factory,
        opengl: shader_version::opengl::OpenGL,
        vertices: &Vec<Vertex>,
        indices: &[u16],
        texture: &::image::DynamicImage,
    ) -> ModelData {
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
        let rgba = texture.to_rgba();
        let image_dimensions = rgba.dimensions();
        let (_, texture_view) = factory
            .create_texture_immutable_u8::<gfx::format::Rgba8>(
                gfx::texture::Kind::D2(
                    image_dimensions.0 as u16,
                    image_dimensions.1 as u16,
                    gfx::texture::AaMode::Single,
                ),
                gfx::texture::Mipmap::Provided,
                &[&rgba],
            )
            .unwrap();

        let sinfo = gfx::texture::SamplerInfo::new(
            gfx::texture::FilterMethod::Bilinear,
            gfx::texture::WrapMode::Clamp,
        );

        let glsl = opengl.to_glsl();

        let pso = {
            let program = factory
                .link_program(
                    Shaders::new()
                        .set(GLSL::V1_50, include_str!("cube_150.glslv"))
                        .get(glsl)
                        .unwrap()
                        .as_bytes(),
                    Shaders::new()
                        .set(GLSL::V1_50, include_str!("cube_150.glslf"))
                        .get(glsl)
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();

            let rasterizer = gfx::state::Rasterizer::new_fill().with_cull_back();
            factory
                .create_pipeline_from_program(
                    &program,
                    gfx::Primitive::TriangleList,
                    rasterizer,
                    pipe::new(),
                )
                .unwrap()
        };

        ModelData {
            vertices: vbuf,
            indices: slice,
            texture: texture_view,
            sampler: factory.create_sampler(sinfo),
            matrix: cgmath::Matrix4::one(),
            matrix_normal: cgmath::Matrix3::one(),
            pipeline: pso,
        }
    }
}

pub trait Drawable {
    fn draw(&self, window: &mut PistonWindow, camera: &Camera<f32>);
}

impl Drawable for ModelData {
    fn draw(&self, window: &mut PistonWindow, camera: &camera_controllers::Camera<f32>) {
        let get_projection = |w: &PistonWindow| {
            let draw_size = w.window.draw_size();
            CameraPerspective {
                fov: 90.0,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32),
            }
            .projection()
        };
        let data = pipe::Data {
            vbuf: self.vertices.clone(),
            u_model_view_proj: model_view_projection(
                cgmath::Matrix4::from_scale(1.0f32).into(),
                camera.orthogonal(),
                get_projection(window),
            ),
            u_model: self.matrix.into(),
            u_model_norm: self.matrix_normal.into(),
            u_camera: camera.position,
            u_light: [0.0, 0.0, 2.0],
            t_color: (self.texture.clone(), self.sampler.clone()),
            out_color: window.output_color.clone(),
            out_depth: window.output_stencil.clone(),
        };

        window.encoder.draw(&self.indices, &self.pipeline, &data);
    }
}
