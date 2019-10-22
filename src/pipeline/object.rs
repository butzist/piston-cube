use cgmath::prelude::One;
use cgmath::{Matrix3, Matrix4};
use gfx;
use gfx::gfx_pipeline_inner;
use gfx::traits::*;
use piston_window::*;
use shader_version::glsl::GLSL;
use shader_version::Shaders;

use super::vertex::Vertex;

gfx::gfx_pipeline!( object_pipe {
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

pub type ObjectPipeline = super::Pipeline<object_pipe::Data<gfx_device_gl::Resources>>;

impl ObjectPipeline {
    pub fn new(window: &PistonWindow, opengl: shader_version::opengl::OpenGL) -> ObjectPipeline {
        let ref mut factory = window.factory.clone();
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
                    object_pipe::new(),
                )
                .unwrap()
        };

        let vbuf = factory.create_vertex_buffer(&[]);
        let (_, texture) = factory
            .create_texture_immutable_u8::<gfx::format::Rgba8>(
                gfx::texture::Kind::D2(0, 0, gfx::texture::AaMode::Single),
                gfx::texture::Mipmap::Provided,
                &[&[]],
            )
            .unwrap();

        let sampler = {
            let sinfo = gfx::texture::SamplerInfo::new(
                gfx::texture::FilterMethod::Bilinear,
                gfx::texture::WrapMode::Clamp,
            );
            factory.create_sampler(sinfo)
        };
        let data = object_pipe::Data {
            vbuf,
            u_model_view_proj: Matrix4::one().into(),
            u_model: Matrix4::one().into(),
            u_model_norm: Matrix3::one().into(),
            u_camera: [0.0, 0.0, 0.0],
            u_light: [0.0, 0.0, 0.0],
            t_color: (texture, sampler.clone()),
            out_color: window.output_color.clone(),
            out_depth: window.output_stencil.clone(),
        };

        ObjectPipeline { pso, data, sampler }
    }
}
