use std::{collections::VecDeque, fs};

use smithay::{input::SeatState, utils::SERIAL_COUNTER, wayland::{compositor::CompositorState, selection::data_device::DataDeviceState, shell::xdg::{ToplevelSurface, XdgShellState}, shm::ShmState}};
use wayland_protocols::xdg::shell::server::xdg_toplevel;
use wayland_server::Display;

use crate::{compositor::{TopLevelWindows, Waycrust}, config::Config};



impl Waycrust {
    pub fn focus_toplevel(&mut self, surface: Option<ToplevelSurface>) {
        let kbd = match self.seat.get_keyboard() {
            Some(k) => k,
            None => return
        };

        if let Some(ref s) = surface {
            kbd.set_focus(self, Some(s.wl_surface().clone()), SERIAL_COUNTER.next_serial());

            s.with_pending_state(|state| {
                state.states.set(xdg_toplevel::State::Fullscreen);
                state.size = self.size
            });
            s.send_configure();
        } else {
            kbd.set_focus(self, None, SERIAL_COUNTER.next_serial());
        }

        self.toplevels.focused = surface;
    }

    pub fn next_toplevel(&mut self) {
        if self.toplevels.toplevels.len() < 2 {
            return
        }

        if let Some(front) = self.toplevels.toplevels.pop_front() {
            self.toplevels.toplevels.push_back(front);
        }

        let next = self.toplevels.toplevels.front().cloned();
        self.focus_toplevel(next);
    }

    pub fn previous_toplevel(&mut self) {
        if self.toplevels.toplevels.len() < 2 {
            return
        }

        if let Some(back) = self.toplevels.toplevels.pop_back() {
            self.toplevels.toplevels.push_front(back);
        }

        let next = self.toplevels.toplevels.front().cloned();
        self.focus_toplevel(next);
    }

    pub fn init() -> Result<(Self, Display<Self>), Box<dyn std::error::Error>> {

        let display: Display<Waycrust> = Display::new()?;
        let dh = display.handle();

        let compositor_state = CompositorState::new::<Waycrust>(&dh);
        let shm_state = ShmState::new::<Waycrust>(&dh, vec![]);
        let mut seat_state = SeatState::new();
        let seat = seat_state.new_wl_seat(&dh, "winit");

        Ok((Waycrust {
            compositor_state,
            xdg_shell_state: XdgShellState::new::<Waycrust>(&dh),
            shm_state,
            seat_state,
            data_device_state: DataDeviceState::new::<Waycrust>(&dh),
            toplevels: TopLevelWindows { toplevels: VecDeque::new(), focused: None },
            seat,
            size: None,
            config: load_config(),
            flipped: false
        }, display))
    }
}

fn load_config() -> Config {
    let config_file = if fs::exists("./config.ron").is_ok_and(|b| b) {
        println!("config found next to bin");
        "./config.ron"
    } else if fs::exists("~/.config/waycrust/config.ron").is_ok_and(|b| b) {
        println!("config found in .config");
        "~/.config/waycrust/config.ron"
    } else {
        println!("config not found");
        return Config::default()
    };

    if let Ok(content) = fs::read_to_string(config_file) {
        println!("reading config");
        ron::from_str(&content).unwrap()
    } else {
        println!("cannot read config");
        Config::default()
    }
}