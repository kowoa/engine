use bevy::prelude::*;
use glutin::surface::GlSurface;
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
    
    let mut state = None;
    let mut renderer = None;
    event_loop.run(move |event, window_target, control_flow| {
        control_flow.set_wait();
        match event {
            Event::Resumed => window.on_resumed(window_target, &mut state, &mut renderer),
            Event::Suspended => window.on_suspended(&mut state),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        window.on_resized(size, &mut state, &mut renderer)
                    }
                },
                WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => (),
            },
            Event::MainEventsCleared => {
                app.update();

                let renderer = renderer.as_ref().unwrap();
                renderer.draw();

                if let Some((gl_context, gl_surface, window)) = &state {
                    gl_surface.swap_buffers(gl_context).unwrap();
                }
            },
            _ => (),
        }
    })
}

