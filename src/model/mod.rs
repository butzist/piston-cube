use crate::pipeline::*;
use cgmath::prelude::One;
use cgmath::{Matrix3, Matrix4};
use gfx;
use gfx::traits::*;
use piston_window::*;
use std::fs::File;
use std::sync::{Arc, Mutex};

mod cube;
pub type Cube = cube::Cube;

mod mario;
pub type Mario = mario::Mario;

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
    pub fn new(
        pipeline: Arc<Mutex<ObjectPipeline>>,
        factory: &mut gfx_device_gl::Factory,
        vertices: gfx_core::handle::Buffer<gfx_device_gl::Resources, Vertex>,
        indices: &[u16],
        texture: gfx_core::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>,
    ) -> ObjectData {
        ObjectData {
            pipeline,
            vertices: vertices,
            indices: load_indices(factory, indices),
            texture: texture,
            matrix: Matrix4::one(),
            matrix_normal: Matrix3::one(),
        }
    }
}

pub fn download_cached(url: &str) -> Result<File, Box<dyn std::error::Error>> {
    let hash: String = url.chars().filter(|c| c.is_alphanumeric()).collect();
    let cache_dir = std::path::Path::new("./.cache/");

    if !cache_dir.exists() {
        std::fs::create_dir(cache_dir)?;
    }

    let cache_file = cache_dir.join(hash);
    if !cache_file.exists() {
        let mut file = File::create(&cache_file)?;
        let mut result = reqwest::get(url)?;
        std::io::copy(&mut result, &mut file)?;
    }

    Ok(File::open(&cache_file)?)
}

pub fn load_texture(
    factory: &mut gfx_device_gl::Factory,
    texture: &::image::DynamicImage,
) -> gfx_core::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]> {
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
    texture
}

pub fn load_indices<B: gfx::IntoIndexBuffer<gfx_device_gl::Resources>>(
    factory: &mut gfx_device_gl::Factory,
    indices: B,
) -> gfx::Slice<gfx_device_gl::Resources> {
    let index_buffer = factory.create_index_buffer(indices);
    let buffer_length = match index_buffer {
        gfx::IndexBuffer::Auto => panic!(),
        gfx::IndexBuffer::Index16(ref ib) => ib.len(),
        gfx::IndexBuffer::Index32(ref ib) => ib.len(),
    };

    gfx::Slice {
        start: 0,
        end: buffer_length as u32,
        base_vertex: 0,
        instances: None,
        buffer: index_buffer,
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
