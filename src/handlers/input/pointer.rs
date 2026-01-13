use smithay::{backend::{input::{AbsolutePositionEvent, PointerButtonEvent}, winit::{WinitMouseInputEvent, WinitMouseMovedEvent}}, input::pointer::{ButtonEvent, MotionEvent}, utils::SERIAL_COUNTER};

use crate::compositor::Waycrust;

pub fn handle_pointer_movement(state: &mut Waycrust, event: WinitMouseMovedEvent) {
    let pointer = match state.seat.get_pointer() {
        Some(p) => p,
        None => return
    };
    let point = event.position();

    let location = (point.x, point.y).into();

    let event = MotionEvent {
        location,
        serial: SERIAL_COUNTER.next_serial(),
        time: 0
    };

    let focus = match state.toplevels.focused {
        Some(ref f) => Some((f.wl_surface().clone(), (0.0, 0.0).into())),
        None => None
    };

    pointer.motion(
        state,
        focus,
        &event,
    );
    pointer.frame(state);
}

pub fn handle_pointer_button(state: &mut Waycrust, event: WinitMouseInputEvent) {
    let pointer = match state.seat.get_pointer() {
        Some(p) => p,
        None => return
    };
    let event = ButtonEvent {
        serial: 0.into(),
        time: 0,
        button: event.button_code(),
        state: event.state()
    };
    pointer.button(state, &event);
    pointer.frame(state);
}