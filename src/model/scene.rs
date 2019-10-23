use super::Drawable;
use crate::pipeline::*;
use camera_controllers::{model_view_projection, CameraPerspective};
use cgmath::prelude::One;
use cgmath::Matrix4;
use gfx;
use gfx::traits::*;
use piston_window::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Scene {
    pipeline: Arc<Mutex<ObjectPipeline>>,
    indices: gfx::Slice<gfx_device_gl::Resources>,
    mvp: [[f32; 4]; 4],
    position: [f32; 3],
}

impl Scene {
    pub fn new(
        pipeline: Arc<Mutex<ObjectPipeline>>,
        factory: &mut gfx_device_gl::Factory,
    ) -> Scene {
        let (_, indices) =
            factory.create_vertex_buffer_with_slice(&vec![] as &Vec<Vertex>, &[] as &[u16]);
        Scene {
            pipeline,
            indices,
            mvp: Matrix4::one().into(),
            position: [0.0, 0.0, 0.0],
        }
    }
}

impl Scene {
    pub fn update(&mut self, window: &PistonWindow, camera: &camera_controllers::Camera<f32>) {
        self.mvp = mvp(window, camera);
        self.position = camera.position.clone();
    }
}

impl<'a> ModelSlice<'a> for &'a Scene {
    fn indices(&'a self) -> &'a gfx::Slice<gfx_device_gl::Resources> {
        &self.indices // TODO: draw skybox here
    }
}

impl<'a> ModelData<'a, <ObjectPipeline as PipelineData>::Data> for &'a Scene {
    fn fill(&'a self, data: &mut <ObjectPipeline as PipelineData>::Data) {
        data.u_model_view_proj = self.mvp.clone();
        data.u_camera = self.position.clone();
        data.u_light = [2.0, 0.0, 2.0];
    }
}

fn mvp(window: &PistonWindow, camera: &camera_controllers::Camera<f32>) -> [[f32; 4]; 4] {
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

    model_view_projection(
        cgmath::Matrix4::one().into(),
        camera.orthogonal(),
        get_projection(window),
    )
}

impl Drawable for Scene {
    fn draw(&self, window: &mut PistonWindow) {
        self.pipeline.lock().unwrap().draw(window, &self);
    }
}
