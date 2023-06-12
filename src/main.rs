use std::{rc::Rc, sync::{Mutex, Arc}};

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

fn setup_graphics() {
    println!("setup graphics");
}

fn main() {
    EcsBuilder::new()
        .add_resource(Time { current: 0.0, delta: 0.0 })
        .add_system(setup_graphics, StartupSingleThreaded)
        .set_runner(runner)
        .run();
}

fn runner(mut ecs: Ecs) {
    let (mut window, event_loop) = window::Window::new();
    let renderer = Arc::new(Mutex::new(None));

    event_loop.run(move |event, window_target, control_flow| {
        control_flow.set_wait();
        match event {
            Event::Resumed => {
                let mut guard = renderer.lock().unwrap();
                match guard.as_ref() {
                    // Return if Renderer has already been initialized
                    Some(_) => (),
                    None => {
                        // Make the window's context current and initialize some other things in Window
                        window.on_resumed(window_target);

                        // Initialize the Renderer after the window's context is current
                        guard.get_or_insert_with(|| Renderer::new(&window));
                        ecs.insert_non_send_resource(renderer.clone());

                        // Run startup schedules
                        ecs.run_schedule(StartupSingleThreaded);
                        ecs.run_schedule(Startup);
                    },
                };
            },
            Event::Suspended => window.on_suspended(),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => if size.width != 0 && size.height != 0 {
                    window.resize(size);

                    let renderer = renderer.lock().unwrap();
                    if let Some(renderer) = renderer.as_ref() {
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
                
                let guard = renderer.lock().unwrap();
                if let Some(renderer) = guard.as_ref() {
                    renderer.draw();
                }

                window.swap_buffers();
            },
            _ => (),
        }
    });
}