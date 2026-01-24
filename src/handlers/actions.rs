use crate::{compositor::Waycrust, socket::Action};

pub fn handle_actions(state: &mut Waycrust, action: Action) {
    match action {
        Action::Exit => {
            if let Some(ref toplevel) = state.toplevels.focused {
                toplevel.send_close();
            }
        }
        Action::Exec(command) => {
            let (command, args) = {
                let mut iter = shlex::Shlex::new(&command);
                let command = iter.next().unwrap_or_default().to_string();
                let args: Vec<String> = iter.collect();
                (command, args)
            };

            std::process::Command::new(command).args(args).spawn().ok();
        }
        Action::Flip => {
            state.flipped = !state.flipped;
        }
    }
}