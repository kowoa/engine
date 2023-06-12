use std::{rc::Rc, sync::{Mutex, Arc}, time::{SystemTime, UNIX_EPOCH, Instant, Duration}};

use bevy_ecs::{schedule::{ScheduleLabel, Schedule}, system::{Res, NonSend}, prelude::Events};
use input::{process_input_event, Input, InputEvent};
use winit::event::{Event, WindowEvent, KeyboardInput};

mod common;
use common::{Time, update_time_res};

mod ecs;
use ecs::*;

mod input;

mod renderer;
use renderer::Renderer;

mod window;
mod macros;

mod systems;
use systems::*;

fn setup_graphics() {
    println!("setup graphics");
}

fn draw(renderer: NonSend<Arc<Mutex<Option<Renderer>>>>) {
    let guard= renderer.lock().unwrap();
    if let Some(renderer) = guard.as_ref() {
        renderer.draw();
    }
}

fn main() {
    EcsBuilder::new()
        .add_plugin(RenderPlugin)
        .add_resource(Time { current: Instant::now(), delta: Duration::ZERO })
        .add_system(setup_graphics, StartupSingleThreaded)
        .add_system(draw, Render)
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
            Event::WindowEvent { event, .. } => {
                process_input_event(&event, &mut ecs);

                match event {
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
                }
            },
            Event::MainEventsCleared => {
                update_time_res(Instant::now(), &mut ecs);

                ecs.run_schedule(PreUpdate);
                ecs.run_schedule(Update);
                ecs.run_schedule(Render);

                window.swap_buffers();
            },
            _ => (),
        }
    });
}