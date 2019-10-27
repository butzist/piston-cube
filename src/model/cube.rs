use super::Vertex;
use cgmath::prelude::*;
use cgmath::{Matrix3, Matrix4, Vector3};
use gfx::traits::*;
use piston_window::*;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Cube {
    model: super::ObjectData,
    start_time: std::time::SystemTime,
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    time_offset: f32,
}

impl Cube {
    pub fn new(
        pipeline: Arc<Mutex<crate::pipeline::ObjectPipeline>>,
        factory: &mut gfx_device_gl::Factory,
    ) -> Cube {
        let vertex_data = vec![
            //top (0, 0, 1)
            Vertex::new([-1.0, -1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
            Vertex::new([1.0, -1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
            Vertex::new([1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
            Vertex::new([-1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
            //bottom (0, 0, -1)
            Vertex::new([1.0, 1.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0]),
            Vertex::new([-1.0, 1.0, -1.0], [0.0, 0.0, -1.0], [1.0, 0.0]),
            Vertex::new([-1.0, -1.0, -1.0], [0.0, 0.0, -1.0], [1.0, 1.0]),
            Vertex::new([1.0, -1.0, -1.0], [0.0, 0.0, -1.0], [0.0, 1.0]),
            //right (1, 0, 0)
            Vertex::new([1.0, -1.0, -1.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
            Vertex::new([1.0, 1.0, -1.0], [1.0, 0.0, 0.0], [1.0, 0.0]),
            Vertex::new([1.0, 1.0, 1.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
            Vertex::new([1.0, -1.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0]),
            //left (-1, 0, 0)
            Vertex::new([-1.0, 1.0, 1.0], [-1.0, 0.0, 0.0], [0.0, 0.0]),
            Vertex::new([-1.0, -1.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 0.0]),
            Vertex::new([-1.0, -1.0, -1.0], [-1.0, 0.0, 0.0], [1.0, 1.0]),
            Vertex::new([-1.0, 1.0, -1.0], [-1.0, 0.0, 0.0], [0.0, 1.0]),
            //front (0, 1, 0)
            Vertex::new([-1.0, 1.0, -1.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
            Vertex::new([1.0, 1.0, -1.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
            Vertex::new([1.0, 1.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
            Vertex::new([-1.0, 1.0, 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
            //back (0, -1, 0)
            Vertex::new([1.0, -1.0, 1.0], [0.0, -1.0, 0.0], [0.0, 0.0]),
            Vertex::new([-1.0, -1.0, 1.0], [0.0, -1.0, 0.0], [1.0, 0.0]),
            Vertex::new([-1.0, -1.0, -1.0], [0.0, -1.0, 0.0], [1.0, 1.0]),
            Vertex::new([1.0, -1.0, -1.0], [0.0, -1.0, 0.0], [0.0, 1.0]),
        ];

        let index_data = [
            0u16, 1, 2, 2, 3, 0, // top
            4, 6, 5, 6, 4, 7, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 14, 13, 14, 12, 15, // left
            16, 18, 17, 18, 16, 19, // front
            20, 21, 22, 22, 23, 20, // back
        ];

        let target = "https://i.imgur.com/40VzkBZ.jpg";
        let mut file = super::download_cached(target).unwrap();

        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();
        drop(file);

        let texture = ::image::load_from_memory(&buffer).unwrap();
        let texture_buffer = super::load_texture(factory, &texture);
        let vertex_buffer = factory.create_vertex_buffer(&vertex_data);

        Cube {
            model: super::ObjectData::new(
                pipeline,
                factory,
                vertex_buffer,
                &index_data,
                texture_buffer,
            ),
            start_time: std::time::SystemTime::now(),
            position: Vector3::zero(),
            velocity: Vector3::zero(),
            time_offset: 0.0,
        }
    }

    pub fn update(&mut self) {
        let t = self.start_time.elapsed().unwrap().as_millis() as f32 / 1000.0;
        let position = t * self.velocity + self.position;

        let model_rotation =
            Matrix3::from_axis_angle([1.0f32, 0.3, 0.3].into(), cgmath::Rad(t + self.time_offset));
        self.model.matrix = Matrix4::from_translation(position)
            * Matrix4::from_scale(0.1)
            * Matrix4::from(model_rotation);
        self.model.matrix_normal = model_rotation.invert().unwrap().transpose();
    }
}

impl super::Drawable for Cube {
    fn draw(&self, window: &mut PistonWindow) {
        self.model.draw(window)
    }
}
