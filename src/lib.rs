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

#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;
use libc::{c_void, c_char, c_int, c_uint, size_t, uint32_t};
use libc::FILE;

mod keysyms;
pub use self::keysyms::*;

mod compose;
pub use self::compose::*;

mod names;
pub use self::names::*;

#[cfg(feature = "x11")]
mod x11;
#[cfg(feature = "x11")]
pub use self::x11::*;

pub type xkb_context = c_void;
pub type xkb_keymap = c_void;
pub type xkb_state = c_void;

pub type xkb_keycode_t = uint32_t;
pub type xkb_keysym_t = uint32_t;

pub type xkb_layout_index_t = uint32_t;
pub type xkb_layout_mask_t = uint32_t;

pub type xkb_level_index_t = uint32_t;

pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;

pub type xkb_led_index_t = uint32_t;
pub type xkb_led_mask_t = uint32_t;

pub const XKB_KEYCODE_INVALID: uint32_t = 0xffffffff;
pub const XKB_LAYOUT_INVALID:  uint32_t = 0xffffffff;
pub const XKB_LEVEL_INVALID:   uint32_t = 0xffffffff;
pub const XKB_MOD_INVALID:     uint32_t = 0xffffffff;
pub const XKB_LED_INVALID:     uint32_t = 0xffffffff;
pub const XKB_KEYCODE_MAX:     uint32_t = 0xffffffff - 1;

#[inline(always)]
pub unsafe fn xkb_keycode_is_legal_ext(key: xkb_keycode_t) -> bool {
	key <= XKB_KEYCODE_MAX
}

#[inline(always)]
pub unsafe fn xkb_keycode_is_legal_x11(key: xkb_keycode_t) -> bool {
	key >= 8 && key <= 255
}

#[repr(C)]
#[derive(Debug)]
pub struct xkb_rule_names {
	pub rules:   *const c_char,
	pub model:   *const c_char,
	pub layout:  *const c_char,
	pub variant: *const c_char,
	pub options: *const c_char,
}

pub type xkb_keysym_flags = c_int;
pub const XKB_KEYSYM_NO_FLAGS:         xkb_keysym_flags = 0;
pub const XKB_KEYSYM_CASE_INSENSITIVE: xkb_keysym_flags = 1 << 0;

pub type xkb_context_flags = c_int;
pub const XKB_CONTEXT_NO_FLAGS:             xkb_context_flags = 0;
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES:  xkb_context_flags = 1 << 0;
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 1 << 1;

pub type xkb_keymap_compile_flags = c_int;
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum xkb_keymap_format {
	XKB_KEYMAP_USE_ORIGINAL_FORMAT = -1,
	XKB_KEYMAP_FORMAT_TEXT_v1      = 1,
}
pub use xkb_keymap_format::*;

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum xkb_log_level {
	XKB_LOG_LEVEL_CRITICAL = 10,
	XKB_LOG_LEVEL_ERROR    = 20,
	XKB_LOG_LEVEL_WARNING  = 30,
	XKB_LOG_LEVEL_INFO     = 40,
	XKB_LOG_LEVEL_DEBUG    = 50,
}
pub use xkb_log_level::*;

pub type xkb_keymap_key_iter_t = extern "C" fn(*mut xkb_keymap, key: xkb_keycode_t, data: *mut c_void);

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum xkb_key_direction {
	XKB_KEY_UP,
	XKB_KEY_DOWN,
}
pub use xkb_key_direction::*;

pub type xkb_state_component = c_int;
pub const XKB_STATE_MODS_DEPRESSED:   xkb_state_component = 1 << 0;
pub const XKB_STATE_MODS_LATCHED:     xkb_state_component = 1 << 1;
pub const XKB_STATE_MODS_LOCKED:      xkb_state_component = 1 << 2;
pub const XKB_STATE_MODS_EFFECTIVE:   xkb_state_component = 1 << 3;
pub const XKB_STATE_LAYOUT_DEPRESSED: xkb_state_component = 1 << 4;
pub const XKB_STATE_LAYOUT_LATCHED:   xkb_state_component = 1 << 5;
pub const XKB_STATE_LAYOUT_LOCKED:    xkb_state_component = 1 << 6;
pub const XKB_STATE_LAYOUT_EFFECTIVE: xkb_state_component = 1 << 7;
pub const XKB_STATE_LEDS:             xkb_state_component = 1 << 8;

pub type xkb_state_match = c_int;
pub const XKB_STATE_MATCH_ANY:           xkb_state_match = 1 << 0;
pub const XKB_STATE_MATCH_ALL:           xkb_state_match = 1 << 1;
pub const XKB_STATE_MATCH_NON_EXCLUSIVE: xkb_state_match = 1 << 16;

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum xkb_consumed_mode {
	XKB_CONSUMED_MODE_XKB,
	XKB_CONSUMED_MODE_GTK,
}
pub use xkb_consumed_mode::*;

extern "C" {
	pub fn xkb_keysym_get_name(keysym: xkb_keysym_t, buffer: *mut c_char, size: size_t) -> c_int;
	pub fn xkb_keysym_from_name(name: *const c_char, flags: xkb_keysym_flags) -> xkb_keysym_t;
	pub fn xkb_keysym_to_utf8(keysym: xkb_keysym_t, buffer: *mut c_char, size: size_t) -> c_int;
	pub fn xkb_keysym_to_utf32(keysym: xkb_keysym_t) -> uint32_t;

	pub fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
	pub fn xkb_context_ref(context: *mut xkb_context) -> *mut xkb_context;
	pub fn xkb_context_unref(context: *mut xkb_context);
	pub fn xkb_context_set_user_data(context: *mut xkb_context, user_data: *mut c_void);
	pub fn xkb_context_get_user_data(context: *mut xkb_context) -> *mut c_void;
	pub fn xkb_context_include_path_append(context: *mut xkb_context, path: *const c_char) -> c_int;
	pub fn xkb_context_include_path_append_default(context: *mut xkb_context) -> c_int;
	pub fn xkb_context_include_path_reset_defaults(context: *mut xkb_context) -> c_int;
	pub fn xkb_context_include_path_clear(context: *mut xkb_context);
	pub fn xkb_context_num_include_paths(context: *mut xkb_context) -> c_uint;
	pub fn xkb_context_include_path_get(context: *mut xkb_context, index: c_uint) -> *const c_char;
	pub fn xkb_context_set_log_level(context: *mut xkb_context, level: xkb_log_level);
	pub fn xkb_context_get_log_level(context: *mut xkb_context) -> xkb_log_level;
	pub fn xkb_context_set_log_verbosity(context: *mut xkb_context, verbosity: c_int);
	pub fn xkb_context_get_log_verbosity(context: *mut xkb_context) -> c_int;
//	pub fn xkb_context_set_log_fn(context: *mut xkb_context, log_fn: extern "C" fn(*mut xkb_context, xkb_log_level, *const c_char, va_list));

	pub fn xkb_keymap_new_from_names(context: *mut xkb_context, names: *const xkb_rule_names, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;
	pub fn xkb_keymap_new_from_file(context: *mut xkb_context, file: *mut FILE, format: xkb_keymap_format, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;
	pub fn xkb_keymap_new_from_string(context: *mut xkb_context, string: *const c_char, format: xkb_keymap_format, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;
	pub fn xkb_keymap_new_from_buffer(context: *mut xkb_context, buffer: *const c_char, length: size_t, format: xkb_keymap_format, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;
	pub fn xkb_keymap_ref(keymap: *mut xkb_keymap) -> *mut xkb_keymap;
	pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
	pub fn xkb_keymap_get_as_string(keymap: *mut xkb_keymap, format: xkb_keymap_format) -> *mut c_char;
	pub fn xkb_keymap_min_keycode(keymap: *mut xkb_keymap) -> xkb_keycode_t;
	pub fn xkb_keymap_max_keycode(keymap: *mut xkb_keymap) -> xkb_keycode_t;
	pub fn xkb_keymap_key_for_each(keymap: *mut xkb_keymap, iter: xkb_keymap_key_iter_t, data: *mut c_void);
	pub fn xkb_keymap_key_get_name(keymap: *mut xkb_keymap, key: xkb_keycode_t) -> *const c_char;
	pub fn xkb_keymap_key_by_name(keymap: *mut xkb_keymap, name: *const c_char) -> xkb_keycode_t;
	pub fn xkb_keymap_num_mods(keymap: *mut xkb_keymap) -> xkb_mod_index_t;
	pub fn xkb_keymap_mod_get_name(keymap: *mut xkb_keymap, idx: xkb_mod_index_t) -> *const c_char;
	pub fn xkb_keymap_mod_get_index(keymap: *mut xkb_keymap, name: *const c_char) -> xkb_mod_index_t;
	pub fn xkb_keymap_num_layouts(keymap: *mut xkb_keymap) -> xkb_layout_index_t;
	pub fn xkb_keymap_layout_get_name(keymap: *mut xkb_keymap, idx: xkb_layout_index_t) -> *const c_char;
	pub fn xkb_keymap_layout_get_index(keymap: *mut xkb_keymap, name: *const c_char) -> *const c_char;
	pub fn xkb_keymap_num_leds(keymap: *mut xkb_keymap) -> xkb_led_index_t;
	pub fn xkb_keymap_led_get_name(keymap: *mut xkb_keymap, idx: xkb_led_index_t) -> *const c_char;
	pub fn xkb_keymap_led_get_index(keymap: *mut xkb_keymap, name: *const c_char) -> xkb_led_index_t;
	pub fn xkb_keymap_num_layouts_for_key(keymap: *mut xkb_keymap, key: xkb_keycode_t) -> xkb_layout_index_t;
	pub fn xkb_keymap_num_levels_for_key(keymap: *mut xkb_keymap, key: xkb_keycode_t, layout: xkb_layout_index_t) -> xkb_level_index_t;
	pub fn xkb_keymap_key_get_syms_by_level(keymap: *mut xkb_keymap, key: xkb_keycode_t, layout: xkb_layout_index_t, level: xkb_level_index_t, syms_out: *mut *mut xkb_keysym_t) -> c_int;
	pub fn xkb_keymap_key_repeats(keymap: *mut xkb_keymap, key: xkb_keycode_t) -> c_int;

	pub fn xkb_state_new(keymap: *mut xkb_keymap) -> *mut xkb_state;
	pub fn xkb_state_ref(state: *mut xkb_state) -> *mut xkb_state;
	pub fn xkb_state_unref(state: *mut xkb_state);
	pub fn xkb_state_get_keymap(state: *mut xkb_state) -> *mut xkb_keymap;
	pub fn xkb_state_update_key(state: *mut xkb_state, key: xkb_keycode_t, direction: xkb_key_direction) -> xkb_state_component;
	pub fn xkb_state_update_mask(state: *mut xkb_state, depressed_mods: xkb_mod_mask_t, latched_mods: xkb_mod_mask_t, locked_mods: xkb_mod_mask_t, depressed_layout: xkb_layout_index_t, latched_layout: xkb_layout_index_t, locked_layout: xkb_layout_index_t) -> xkb_state_component;
	pub fn xkb_state_key_get_syms(state: *mut xkb_state, key: xkb_keycode_t, syms_out: *mut *mut xkb_keysym_t) -> c_int;
	pub fn xkb_state_key_get_utf8(state: *mut xkb_state, key: xkb_keycode_t, buffer: *mut c_char, size: size_t) -> c_int;
	pub fn xkb_state_key_get_utf32(state: *mut xkb_state, key: xkb_keycode_t) -> uint32_t;
	pub fn xkb_state_key_get_one_sym(state: *mut xkb_state, key: xkb_keycode_t) -> xkb_keysym_t;
	pub fn xkb_state_key_get_layout(state: *mut xkb_state, key: xkb_keycode_t) -> xkb_layout_index_t;
	pub fn xkb_state_key_get_level(state: *mut xkb_state, key: xkb_keycode_t, layout: xkb_layout_index_t) -> xkb_level_index_t;
	pub fn xkb_state_serialize_mods(state: *mut xkb_state, components: xkb_state_component) -> xkb_mod_mask_t;
	pub fn xkb_state_serialize_layout(state: *mut xkb_state, components: xkb_state_component) -> xkb_layout_index_t;
	pub fn xkb_state_mod_name_is_active(state: *mut xkb_state, name: *const c_char, _type: xkb_state_component) -> c_int;
	pub fn xkb_state_mod_names_are_active(state: *mut xkb_state, _type: xkb_state_component, _match: xkb_state_match, ...) -> c_int;
	pub fn xkb_state_mod_index_is_active(state: *mut xkb_state, idx: xkb_mod_index_t, _type: xkb_state_component) -> c_int;
	pub fn xkb_state_mod_indices_are_active(state: *mut xkb_state, _type: xkb_state_component, _match: xkb_state_match, ...) -> c_int;
	pub fn xkb_state_key_get_consumed_mods2(state: *mut xkb_state, key: xkb_keycode_t, mode: xkb_consumed_mode) -> xkb_mod_mask_t;
	pub fn xkb_state_key_get_consumed_mods(state: *mut xkb_state, key: xkb_keycode_t) -> xkb_mod_mask_t;
	pub fn xkb_state_mod_index_is_consumed2(state: *mut xkb_state, key: xkb_keycode_t, idx: xkb_mod_index_t, mode: xkb_consumed_mode) -> c_int;
	pub fn xkb_state_mod_index_is_consumed(state: *mut xkb_state, key: xkb_keycode_t, idx: xkb_mod_index_t) -> c_int;
	pub fn xkb_state_mod_mask_remove_consumed(state: *mut xkb_state, key: xkb_keycode_t, mask: xkb_mod_mask_t) -> xkb_mod_mask_t;
	pub fn xkb_state_layout_name_is_active(state: *mut xkb_state, name: *const c_char, _type: xkb_state_component) -> c_int;
	pub fn xkb_state_layout_index_is_active(state: *mut xkb_state, idx: xkb_layout_index_t, _type: xkb_state_component) -> c_int;
	pub fn xkb_state_led_name_is_active(state: *mut xkb_state, name: *const c_char) -> c_int;
	pub fn xkb_state_led_index_is_active(state: *mut xkb_state, idx: xkb_led_index_t) -> c_int;
}
