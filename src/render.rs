use std::sync::Arc;

use smithay::{backend::{
    renderer::{
        Color32F, Frame, Renderer, element::{
            Kind, 
            surface::{WaylandSurfaceRenderElement, render_elements_from_surface_tree}}, gles::GlesRenderer, utils::draw_render_elements
        }, 
    winit::WinitGraphicsBackend
}, utils::{Physical, Rectangle, Transform}, wayland::compositor::{SurfaceAttributes, TraversalAction, with_surface_tree_downward}};
use wayland_server::{Display, ListeningSocket, protocol::wl_surface};

use crate::compositor::{ClientState, Waycrust};

pub fn render_screen(
    state: &mut Waycrust, 
    backend: &mut WinitGraphicsBackend<GlesRenderer>, 
    display: &mut Display<Waycrust>, 
    listener: &ListeningSocket,
    time: u32
) -> Result<Rectangle<i32, Physical>, Box<dyn std::error::Error>> {
    let size = backend.window_size();
    let (renderer, mut framebuffer) = backend.bind().unwrap();
    
    let to_render: Vec<WaylandSurfaceRenderElement<GlesRenderer>> = match state.toplevels.focused {
        Some(ref top) => render_elements_from_surface_tree(
            renderer,
            top.wl_surface(),
            (0, 0),
            1.0,
            1.0,
            Kind::Unspecified
        ),
        None => vec![]
    };
    
    let damage = Rectangle::from_size(size);

    let mut frame = if state.flipped {
        renderer
            .render(&mut framebuffer, size, Transform::_180)
            .unwrap()
    } else {
        renderer
            .render(&mut framebuffer, size, Transform::Flipped180)
            .unwrap()
    };
    frame.clear(Color32F::new(0.1, 0.1, 0.1, 1.0), &[damage]).unwrap();
    draw_render_elements(&mut frame, 1.0, &to_render, &[damage]).unwrap();
    // We rely on the nested compositor to do the sync for us
    let _ = frame.finish().unwrap();

    if let Some(ref surface) = state.toplevels.focused {
        send_frames_surface_tree(surface.wl_surface(), time);
    }

    if let Some(stream) = listener.accept()? {
        println!("Got a client: {:?}", stream);

        let _ = display.handle()
               .insert_client(stream, Arc::new(ClientState::default())).unwrap();
    }

    display.dispatch_clients(state)?;
    display.flush_clients()?;

    Ok(damage)
}

fn send_frames_surface_tree(surface: &wl_surface::WlSurface, time: u32) {
    with_surface_tree_downward(
        surface,
        (),
        |_, _, &()| TraversalAction::DoChildren(()),
        |_surf, states, &()| {
            // the surface may not have any user_data if it is a subsurface and has not
            // yet been commited
            for callback in states
                .cached_state
                .get::<SurfaceAttributes>()
                .current()
                .frame_callbacks
                .drain(..)
            {
                callback.done(time);
            }
        },
        |_, _, &()| true,
    );
}


