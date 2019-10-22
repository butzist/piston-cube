use gfx;
use piston_window::*;

mod object;
mod vertex;

pub type ObjectPipeline = object::ObjectPipeline;
pub type Vertex = vertex::Vertex;

pub struct Pipeline<D: gfx::pso::PipelineData<gfx_device_gl::Resources>> {
    pso: gfx::pso::PipelineState<gfx_device_gl::Resources, D::Meta>,
    data: D,
}

pub trait ModelData<'a, D: gfx::pso::PipelineData<gfx_device_gl::Resources>> {
    fn fill(&'a self, data: &mut D);
}

pub trait ModelSlice<'a> {
    fn indices(&'a self) -> &'a gfx::Slice<gfx_device_gl::Resources>;
}

pub trait PipelineDraw<D: gfx::pso::PipelineData<gfx_device_gl::Resources>> {
    type Data;
    fn draw<'a, S: ModelSlice<'a> + ModelData<'a, D>>(
        &mut self,
        window: &mut PistonWindow,
        data: &'a S,
    );

    fn fill<'a, S: ModelData<'a, D>>(&mut self, data: &'a S);
}

pub trait PipelineData {
    type Data;
}

impl<D: gfx::pso::PipelineData<gfx_device_gl::Resources>> PipelineDraw<D> for Pipeline<D> {
    type Data = D;
    fn draw<'a, S: ModelSlice<'a> + ModelData<'a, D>>(
        &mut self,
        window: &mut PistonWindow,
        data: &'a S,
    ) {
        self.fill(data);
        window.encoder.draw(data.indices(), &self.pso, &self.data);
    }

    fn fill<'a, S: ModelData<'a, D>>(&mut self, data: &'a S) {
        data.fill(&mut self.data);
    }
}

impl<D: gfx::pso::PipelineData<gfx_device_gl::Resources>> PipelineData for Pipeline<D> {
    type Data = D;
}
