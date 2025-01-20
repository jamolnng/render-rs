use std::str::from_utf8;

use wgpu::{ShaderModuleDescriptor, ShaderSource};

pub(crate) struct Shader {}

impl Shader {
    pub fn new(device: &wgpu::Device, path: &str) -> Self {
        // let data: String = std::fs::read_to_string(path).unwrap();
        // let desc = ShaderModuleDescriptor {
        //     label: Some(path),
        //     source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(data.as_str())),
        // };
        // let shader = device.create_shader_module(desc);
        Self {}
    }
}
