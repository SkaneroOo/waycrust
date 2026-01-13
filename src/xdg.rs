use smithay::{utils::Serial, wayland::shell::xdg::{PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState}};
use wayland_protocols::xdg::shell::server::xdg_toplevel;
use wayland_server::protocol::wl_seat;

use crate::compositor::Waycrust;

impl XdgShellHandler for Waycrust {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        surface.with_pending_state(|state| {
            state.states.set(xdg_toplevel::State::Fullscreen);
            state.size = self.size;
        });
        
        surface.send_configure();

        self.toplevels.toplevels.push_front(surface.clone());

        self.focus_toplevel(Some(surface));
    }

    fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
        self.toplevels.toplevels.retain(|s| s != &surface);

        if self.toplevels.focused.as_ref() == Some(&surface) {
            let next = self.toplevels.toplevels.front().cloned();
            self.focus_toplevel(next);
        }
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        // Handle popup creation here
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {
        // Handle popup grab here
    }

    fn reposition_request(&mut self, _surface: PopupSurface, _positioner: PositionerState, _token: u32) {
        // Handle popup reposition here
    }
}