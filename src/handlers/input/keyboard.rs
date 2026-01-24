use smithay::{backend::{
    input::KeyboardKeyEvent,
    winit::WinitKeyboardInputEvent
}, input::keyboard::FilterResult, utils::SERIAL_COUNTER};

use crate::{
    compositor::Waycrust, 
    config::KeybindShortcut,
    handlers::input::keybinds::handle_keybind
};



pub fn handle_keyboard_event(state: &mut Waycrust, event: WinitKeyboardInputEvent) {
    let keyboard = match state.seat.get_keyboard() {    // might replace it with loop if I decide to add multiple seats to distinguish inputs of the same type
        Some(k) => k,
        None => return
    };
    let key_state = event.state();
    let mut key_code = event.key_code();
    for remap in &state.config.remaps {
        if remap.from.raw() == key_code.raw() {
            key_code = remap.into.raw().into();
            break
        }
    }
    keyboard.input::<(), _>(
        state,
        key_code,
        key_state,
        SERIAL_COUNTER.next_serial(),
        0,
        |state, modifiers, handle| {
            let key_symbol = handle.modified_sym();
            if key_state == smithay::backend::input::KeyState::Pressed {
                let pressed = KeybindShortcut::new_verbose(
                    key_symbol,
                    modifiers.alt,
                    modifiers.ctrl,
                    modifiers.shift,
                    modifiers.logo
                );
                let action = state.config.keybinds
                    .iter()
                    .find(|kb| kb.shortcut == pressed)
                    .map(|kb| kb.action.clone());

                handle_keybind(state, action, &keyboard)
            } else {
                FilterResult::Forward
            }
        },
    );
}