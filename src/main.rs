use winit::event::*;
use winit::event_loop::EventLoop;
use winit::window::Window;

use wgpu::Backends;
use wgpu::Instance;
use wgpu::InstanceDescriptor;

#[tokio::main]
async fn main() {
    State::new().await.run();
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    event_loop: EventLoop<()>
}

impl State {
    pub async fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).expect("Could not create a window");

        let size = window.inner_size();

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
                    limits: wgpu::Limits::default(),
                    label: None,
                },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        Self {
            event_loop: event_loop,
            window: window,
            surface: surface,
            queue: queue,
            device: device,
            config: config,
            size: size
        }
    }

    pub fn run(mut self) {
        let window = self.window;
        self.event_loop.run(move |root_event, _, control_flow| {
            control_flow.set_wait();
            match root_event {
                Event::MainEventsCleared => window.request_redraw(),
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::Resized(new_size) => self.resize(new_size),
                    _ => {}
                },
                Event::RedrawRequested(_) => {}
                _ => {}
            }
        });
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
