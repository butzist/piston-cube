use crate::pipeline::*;
use cgmath::prelude::One;
use cgmath::{Matrix3, Matrix4};
use gfx;
use gfx::traits::*;
use piston_window::*;
use std::sync::{Arc, Mutex};

mod cube;
pub type Cube = cube::Cube;

mod scene;
pub type Scene = scene::Scene;

#[derive(Clone)]
struct ObjectData {
    pipeline: Arc<Mutex<ObjectPipeline>>,
    vertices: gfx_core::handle::Buffer<gfx_device_gl::Resources, Vertex>,
    indices: gfx::Slice<gfx_device_gl::Resources>,
    texture: gfx_core::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>,
    matrix: cgmath::Matrix4<f32>,
    matrix_normal: cgmath::Matrix3<f32>,
}

impl ObjectData {
    fn new(
        pipeline: Arc<Mutex<ObjectPipeline>>,
        factory: &mut gfx_device_gl::Factory,
        vertices: &Vec<Vertex>,
        indices: &[u16],
        texture: &::image::DynamicImage,
    ) -> ObjectData {
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
        let rgba = texture.to_rgba();
        let image_dimensions = rgba.dimensions();
        let (_, texture) = factory
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

        ObjectData {
            pipeline,
            vertices: vbuf,
            indices: slice,
            texture: texture,
            matrix: Matrix4::one(),
            matrix_normal: Matrix3::one(),
        }
    }
}

pub trait Drawable {
    fn draw(&self, window: &mut PistonWindow);
}

impl<'a> ModelSlice<'a> for &'a ObjectData {
    fn indices(&'a self) -> &'a gfx::Slice<gfx_device_gl::Resources> {
        &self.indices
    }
}

impl<'a> ModelData<'a, <ObjectPipeline as PipelineData>::Data> for &'a ObjectData {
    fn fill(&'a self, data: &mut <ObjectPipeline as PipelineData>::Data) {
        data.vbuf = self.vertices.clone();
        data.u_model = self.matrix.into();
        data.u_model_norm = self.matrix_normal.into();
        data.t_color.0 = self.texture.clone();
    }
}

impl Drawable for ObjectData {
    fn draw(&self, window: &mut PistonWindow) {
        self.pipeline.lock().unwrap().draw(window, &self);
    }
}
