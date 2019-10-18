use super::Vertex;
use camera_controllers::Camera;
use cgmath::prelude::*;
use piston_window::*;

pub struct Cube {
    model: super::ModelData,
    start_time: std::time::SystemTime,
}

impl Cube {
    pub fn new(
        factory: &mut gfx_device_gl::Factory,
        opengl: shader_version::opengl::OpenGL,
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
            model: super::ModelData::new(factory, opengl, &vertex_data, &index_data, &texture),
            start_time: std::time::SystemTime::now(),
        }
    }

    pub fn update(&mut self) {
        let t = self.start_time.elapsed().unwrap().as_millis() as f32 / 1000.0;
        let model_rotation =
            cgmath::Matrix3::from_axis_angle([1.0f32, 0.3, 0.3].into(), cgmath::Rad(t));
        self.model.matrix = cgmath::Matrix4::from(model_rotation);
        self.model.matrix_normal = model_rotation.invert().unwrap().transpose();
    }
}

impl super::Drawable for Cube {
    fn draw(&self, window: &mut PistonWindow, camera: &Camera<f32>) {
        self.model.draw(window, camera)
    }
}
