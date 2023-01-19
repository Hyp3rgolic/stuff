use wgpu::util::DeviceExt;

use crate::vertex::Vertex;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_hull: Vec<Vertex>,
    camera: crate::camera::Camera,
    input: crate::input::Input,
}

impl State {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);
        let camera = crate::camera::Camera::new(&config, &device);
        let render_pipeline = crate::pipeline::create_render_pipeline(&device, &config, &[&camera.camera_bind_group_layout]);
        let mut vertex_hull = crate::vertex::Vertex::create_hull();

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 4096,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera = crate::camera::Camera::new(&config, &device);

        let input = crate::input::Input::init();

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            vertex_hull,
            camera,
            input,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn handle_input(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.input.handle(event)
    }

    pub fn update(&mut self) {
        if self.input.space {
            //self.vertex_hull.remove(2);
            self.vertex_hull.push(Vertex { p: [2.0, 0.6, 1.0] });
            self.queue.write_buffer(
                &self.vertex_buffer,
                0,
                bytemuck::cast_slice(self.vertex_hull.as_slice()),
            )
        }
        crate::camera::Camera::update_camera(&self.input, &mut self.camera.camera_fow, 0.2);
        self.camera.camera_uniform.update_view_proj(&self.camera.camera_fow);

        self.queue.write_buffer(&self.camera.camera_buffer, 0, bytemuck::cast_slice(&[self.camera.camera_uniform]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.02,
                            g: 0.02,
                            b: 0.02,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..(self.vertex_hull.len() as u32), 0..1)
        }

        // submit will accept anything that implements IntoIter

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
