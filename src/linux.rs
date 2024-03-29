use std::io::{Error, ErrorKind};
use std::mem;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ushort};
use std::ptr;

use crate::{LockKey, LockKeyResult, LockKeyState, LockKeyWrapper, LockKeys};

#[doc(hidden)]
#[allow(non_upper_case_globals)]
pub const XkbUseCoreKbd: c_uint = 0x0100;
#[doc(hidden)]
#[allow(non_upper_case_globals)]
pub const XK_Caps_Lock: c_uint = 0xffe5;
#[doc(hidden)]
#[allow(non_upper_case_globals)]
pub const XK_Num_Lock: c_uint = 0xff7f;
#[doc(hidden)]
#[allow(non_upper_case_globals)]
pub const XK_Scroll_Lock: c_uint = 0xff14;

#[doc(hidden)]
pub enum Display {}
pub type KeySym = c_ulong;

#[doc(hidden)]
#[repr(C)]
pub struct XkbStateRec {
    pub group: c_uchar,
    pub locked_group: c_uchar,
    pub base_group: c_ushort,
    pub latched_group: c_ushort,
    pub mods: c_uchar,
    pub base_mods: c_uchar,
    pub latched_mods: c_uchar,
    pub locked_mods: c_uchar,
    pub compat_state: c_uchar,
    pub grab_mods: c_uchar,
    pub compat_grab_mods: c_uchar,
    pub lookup_mods: c_uchar,
    pub compat_lookup_mods: c_uchar,
    pub ptr_buttons: c_ushort,
}

#[doc(hidden)]
pub type XkbStatePtr = *mut XkbStateRec;

#[link(name = "X11")]
extern "C" {
    pub fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
    pub fn XCloseDisplay(display: *mut Display) -> c_int;
    pub fn XkbLockModifiers(
        display: *mut Display,
        device_spec: c_uint,
        affect: c_uint,
        values: c_uint,
    ) -> c_int;
    pub fn XkbKeysymToModifiers(dpy: *mut Display, ks: KeySym) -> c_uint;
    pub fn XkbGetState(
        display: *mut Display,
        device_spec: c_uint,
        state_return: XkbStatePtr,
    ) -> c_int;
}

#[doc(hidden)]
#[macro_export]
macro_rules! xkb_lock_key_mask {
    ($handle:expr,$key:expr) => {
        XkbKeysymToModifiers(
            $handle as *mut _,
            match $key {
                LockKeys::CapitalLock => XK_Caps_Lock,
                LockKeys::NumberLock => XK_Num_Lock,
                LockKeys::ScrollingLock => XK_Scroll_Lock,
            } as KeySym,
        )
    };
}

impl LockKeyWrapper for LockKey {
    /// Creates a new lock key object using [Xlib](https://en.wikipedia.org/wiki/Xlib) for handling.
    fn new() -> Self {
        LockKey {
            handle: unsafe { XOpenDisplay(ptr::null()) } as *mut _,
        }
    }

    /// Sets a new state for the lock key using [Xlib](https://en.wikipedia.org/wiki/Xlib).
    fn set(&self, key: LockKeys, state: LockKeyState) -> LockKeyResult {
        unsafe {
            let mask = xkb_lock_key_mask!(self.handle, key);
            if XkbLockModifiers(
                self.handle as *mut _,
                XkbUseCoreKbd,
                mask,
                if state.into() { mask } else { 0 },
            ) != 1
            {
                return Err(Error::new(ErrorKind::Other, "XkbLockModifiers"));
            }
            Ok(state)
        }
    }

    /// Enables the lock key using [Xlib](https://en.wikipedia.org/wiki/Xlib).
    fn enable(&self, key: LockKeys) -> LockKeyResult {
        self.set(key, LockKeyState::Enabled)
    }

    /// Disables the lock key using [Xlib](https://en.wikipedia.org/wiki/Xlib).
    fn disable(&self, key: LockKeys) -> LockKeyResult {
        self.set(key, LockKeyState::Disabled)
    }

    /// Toggles the lock key state returning its previous state using [Xlib](https://en.wikipedia.org/wiki/Xlib).
    fn toggle(&self, key: LockKeys) -> LockKeyResult {
        let state = self.state(key)?;
        self.set(key, state.toggle())?;
        Ok(state)
    }

    /// Retrieves the lock key state using [Xlib](https://en.wikipedia.org/wiki/Xlib).
    fn state(&self, key: LockKeys) -> LockKeyResult {
        unsafe {
            let mask = xkb_lock_key_mask!(self.handle, key);
            let mut state: XkbStateRec = mem::zeroed();
            XkbGetState(self.handle as *mut _, XkbUseCoreKbd, &mut state);
            Ok(((state.locked_mods as c_uint) & mask != 0).into())
        }
    }
}

impl Drop for LockKey {
    fn drop(&mut self) {
        unsafe { XCloseDisplay(self.handle as *mut _) };
    }
}
