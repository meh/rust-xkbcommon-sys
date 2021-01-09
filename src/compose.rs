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

use libc::{c_void, c_char, c_int, size_t, FILE};
use crate::{xkb_context, xkb_keysym_t};

pub type xkb_compose_table = c_void;
pub type xkb_compose_state = c_void;

pub type xkb_compose_compile_flags = c_int;
pub const XKB_COMPOSE_COMPILE_NO_FLAGS: xkb_compose_compile_flags = 0;

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum xkb_compose_format {
	XKB_COMPOSE_FORMAT_TEXT_V1 = 1,
}
pub use self::xkb_compose_format::*;

pub type xkb_compose_state_flags = c_int;
pub const XKB_COMPOSE_STATE_NO_FLAGS: xkb_compose_state_flags = 0;

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum xkb_compose_status {
	XKB_COMPOSE_NOTHING,
	XKB_COMPOSE_COMPOSING,
	XKB_COMPOSE_COMPOSED,
	XKB_COMPOSE_CANCELLED
}
pub use self::xkb_compose_status::*;

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum xkb_compose_feed_result {
	XKB_COMPOSE_FEED_IGNORED,
	XKB_COMPOSE_FEED_ACCEPTED,
}
pub use self::xkb_compose_feed_result::*;

extern "C" {
	pub fn xkb_compose_table_new_from_locale(context: *mut xkb_context, locale: *const c_char, flags: xkb_compose_compile_flags) -> *mut xkb_compose_table;
	pub fn xkb_compose_table_new_from_file(context: *mut xkb_context, file: *mut FILE, format: xkb_compose_format, flags: xkb_compose_compile_flags) -> *mut xkb_compose_table;
	pub fn xkb_compose_table_new_from_buffer(context: *mut xkb_context, buffer: *const c_char, length: size_t, locale: *const c_char, format: xkb_compose_format, flags: xkb_compose_compile_flags) -> *mut xkb_compose_table;
	pub fn xkb_compose_table_ref(table: *mut xkb_compose_table) -> *mut xkb_compose_table;
	pub fn xkb_compose_table_unref(table: *mut xkb_compose_table);

	pub fn xkb_compose_state_new(table: *mut xkb_compose_table, flags: xkb_compose_state_flags) -> *mut xkb_compose_state;
	pub fn xkb_compose_state_ref(state: *mut xkb_compose_state) -> *mut xkb_compose_state;
	pub fn xkb_compose_state_unref(state: *mut xkb_compose_state);
	pub fn xkb_compose_state_get_compose_table(state: *mut xkb_compose_state) -> *mut xkb_compose_table;
	pub fn xkb_compose_state_feed(state: *mut xkb_compose_state, keysym: xkb_keysym_t) -> xkb_compose_feed_result;
	pub fn xkb_compose_state_reset(state: *mut xkb_compose_state);
	pub fn xkb_compose_state_get_status(state: *mut xkb_compose_state) -> xkb_compose_status;
	pub fn xkb_compose_state_get_utf8(state: *mut xkb_compose_state, buffer: *mut c_char, size: size_t) -> c_int;
	pub fn xkb_compose_state_get_one_sym(state: *mut xkb_compose_state) -> xkb_keysym_t;
}
