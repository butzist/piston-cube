use super::Vertex;
use cgmath::prelude::*;
use cgmath::{Matrix3, Matrix4};
use piston_window::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Cube {
    model: super::ObjectData,
    start_time: std::time::SystemTime,
    position: [f32; 3],
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
        let mut buffer: Vec<u8> = vec![];
        reqwest::get(target).unwrap().copy_to(&mut buffer).unwrap();

        let texture = ::image::load_from_memory(&buffer).unwrap();

        Cube {
            model: super::ObjectData::new(pipeline, factory, &vertex_data, &index_data, &texture),
            start_time: std::time::SystemTime::now(),
            position: [0.0, 0.0, 0.0],
        }
    }

    pub fn update(&mut self) {
        let t = self.start_time.elapsed().unwrap().as_millis() as f32 / 1000.0;
        let model_rotation = Matrix3::from_axis_angle([1.0f32, 0.3, 0.3].into(), cgmath::Rad(t));
        self.model.matrix = Matrix4::from_translation(self.position.into())
            * Matrix4::from_scale(0.1)
            * Matrix4::from(model_rotation);
        self.model.matrix_normal = model_rotation.invert().unwrap().transpose();
    }

    pub fn clone_to(&self, position: [f32; 3]) -> Cube {
        let mut new_cube = self.clone();
        new_cube.position = position;
        new_cube
    }
}

impl super::Drawable for Cube {
    fn draw(&self, window: &mut PistonWindow) {
        self.model.draw(window)
    }
}
