use waycrust::{
    compositor::Waycrust, 
    handlers::{
        input::{
            keyboard::handle_keyboard_event, 
            pointer::{handle_pointer_button, handle_pointer_movement}
        }, 
        window::window_resize_handler
    }, 
    render::render_screen
};

use ::winit::platform::pump_events::PumpStatus;
use smithay::{
    backend::{
        input::InputEvent,
        renderer::gles::GlesRenderer,
        winit::{self, WinitEvent},
    }
};
use wayland_server::ListeningSocket;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }

    run_winit()
}

pub fn run_winit() -> Result<(), Box<dyn std::error::Error>> {
    let (mut state, mut display) = Waycrust::init()?;
    
    let listener = ListeningSocket::bind("wayland-5").unwrap();
    
    let (mut backend, mut winit) = winit::init::<GlesRenderer>()?;

    state.size = Some(backend.window_size().to_logical(1));
    
    let start_time = std::time::Instant::now();
    
    let _keyboard = state.seat.add_keyboard(Default::default(), 200, 200);
    let _pointer = state.seat.add_pointer();
    
    unsafe {
        std::env::set_var("WAYLAND_DISPLAY", "wayland-5");
    }
    
    loop {
        let status = winit.dispatch_new_events(|event| match event {
            WinitEvent::Resized { size, .. } => {
                window_resize_handler(&mut state, size);
            }
            WinitEvent::Input(event) => match event {
                InputEvent::Keyboard { event } => {
                    handle_keyboard_event(&mut state, event);
                }
                InputEvent::PointerMotionAbsolute { event } => {
                    handle_pointer_movement(&mut state, event);
                }
                InputEvent::PointerButton { event } => {
                    handle_pointer_button(&mut state, event);
                }
                _ => {}
            },
            WinitEvent::CloseRequested => {

            }
            _ => (),
        });

        match status {
            PumpStatus::Continue => (),
            PumpStatus::Exit(_) => return Ok(()),
        };

        let damage = render_screen(&mut state, &mut backend, &mut display, &listener, start_time.elapsed().as_millis() as u32)?;

        // It is important that all events on the display have been dispatched and flushed to clients before
        // swapping buffers because this operation may block.
        backend.submit(Some(&[damage])).unwrap();
    }
}

