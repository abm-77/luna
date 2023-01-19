use std::cell::RefCell;
use std::rc::Rc;

use winit::{
    event::VirtualKeyCode,
    event_loop::EventLoop,
};

use crate::audio::audio_subsystem::AudioSubsystem;
use crate::gfx::graphics_subsystem::GraphicsSubsystem;
use crate::gfx::renderer2d::*;
use crate::sys::input_subsystem::InputSubsystem;
use crate::sys::resource_manager::ResourceManager;
use crate::window::window_subsystem::{WindowConfig, WindowSubsystem};

pub struct Context {
    pub res: Rc<RefCell<ResourceManager>>,
    pub r2d: Renderer2D,
    pub input: InputSubsystem,
    pub audio: AudioSubsystem,
}

pub trait LunarApp {
    fn setup(&mut self, ctx: &mut Context);
    fn update(&mut self, ctx: &mut Context);
    fn shutdown(&mut self, ctx: &mut Context);
}

pub async fn run<T: LunarApp + 'static>(mut client: T) {
    // Initialize subsystems
    env_logger::init();
    let event_loop = EventLoop::new();
    let input = InputSubsystem::new();

    let window_sys = WindowSubsystem::new(WindowConfig::default(), &event_loop);

    // Subsyststems requiring resources
    let gfx = Rc::new(RefCell::new(GraphicsSubsystem::new(&window_sys).await));
    let res = Rc::new(RefCell::new(ResourceManager::new(gfx.clone())));

    // Now that all subsystems requiring resources to be loaded are initialized
    // load all program resources

    // Initialize subsystems that depend on loaded resources
    let r2d = Renderer2D::init(gfx.clone(), res.clone());
    let audio = AudioSubsystem::new(res.clone());

    let mut ctx = Context {
        // TODO(bryson): Do we actually need gfx here?
        res,
        r2d,
        input,
        audio,
    };

    // Run app setup
    client.setup(&mut ctx);

    event_loop.run(move |event, _, control_flow| {
       if ctx.input.update(&event) {
           if ctx.input.key_pressed(VirtualKeyCode::Escape) || ctx.input.close_requested() {
               client.shutdown(&mut ctx);
               control_flow.set_exit();
               return;
           }

           if let Some(new_size) = ctx.input.window_resized() {
               ctx.r2d.resize(new_size);
           }

           window_sys.window.request_redraw();

           client.update(&mut ctx);

           match ctx.r2d.render() {
               Ok(_) => {}
               Err(wgpu::SurfaceError::Lost) => ctx.r2d.resize(window_sys.window.inner_size()),
               Err(wgpu::SurfaceError::OutOfMemory) => {
                   client.shutdown(&mut ctx);
                   control_flow.set_exit()
               },
               Err(e) => eprintln!("{:?}", e),
           }
       }
    });
}