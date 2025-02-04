use std::env::current_dir;

pub enum ShaderSource<'a> {
    Path(String),
    Str(&'a str),
}

pub(crate) struct PipelineBuilder<'a> {
    shader_source: ShaderSource<'a>,
    vert_main: String,
    frag_main: Option<String>,
    pixel_format: wgpu::TextureFormat,
    vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'a>>,
    bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,
    device: &'a wgpu::Device,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        Self {
            shader_source: ShaderSource::Path(String::new()),
            vert_main: String::new(),
            frag_main: None,
            pixel_format: wgpu::TextureFormat::Rgba8Unorm,
            vertex_buffer_layouts: Vec::new(),
            bind_group_layouts: Vec::new(),
            device: &device,
        }
    }

    fn reset(&mut self) {
        self.vertex_buffer_layouts.clear();
        self.bind_group_layouts.clear();
    }

    pub fn add_vertex_buffer_layout(&mut self, layout: wgpu::VertexBufferLayout<'a>) -> &mut Self {
        self.vertex_buffer_layouts.push(layout);
        self
    }

    pub fn add_vertex_buffer_layouts(
        &mut self,
        layouts: &[wgpu::VertexBufferLayout<'a>],
    ) -> &mut Self {
        for layout in layouts {
            self.vertex_buffer_layouts.push(layout.clone());
        }
        self
    }

    pub fn add_bind_group_layout(&mut self, layout: &'a wgpu::BindGroupLayout) -> &mut Self {
        self.bind_group_layouts.push(layout);
        self
    }

    pub fn add_bind_group_layouts(&mut self, layouts: &[&'a wgpu::BindGroupLayout]) -> &mut Self {
        for layout in layouts {
            self.bind_group_layouts.push(layout);
        }
        self
    }

    pub fn set_shader_module(
        &mut self,
        shader_source: ShaderSource<'a>,
        vert_main: &str,
        frag_main: Option<&str>,
    ) -> &mut Self {
        self.shader_source = shader_source;
        self.vert_main = vert_main.to_string();
        self.frag_main = match frag_main {
            Some(frag_main) => Some(frag_main.to_string()),
            None => None,
        };
        self
    }

    pub fn set_pixel_format(&mut self, format: wgpu::TextureFormat) -> &mut Self {
        self.pixel_format = format;
        self
    }

    async fn build_shader(&self) -> wgpu::ShaderModule {
        let source: String = match &self.shader_source {
            ShaderSource::Path(path) => {
                crate::utils::load_string(path).await.unwrap()
            }
            ShaderSource::Str(str) => str.to_string(),
        };
        let shader_module_des = wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        };
        self.device.create_shader_module(shader_module_des)
    }

    fn build_pipeline_layout(&self) -> wgpu::PipelineLayout {
        let render_pipeline_layout_desc = &wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &self.bind_group_layouts,
            push_constant_ranges: &[],
        };
        self.device
            .create_pipeline_layout(render_pipeline_layout_desc)
    }

    pub async fn build(&mut self) -> wgpu::RenderPipeline {
        let shader = self.build_shader().await;
        let render_pipeline_layout = self.build_pipeline_layout();

        let fs_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent::REPLACE,
                alpha: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        })];
        let pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),

                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some(&self.vert_main),
                    buffers: &self.vertex_buffer_layouts,
                    compilation_options: Default::default(),
                },

                fragment: match &self.frag_main {
                    Some(fs_main) => Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some(&fs_main),
                        targets: &fs_targets,
                        compilation_options: Default::default(),
                    }),
                    None => None,
                },

                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                    // or Features::POLYGON_MODE_POINT
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },

                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                // If the pipeline will be used with a multiview render pass, this
                // indicates how many array layers the attachments will have.
                multiview: None,
                // Useful for optimizing shader compilation on Android
                cache: None,
            });
        self.reset();
        pipeline
    }
}
