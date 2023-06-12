use bevy_ecs::schedule::{ScheduleLabel, Schedule};
use winit::event::{Event, WindowEvent};

mod common;
use common::Time;

mod ecs;
use ecs::*;

mod renderer;
use renderer::Renderer;

mod window;
mod macros;

mod systems;
use systems::*;


fn test_system() {
    println!("testing");
}

fn main() {
    EcsBuilder::new()
        .add_resource(Time { current: 0.0, delta: 0.0 })
        .add_system(test_system, Startup)
        .set_runner(runner)
        .run();
}

fn runner(mut ecs: Ecs) {
    let (mut window, event_loop) = window::Window::new();
    let mut renderer = None;

    ecs.run_schedule(StartupSingleThreaded);
    ecs.run_schedule(Startup);

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
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        if key == winit::event::VirtualKeyCode::Escape {
                            control_flow.set_exit();
                        }
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                ecs.run_schedule(Update);
                ecs.run_schedule(Render);
                
                let renderer = renderer.as_ref().unwrap();
                renderer.draw();

                window.swap_buffers();
            },
            _ => (),
        }
    });
}