use smithay::utils::{Physical, Size};

use crate::compositor::Waycrust;

pub fn window_resize_handler(state: &mut Waycrust, size: Size<i32, Physical>) {
    state.size = Some(size.to_logical(1));

    if let Some(ref s) = state.toplevels.focused {
        s.with_pending_state(|fs| {
            fs.size = state.size;
        });

        s.send_configure();
    }
}