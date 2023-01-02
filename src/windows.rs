use std::ptr;

use winapi::shared::minwindef::BYTE;
use winapi::um::winuser::{
    keybd_event, GetKeyState, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, VK_CAPITAL, VK_NUMLOCK,
    VK_SCROLL,
};

use crate::{LockKey, LockKeyResult, LockKeyState, LockKeyWrapper, LockKeys};

#[doc(hidden)]
#[macro_export]
macro_rules! lock_key_to_vkkey {
    ($key:expr) => {
        match $key {
            LockKeys::CapitalLock => VK_CAPITAL,
            LockKeys::NumberLock => VK_NUMLOCK,
            LockKeys::ScrollingLock => VK_SCROLL,
        }
    };
}

impl LockKeyWrapper for LockKey {
    /// Creates a new lock key object using [winuser API](https://docs.microsoft.com/en-us/windows/win32/api/winuser) for handling.
    fn new() -> Self {
        LockKey {
            handle: ptr::null_mut(),
        }
    }

    /// Sets a new state for the lock key using [winuser API](https://docs.microsoft.com/en-us/windows/win32/api/winuser).
    fn set(&self, key: LockKeys, state: LockKeyState) -> LockKeyResult {
        unsafe {
            let key = lock_key_to_vkkey!(key) as BYTE;
            keybd_event(key, 0x45, KEYEVENTF_EXTENDEDKEY | 0, 0);
            keybd_event(key, 0x45, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
        }
        Ok(state)
    }

    /// Enables the lock key using [winuser API](https://docs.microsoft.com/en-us/windows/win32/api/winuser).
    fn enable(&self, key: LockKeys) -> LockKeyResult {
        self.set(key, LockKeyState::Enabled)
    }

    /// Disables the lock key using [winuser API](https://docs.microsoft.com/en-us/windows/win32/api/winuser).
    fn disable(&self, key: LockKeys) -> LockKeyResult {
        self.set(key, LockKeyState::Disabled)
    }

    /// Toggles the lock key state returning its previous state using [winuser API](https://docs.microsoft.com/en-us/windows/win32/api/winuser).
    fn toggle(&self, key: LockKeys) -> LockKeyResult {
        let state = self.state(key)?;
        self.set(key, state.toggle())?;
        Ok(state)
    }

    /// Retrieves the lock key state using [winuser API](https://docs.microsoft.com/en-us/windows/win32/api/winuser).
    fn state(&self, key: LockKeys) -> LockKeyResult {
        let key_state = unsafe { GetKeyState(lock_key_to_vkkey!(key)) == 1 };
        Ok(key_state.into())
    }
}
