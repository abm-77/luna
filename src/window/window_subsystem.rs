use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{
    Window, WindowBuilder
};

pub struct WindowSubsystem {
    pub window: Window,
}

pub struct WindowConfig {
    width: u32,
    height: u32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}

impl WindowSubsystem {
    pub fn new (config: WindowConfig, event_loop: &EventLoop<()>) -> Self {
        let w = WindowBuilder::new()
            .with_inner_size(LogicalSize {width: config.width, height: config.height})
            .build(&event_loop).unwrap();

        Self {
            window: w
        }
    }
}