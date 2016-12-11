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

use libc::c_char;

pub const XKB_MOD_NAME_SHIFT: *const c_char = b"Shift\0" as *const _ as *const c_char;
pub const XKB_MOD_NAME_CAPS:  *const c_char = b"Lock\0" as *const _ as *const c_char;
pub const XKB_MOD_NAME_CTRL:  *const c_char = b"Control\0" as *const _ as *const c_char;
pub const XKB_MOD_NAME_ALT:   *const c_char = b"Mod1\0" as *const _ as *const c_char;
pub const XKB_MOD_NAME_NUM:   *const c_char = b"Mod2\0" as *const _ as *const c_char;
pub const XKB_MOD_NAME_LOGO:  *const c_char = b"Mod4\0" as *const _ as *const c_char;

pub const XKB_LED_NAME_CAPS:   *const c_char = b"Caps Lock\0" as *const _ as *const c_char;
pub const XKB_LED_NAME_NUM:    *const c_char = b"Num Lock\0" as *const _ as *const c_char;
pub const XKB_LED_NAME_SCROLL: *const c_char = b"Scroll Lock\0" as *const _ as *const c_char;
