use crate::window::window_subsystem::WindowSubsystem;

pub struct GraphicsSubsystem {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub surface_format: wgpu::TextureFormat,
    pub config: wgpu::SurfaceConfiguration,
}

impl GraphicsSubsystem {
    pub async fn new (window_sys: &WindowSubsystem) -> Self {
        let size = window_sys.window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(&window_sys.window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let surface_format = surface.get_supported_formats(&adapter)[0];

        let desired_features =
            wgpu::Features::TEXTURE_BINDING_ARRAY |
                wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: desired_features,
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        Self {
            device,
            queue,
            surface,
            surface_format,
            config,
        }
    }
}