use std::{rc::Rc, sync::{Mutex, Arc}, time::{SystemTime, UNIX_EPOCH, Instant, Duration}};

use bevy_ecs::{schedule::{ScheduleLabel, Schedule}, system::{Res, NonSend}, prelude::{Events, EventReader}, world::World};
use input::{process_input_event, InputPlugin, InputEvent};
use render::RenderPlugin;
use window::WindowInfo;
use winit::event::{Event, WindowEvent, KeyboardInput};

mod common;
use common::{Time, update_time_res};

mod ecs;
use ecs::*;
mod input;
mod render;
mod window;

fn main() {
    EcsBuilder::new()
        .add_plugin(InputPlugin)
        .add_plugin(RenderPlugin)
        .insert_resource(Time { current: 0.0, delta: 0.0 })
        .set_runner(runner)
        .build()
        .run();
}

fn runner(mut world: World) {
    let window_info = WindowInfo {
        width: 800,
        height: 600,
        title: "engine",
    };
    let (mut window, event_loop) = window::Window::new(&window_info);

    let mut renderer_initialized = false;
    let start_time = Instant::now();

    event_loop.run(move |event, window_target, control_flow| {
        control_flow.set_wait();

        match event {
            Event::Resumed => {
                if renderer_initialized { return; }

                // Make the window's context current and initialize some other things in Window
                window.resume(window_target, &window_info);
                
                // Add window info as a resource
                world.insert_resource(window_info.clone());

                // Run startup schedules
                world.run_schedule(StartupSingleThreaded); // Renderer should be initialized here
                world.run_schedule(Startup); // App logic should be initialized here
                
                renderer_initialized = true;
            },
            Event::Suspended => window.suspend(),
            Event::WindowEvent { event, .. } => {
                process_input_event(&event, &mut world);

                match event {
                    WindowEvent::Resized(size) => if size.width != 0 && size.height != 0 {
                        // Update the Window size
                        window.resize(size);

                        // Update the WindowInfo resource
                        world.insert_resource({
                            let mut window_info = window_info.clone();
                            window_info.width = size.width;
                            window_info.height = size.height;
                            window_info
                        });

                        // Update the Renderer size
                        if renderer_initialized {
                            render::resize(size.width as i32, size.height as i32);
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
                update_time_res(start_time, &mut world);

                world.run_schedule(PreUpdate);
                world.run_schedule(Update);
                world.run_schedule(Render);

                window.swap_buffers();
            },
            _ => (),
        }
    });
}