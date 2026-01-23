pub mod compositor;
pub mod comp_utils;
pub mod xdg;
pub mod handlers;
pub mod config;
pub mod render;

use compositor::Waycrust;
use smithay::{delegate_xdg_shell, delegate_compositor, delegate_shm, delegate_seat, delegate_data_device};
// Macros used to delegate protocol handling to types in the app state.
delegate_xdg_shell!(Waycrust);
delegate_compositor!(Waycrust);
delegate_shm!(Waycrust);
delegate_seat!(Waycrust);
delegate_data_device!(Waycrust);