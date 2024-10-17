#[macro_use]
extern crate glium;
use glium::Surface;

#[path = "../book/tuto-07-teapot.rs"]
mod teapot;

mod support;
use support::view_matrix;

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
                .with_title("Glium tutorial #12")
                .build(event_loop);

            let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
                let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
                let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                                    &teapot::INDICES).unwrap();

            let vertex_shader_src = r#"
                #version 150

                in vec3 position;
                in vec3 normal;

                out vec3 v_normal;

                uniform mat4 perspective;
                uniform mat4 view;
                uniform mat4 model;

                void main() {
                    mat4 modelview = view * model;
                    v_normal = transpose(inverse(mat3(modelview))) * normal;
                    gl_Position = perspective * modelview * vec4(position, 1.0);
                }
            "#;

            let fragment_shader_src = r#"
                #version 150

                in vec3 v_normal;
                out vec4 color;
                uniform vec3 u_light;

                void main() {
                    float brightness = dot(normalize(v_normal), normalize(u_light));
                    vec3 dark_color = vec3(0.6, 0.0, 0.0);
                    vec3 regular_color = vec3(1.0, 0.0, 0.0);
                    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
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
                    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                    let model = [
                        [0.01, 0.0, 0.0, 0.0],
                        [0.0, 0.01, 0.0, 0.0],
                        [0.0, 0.0, 0.01, 0.0],
                        [0.0, 0.0, 2.0, 1.0f32]
                    ];

                    let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

                    let perspective = {
                        let (width, height) = target.get_dimensions();
                        let aspect_ratio = height as f32 / width as f32;

                        let fov: f32 = 3.141592 / 3.0;
                        let zfar = 1024.0;
                        let znear = 0.1;

                        let f = 1.0 / (fov / 2.0).tan();

                        [
                            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                            [         0.0         ,     f ,              0.0              ,   0.0],
                            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                        ]
                    };

                    let light = [-1.0, 0.4, 0.9f32];

                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                        .. Default::default()
                    };

                    target.draw((&tutorial.positions, &tutorial.normals), &tutorial.indices, &tutorial.program,
                                &uniform! { model: model, view: view, perspective: perspective, u_light: light },
                                &params).unwrap();
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

