use std::ptr;

use winapi::shared::minwindef::BYTE;
use winapi::um::winuser::{
    keybd_event, GetKeyState, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, VK_CAPITAL, VK_NUMLOCK,
    VK_SCROLL,
};

use {LockKey, LockKeyResult, LockKeyState, LockKeyWrapper, LockKeys};

#[doc(hidden)]
#[macro_export]
macro_rules! lockkey_to_vkkey {
    ($key:expr) => {
        match $key {
            LockKeys::CapitalLock => VK_CAPITAL,
            LockKeys::NumberLock => VK_NUMLOCK,
            LockKeys::ScrollingLock => VK_SCROLL,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! vk_state {
    ($key:expr) => {
        GetKeyState(lockkey_to_vkkey!($key)) == 1
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! send_vkkey {
    ($key:expr) => {
        let key = lockkey_to_vkkey!($key) as BYTE;
        keybd_event(key, 0x45, KEYEVENTF_EXTENDEDKEY | 0, 0);
        keybd_event(key, 0x45, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
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
            let key_state = vk_state!(key);
            if key_state != state.into() {
                send_vkkey!(key);
            }
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
        unsafe {
            let old_key_state = vk_state!(key);
            send_vkkey!(key);
            Ok(old_key_state.into())
        }
    }

    /// Retrieves the lock key state using [winuser API](https://docs.microsoft.com/en-us/windows/win32/api/winuser).
    fn state(&self, key: LockKeys) -> LockKeyResult {
        let key_state = unsafe { GetKeyState(lockkey_to_vkkey!(key)) == 1 };
        Ok(key_state.into())
    }
}
