#[macro_use]
extern crate glium;
use glium::Surface;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let mut app = Tutorials { tutorial: None };
    event_loop.run_app(&mut app).unwrap();
}

use glium::Display;
use glium::index::NoIndices;
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
    vertex_buffer: VertexBuffer<Vertex>,
    indices: NoIndices,
    window: Window,
    t: f32,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

impl ApplicationHandler<()> for Tutorials {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.tutorial.is_none() {
            let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
                .with_title("Glium tutorial #4")
                .build(event_loop);

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

                uniform mat4 matrix;

                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
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

            self.tutorial = Some(Tutorial {
                display,
                program,
                vertex_buffer,
                indices,
                window,
                t: 0.0,
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
                    // we update `t`
                    tutorial.t += 0.02;
                    let x = tutorial.t.sin() * 0.5;

                    let mut target = tutorial.display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);

                    let uniforms = uniform! {
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [  x, 0.0, 0.0, 1.0f32],
                        ]
                    };

                    target.draw(&tutorial.vertex_buffer, &tutorial.indices, &tutorial.program, &uniforms,
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

