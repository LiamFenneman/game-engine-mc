use ge_resource::ResourceManager;

fn main() {
    std::env::set_var(
        "ASSET_DIR",
        "/home/liam/Documents/Projects/game_engine/assets/",
    );

    let (device, queue) = beul::execute(mock_renderer());

    let mut rm = ResourceManager::default();
    let textures = rm.load_texture_array("grass", &device, &queue);

    dbg!(textures);
}

/// Mock "renderer" for testing.
async fn mock_renderer() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::TEXTURE_BINDING_ARRAY,
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    (device, queue)
}
