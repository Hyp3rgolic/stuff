pub fn dosmt(device: &wgpu::Device) {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Test Buffer"),
        size: 1024,
        usage: wgpu::BufferUsages::all(),
        mapped_at_creation: false,
    });
}
