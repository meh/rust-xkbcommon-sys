//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use libc::{c_void, c_int, uint8_t, uint16_t, int32_t};
use crate::{xkb_context, xkb_keymap, xkb_state, xkb_keymap_compile_flags};

pub const XKB_X11_MIN_MAJOR_XKB_VERSION: c_int = 1;
pub const XKB_X11_MIN_MINOR_XKB_VERSION: c_int = 0;

pub type xkb_x11_setup_xkb_extension_flags = c_int;
pub const XKB_X11_SETUP_XKB_EXTENSION_NO_FLAGS: xkb_x11_setup_xkb_extension_flags = 0;

extern "C" {
	pub fn xkb_x11_setup_xkb_extension(connection: *mut c_void, major_xkb_version: uint16_t, minor_xkb_version: uint16_t, flags: xkb_x11_setup_xkb_extension_flags, major_xkb_version_out: *mut uint16_t, minor_xkb_version_out: *mut uint16_t, base_event_out: *mut uint8_t, base_error_out: *mut uint8_t) -> c_int;
	pub fn xkb_x11_get_core_keyboard_device_id(connection: *mut c_void) -> int32_t;
	pub fn xkb_x11_keymap_new_from_device(context: *mut xkb_context, connection: *mut c_void, device_id: int32_t, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;
	pub fn xkb_x11_state_new_from_device(keymap: *mut xkb_keymap, connection: *mut c_void, device_id: int32_t) -> *mut xkb_state;
}
