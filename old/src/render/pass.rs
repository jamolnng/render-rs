use crate::render::Context;

pub trait Pass {
    fn draw(&mut self, state: &Context) -> Result<(), wgpu::SurfaceError>;
}
