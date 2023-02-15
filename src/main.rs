use winit::event::*;
use winit::event_loop::EventLoop;
use winit::window::Window;

use wgpu::Adapter;
use wgpu::Backends;
use wgpu::Instance;
use wgpu::InstanceDescriptor;
use std::future::Future;
fn main() {
}

struct State {
    event_loop: EventLoop<()>,
    window: Window,
}

impl State {
    pub async fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).expect("Could not create a window");

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe {
            instance
                .create_surface(&window)
                .expect("Could not create surface")
        };

        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter(|adapter| {
                // Check if this adapter supports our surface
                adapter.is_surface_supported(&surface)
            })
            .next()
            .unwrap();

        let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
            None, // Trace path
        ).await.unwrap();

        Self {
            event_loop: event_loop,
            window: window,
        }
    }

    pub fn run(self) {
        let window = self.window;
        self.event_loop.run(move |root_event, _, control_flow| {
            control_flow.set_wait();
            match root_event {
                Event::MainEventsCleared => window.request_redraw(),
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => {}
                },
                Event::RedrawRequested(_) => {}
                _ => {}
            }
        });
    }
}
