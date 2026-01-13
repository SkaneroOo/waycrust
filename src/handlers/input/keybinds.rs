use smithay::input::keyboard::{FilterResult, KeyboardHandle};

use crate::{
    config::KeybindAction::{self, *},
    Waycrust
};

pub fn handle_keybind(state: &mut Waycrust, action: Option<KeybindAction>, keyboard: &KeyboardHandle<Waycrust>) -> FilterResult<()> {
    if let Some(action) = action {
        match action {
            CycleNext => state.next_toplevel(),
            CyclePrev => state.previous_toplevel(),
            Kill => {
                let focused = match keyboard.current_focus() {
                    Some(f) => f,
                    None => {
                        return FilterResult::Intercept(())
                    }
                };
                let toplevel = match state.xdg_shell_state.toplevel_surfaces().iter().find(|t| t.wl_surface() == &focused) {
                    Some(t) => t,
                    None => {
                        return FilterResult::Intercept(())
                    }
                };
                toplevel.send_close();
            }
            Exec(command) => {
                let (command, args) = {
                    let mut iter = shlex::Shlex::new(&command);
                    let command = iter.next().unwrap_or_default().to_string();
                    let args: Vec<String> = iter.collect();
                    (command, args)
                };

                std::process::Command::new(command).args(args).spawn().ok();
            }
        }   
        FilterResult::Intercept(())
    } else {
        FilterResult::Forward
    }
}