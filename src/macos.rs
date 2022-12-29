use std::{
    io::{Error, ErrorKind},
    os::raw::{c_char, c_int, c_uint},
};

use core_foundation::base::{CFRelease, CFTypeRef};
use io_kit_sys::{
    kIOMasterPortDefault, types::io_connect_t, IOObjectRelease, IOServiceClose,
    IOServiceGetMatchingService, IOServiceMatching, IOServiceOpen,
};
use mach::{
    kern_return::{kern_return_t, KERN_SUCCESS},
    traps::mach_task_self,
};

use crate::{LockKey, LockKeyHandle, LockKeyResult, LockKeyState, LockKeyWrapper, LockKeys};

#[allow(non_upper_case_globals)]
const kIOHIDSystemClass: &[u8; 12] = b"IOHIDSystem\0";
#[allow(non_upper_case_globals)]
const kIOHIDParamConnectType: c_uint = 1;
#[allow(non_upper_case_globals)]
const kIOHIDCapsLockState: c_int = 0x00000001;
#[allow(non_upper_case_globals)]
const kIOHIDNumLockState: c_int = 0x00000002;

extern "C" {
    #[doc(hidden)]
    pub fn IOHIDSetModifierLockState(
        handle: io_connect_t,
        selector: c_int,
        state: bool,
    ) -> kern_return_t;
    #[doc(hidden)]
    pub fn IOHIDGetModifierLockState(
        handle: io_connect_t,
        selector: c_int,
        state: &bool,
    ) -> kern_return_t;
}

#[doc(hidden)]
#[macro_export]
macro_rules! io_kit_raise_error {
    ($ident:expr) => {
        Error::new(ErrorKind::Other, $ident)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! io_kit_check_modifier_lock_state {
    ($ident:expr) => {
        if $ident != KERN_SUCCESS {
            return Err(io_kit_raise_error!("IOHIDModifierLockState"));
        }
    };
}

impl LockKeyWrapper for LockKey {
    /// Creates a new lock key object using [IOKit](https://developer.apple.com/documentation/iokit) for handling.
    fn new() -> Self {
        let mut ioc: io_connect_t = 0;
        unsafe {
            let mdict = IOServiceMatching(kIOHIDSystemClass.as_ptr() as *const c_char);
            let ios = IOServiceGetMatchingService(kIOMasterPortDefault, mdict);
            if ios == 0 && !mdict.is_null() {
                CFRelease(mdict as CFTypeRef);
                panic!("IOServiceGetMatchingService() failed");
            }
            let kr = IOServiceOpen(ios, mach_task_self(), kIOHIDParamConnectType, &mut ioc);
            IOObjectRelease(ios);
            if kr != KERN_SUCCESS {
                panic!("IOServiceOpen() failed");
            }
        }
        LockKey {
            handle: ioc as *mut LockKeyHandle,
        }
    }

    /// Sets a new state for the lock key using [IOKit](https://developer.apple.com/documentation/iokit).
    fn set(&self, key: LockKeys, state: LockKeyState) -> LockKeyResult {
        io_kit_check_modifier_lock_state!(unsafe {
            IOHIDSetModifierLockState(self.handle as io_connect_t, key.into(), state.into())
        });
        Ok(state)
    }

    /// Enables the lock key using [IOKit](https://developer.apple.com/documentation/iokit).
    fn enable(&self, key: LockKeys) -> LockKeyResult {
        self.set(key, LockKeyState::Enabled)
    }

    /// Disables the lock key using [IOKit](https://developer.apple.com/documentation/iokit).
    fn disable(&self, key: LockKeys) -> LockKeyResult {
        self.set(key, LockKeyState::Disabled)
    }

    /// Toggles the lock key state returning its previous state using [IOKit](https://developer.apple.com/documentation/iokit).
    fn toggle(&self, key: LockKeys) -> LockKeyResult {
        let state = self.state(key)?;
        self.set(key, state.toggle())?;
        Ok(state)
    }

    /// Retrieves the lock key state using [IOKit](https://developer.apple.com/documentation/iokit).
    fn state(&self, key: LockKeys) -> LockKeyResult {
        let state: bool = false;
        io_kit_check_modifier_lock_state!(unsafe {
            IOHIDGetModifierLockState(self.handle as io_connect_t, key.into(), &state)
        });
        Ok(state.into())
    }
}

impl From<LockKeys> for c_int {
    fn from(val: LockKeys) -> Self {
        match val {
            LockKeys::CapitalLock => kIOHIDCapsLockState,
            LockKeys::NumberLock => kIOHIDNumLockState,
            LockKeys::ScrollingLock => todo!(),
        }
    }
}

impl Drop for LockKey {
    fn drop(&mut self) {
        unsafe { IOServiceClose(self.handle as io_connect_t) };
    }
}
