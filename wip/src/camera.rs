use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;

pub struct Camera {
    pub camera_fow: CameraFOW,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub camera_bind_group: wgpu::BindGroup,

}

impl Camera {
    pub fn new(config: &wgpu::SurfaceConfiguration, device: &wgpu::Device) -> Self {
        let camera_uniform = CameraUniform {
            view_proj: cgmath::Matrix4::identity().into(),
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        Self {
            camera_fow: CameraFOW {
                // position the camera one unit up and 2 units back
                // +z is out of the screen
                eye: (0.0, 1.0, 2.0).into(),
                // have it look at the origin
                target: (0.0, 0.0, 0.0).into(),
                // which way is "up"
                up: cgmath::Vector3::unit_y(),
                aspect: config.width as f32 / config.height as f32,
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
            },
            camera_uniform,

            camera_bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            }),
            camera_buffer: camera_buffer,
            camera_bind_group_layout: camera_bind_group_layout,

        }
    }

    pub fn update_camera(input: &crate::input::Input, camera: &mut CameraFOW, speed: f32) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        if input.w && forward_mag > speed {
            camera.eye += forward_norm * speed;
        }
        if input.s {
            camera.eye -= forward_norm * speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the fowrard/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if input.d {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * speed).normalize() * forward_mag;
        }
        if input.a {
            camera.eye = camera.target - (forward - right * speed).normalize() * forward_mag;
        }
    }

}

pub struct CameraFOW {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl CameraFOW {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn update_view_proj(&mut self, camera: &CameraFOW) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}


