#[macro_use]
extern crate glium;
use glium::Surface;

fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let mut app = Tutorial02 { window_display: None };
    event_loop.run_app(&mut app).unwrap();
}

use glium::Display;
use glutin::surface::WindowSurface;
use winit::application::ApplicationHandler;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};
use winit::window::{Window, WindowId};

struct Tutorial02 {
    window_display: Option<(Window, Display<WindowSurface>)>
}

impl ApplicationHandler<()> for Tutorial02 {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_display.is_none() {
            let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
                .with_title("Glium tutorial #2")
                .build(event_loop);

            #[derive(Copy, Clone)]
            struct Vertex {
                position: [f32; 2],
            }
            implement_vertex!(Vertex, position);
            let shape = vec![
                Vertex { position: [-0.5, -0.5] },
                Vertex { position: [ 0.0,  0.5] },
                Vertex { position: [ 0.5, -0.25] }
            ];
            let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

            let vertex_shader_src = r#"
                #version 140

                in vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            "#;

            let fragment_shader_src = r#"
                #version 140

                out vec4 color;

                void main() {
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "#;

            let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();
            target.finish().unwrap();

            self.window_display = Some((window, display));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        // Now we wait until the program is closed
        match event {
            // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => ()
        }
    }
}

