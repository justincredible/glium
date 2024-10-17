#[macro_use]
extern crate glium;
use glium::Surface;

#[path = "../book/tuto-07-teapot.rs"]
mod teapot;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let mut app = Tutorials { tutorial: None };
    event_loop.run_app(&mut app).unwrap();
}

use glium::Display;
use glium::index::IndexBuffer;
use glium::program::Program;
use glium::vertex::VertexBuffer;
use glutin::surface::WindowSurface;
use winit::application::ApplicationHandler;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};
use winit::window::{Window, WindowId};

struct Tutorials {
    tutorial: Option<Tutorial>,
}

struct Tutorial {
    display: Display<WindowSurface>,
    program: Program,
    positions: VertexBuffer<teapot::Vertex>,
    normals: VertexBuffer<teapot::Normal>,
    indices: IndexBuffer<u16>,
    window: Window,
}

impl ApplicationHandler<()> for Tutorials {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.tutorial.is_none() {
            let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
                .with_title("Glium tutorial #7")
                .build(event_loop);

            let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
            let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
            let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                                &teapot::INDICES).unwrap();

            let vertex_shader_src = r#"
                #version 140

                in vec3 position;
                in vec3 normal;

                uniform mat4 matrix;

                void main() {
                    gl_Position = matrix * vec4(position, 1.0);
                }
            "#;

            let fragment_shader_src = r#"
                #version 140

                out vec4 color;

                void main() {
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "#;

            let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                                    None).unwrap();

           self.tutorial = Some(Tutorial {
                display,
                program,
                positions,
                normals,
                indices,
                window,
            });
       }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        // Now we wait until the program is closed
        match event {
            // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
            WindowEvent::CloseRequested => event_loop.exit(),
            // Because glium doesn't know about windows we need to resize the display
            // when the window's size has changed.
            WindowEvent::Resized(window_size) => {
                self.tutorial.as_ref().expect("Set during resumed").display.resize(window_size.into());
            },
            // We now need to render everything in response to a RedrawRequested event due to the animation
            WindowEvent::RedrawRequested => {
                if let Some(tutorial) = self.tutorial.as_mut() {
                    let mut target = tutorial.display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);

                    let matrix = [
                        [0.01, 0.0, 0.0, 0.0],
                        [0.0, 0.01, 0.0, 0.0],
                        [0.0, 0.0, 0.01, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32]
                    ];

                    target.draw((&tutorial.positions, &tutorial.normals), &tutorial.indices, &tutorial.program, &uniform! { matrix: matrix },
                                &Default::default()).unwrap();
                    target.finish().unwrap();
                }
            },
            _ => ()
        }
    }

    // By requesting a redraw in response to a RedrawEventsCleared event we get continuous rendering.
    // For applications that only change due to user input you could remove this handler.
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.tutorial.as_ref().expect("Set during resumed").window.request_redraw();
    }
}

