#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub p: [f32; 3],
}

impl Vertex {
    pub fn create_hull() -> Vec<Vertex> {
        vec![
            Vertex { p: [0.0, 0.5, 0.0] },
            Vertex {
                p: [-0.5, -0.5, 0.0],
            },
            Vertex {
                p: [0.5, -0.5, 0.0],
            },
        ]
    }
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}
