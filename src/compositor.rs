use std::collections::VecDeque;

use smithay::{
    backend::renderer::utils::on_commit_buffer_handler, input::{Seat, SeatHandler, SeatState}, utils::Size, wayland::{
        buffer::BufferHandler, compositor::{CompositorClientState, CompositorHandler, CompositorState}, selection::{SelectionHandler, data_device::{DataDeviceHandler, DataDeviceState, WaylandDndGrabHandler}}, shell::xdg::{ToplevelSurface, XdgShellState}, shm::{ShmHandler, ShmState}
    }
};
use wayland_server::{Client, backend::{ClientData, ClientId, DisconnectReason}, protocol::{wl_buffer, wl_surface::WlSurface}};

use crate::config::Config;



pub struct Waycrust {
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub shm_state: ShmState,
    pub seat_state: SeatState<Self>,
    pub data_device_state: DataDeviceState,
    pub toplevels: TopLevelWindows,

    pub seat: Seat<Self>,
    pub size: Option<Size<i32, smithay::utils::Logical>>,
    pub config: Config
}

pub struct TopLevelWindows {
    pub toplevels: VecDeque<ToplevelSurface>,
    pub focused: Option<ToplevelSurface>
}


impl SeatHandler for Waycrust {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&WlSurface>) {}
    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: smithay::input::pointer::CursorImageStatus) {}
}

impl SelectionHandler for Waycrust {
    type SelectionUserData = ();
}

impl DataDeviceHandler for Waycrust {
    fn data_device_state(&mut self) -> &mut DataDeviceState {
        &mut self.data_device_state
    }
}

impl WaylandDndGrabHandler for Waycrust {}

impl CompositorHandler for Waycrust {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);
    }
}

impl ShmHandler for Waycrust {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

impl BufferHandler for Waycrust {
    fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {}
}

#[derive(Default)]
pub struct ClientState {
    compositor_state: CompositorClientState,
}
impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {
        println!("initialized");
    }

    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {
        println!("disconnected");
    }
}