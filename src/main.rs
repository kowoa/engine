use bevy::prelude::*;
use renderer::Renderer;
use winit::event::{Event, WindowEvent};

mod renderer;
mod window;

fn main() {
    App::new()
        .set_runner(run)
        .run();
}

fn run(mut app: App) {
    let (mut window, event_loop) = window::Window::new();
    
    let mut renderer = None;
    event_loop.run(move |event, window_target, control_flow| {
        control_flow.set_wait();
        match event {
            Event::Resumed => {
                window.on_resumed(window_target);

                // The context needs to be current for the Renderer to set up shaders and
                // buffers. It also performs function loading, which needs a current context on
                // WGL.
                renderer.get_or_insert_with(|| Renderer::new(&window));
            },
            Event::Suspended => window.on_suspended(),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        window.resize(size);

                        let renderer = renderer.as_ref().unwrap();
                        renderer.resize(size.width as i32, size.height as i32);
                    }
                },
                WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => (),
            },
            Event::MainEventsCleared => {
                app.update();

                let renderer = renderer.as_ref().unwrap();
                renderer.draw();

                window.swap_buffers();
            },
            _ => (),
        }
    })
}

