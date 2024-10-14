use glium::Surface;

fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let mut app = Tutorial01 { window_display: None };
    event_loop.run_app(&mut app).unwrap();
}

use glium::Display;
use glutin::surface::WindowSurface;
use winit::application::ApplicationHandler;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};
use winit::window::{Window, WindowId};

struct Tutorial01 {
    window_display: Option<(Window, Display<WindowSurface>)>
}

impl ApplicationHandler<()> for Tutorial01 {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_display.is_none() {
            let window_display = glium::backend::glutin::SimpleWindowBuilder::new()
                .with_title("Glium tutorial #1")
                .build(event_loop);

            // Start rendering by creating a new frame
            let mut frame = window_display.1.draw();
            // Which we fill with an opaque blue color
            frame.clear_color(0.0, 0.0, 1.0, 1.0);
            // By finishing the frame swap buffers and thereby make it visible on the window
            frame.finish().unwrap();

            self.window_display = Some(window_display);
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

