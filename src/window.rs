use std::ffi::{CString, self};
use std::num::NonZeroU32;

use winit::dpi::PhysicalSize;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::WindowBuilder;

use raw_window_handle::HasRawWindowHandle;

use glutin::config::{ConfigTemplateBuilder, Config};
use glutin::context::{ContextApi, ContextAttributesBuilder, Version, NotCurrentContext, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay, Display};
use glutin::prelude::*;
use glutin::surface::{SwapInterval, Surface, WindowSurface};

use glutin_winit::{self, DisplayBuilder, GlWindow};

pub struct Window {
    gl_config: Config,
    gl_display: Display,
    not_current_gl_context: Option<NotCurrentContext>,
    gl_context: Option<PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    window: Option<winit::window::Window>,
}

impl Window {
    pub fn new() -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new();

        // Only windows requires the window to be present before creating the display.
        // Other platforms don't really need one.
        //
        // XXX if you don't care about running on android or so you can safely remove
        // this condition and always pass the window builder.
        let window_builder =
            if cfg!(wgl_backend) { Some(WindowBuilder::new().with_transparent(true)) } else { None };

        // The template will match only the configurations supporting rendering
        // to windows.
        //
        // XXX We force transparency only on macOS, given that EGL on X11 doesn't
        // have it, but we still want to show window. The macOS situation is like
        // that, because we can query only one config at a time on it, but all
        // normal platforms will return multiple configs, so we can find the config
        // with transparency ourselves inside the `reduce`.
        let template =
            ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(cfg!(cgl_backend));

        let display_builder = DisplayBuilder::new().with_window_builder(window_builder);

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                // Find the config with the maximum number of samples, so our triangle will
                // be smooth.
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        println!("Picked a config with {} samples", gl_config.num_samples());

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

        // XXX The display could be obtained from the any object created by it, so we
        // can query it from the config.
        let gl_display = gl_config.display();

        // The context creation part. It can be created before surface and that's how
        // it's expected in multithreaded + multiwindow operation mode, since you
        // can send NotCurrentContext, but not Surface.
        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
            .build(raw_window_handle);

        let not_current_gl_context = Some(unsafe {
            gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
                gl_display.create_context(&gl_config, &fallback_context_attributes).unwrap_or_else(
                    |_| {
                        gl_display
                            .create_context(&gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    },
                )
            })
        });
        
        (
            Self {
                gl_config,
                gl_display,
                not_current_gl_context,
                gl_context: None,
                gl_surface: None,
                window,
            },
            event_loop
        )
    }
    
    pub fn swap_buffers(&self) {
        if let (Some(gl_context), Some(gl_surface))
            = (&self.gl_context, &self.gl_surface) {
            gl_surface.swap_buffers(gl_context).unwrap();
        }
    }
    
    pub fn get_proc_address(&self, symbol: &str) -> *const ffi::c_void {
        let symbol = CString::new(symbol).unwrap();
        self.gl_display.get_proc_address(symbol.as_c_str())
    }
    
    pub fn on_resumed(&mut self,
        window_target: &EventLoopWindowTarget<()>,
    ) {
        #[cfg(android_platform)]
        println!("Android window available");

        let window = self.window.take().unwrap_or_else(|| {
            let window_builder = WindowBuilder::new().with_transparent(true);
            glutin_winit::finalize_window(window_target, window_builder, &self.gl_config)
                .unwrap()
        });

        let attrs = window.build_surface_attributes(<_>::default());
        let gl_surface = unsafe {
            self.gl_config.display().create_window_surface(&self.gl_config, &attrs).unwrap()
        };

        // Make it current.
        let gl_context =
            self.not_current_gl_context.take().unwrap().make_current(&gl_surface).unwrap();

        // Try setting vsync.
        if let Err(res) = gl_surface
            .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("Error setting vsync: {res:?}");
        }
        
        assert!(self.gl_context.replace(gl_context).is_none()
            && self.gl_surface.replace(gl_surface).is_none()
            && self.window.replace(window).is_none()
        );
        
        // Load OpenGL function pointers
        gl::load_with(|symbol| self.get_proc_address(symbol));
        
    }
    
    pub fn on_suspended(&mut self) {
        // This event is only raised on Android, where the backing NativeWindow for a GL
        // Surface can appear and disappear at any moment.
        println!("Android window removed");

        // Destroy the GL Surface and un-current the GL Context before ndk-glue releases
        // the window back to the system.
        let gl_context = self.gl_context.take().unwrap();
        assert!(self.not_current_gl_context
            .replace(gl_context.make_not_current().unwrap())
            .is_none()
        );
    }
    
    pub fn resize(&self, size: PhysicalSize<u32>) {
        // Some platforms like EGL require resizing GL surface to update the size
        // Notable platforms here are Wayland and macOS, other don't require it
        // and the function is no-op, but it's wise to resize it for portability
        // reasons.
        if let (Some(gl_context), Some(gl_surface))
            = (&self.gl_context, &self.gl_surface) {
            gl_surface.resize(
                gl_context,
                NonZeroU32::new(size.width).unwrap(),
                NonZeroU32::new(size.height).unwrap(),
            );
        }
    }
}

