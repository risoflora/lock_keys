//! `lock_keys` provides a cross platform way for lock keys handling.
//!
//! Supported platforms: Linux ([Xlib](https://en.wikipedia.org/wiki/Xlib) static) and Windows ([winuser API](https://docs.microsoft.com/en-us/windows/win32/api/winuser)).
//!
//! # Example
//!
//! The example below shows how to turn on the Number Lock key:
//!
//! ```rust
//! extern crate lock_keys;
//!
//! use lock_keys::*;
//!
//! fn main() {
//!     let lockkey = LockKey::new();
//!     lockkey.enable(LockKeys::NumberLock).unwrap();
//! }
//! ```

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

use std::fmt;
use std::io;

#[doc(hidden)]
enum LockKeyHandle {}

// Indicates the lock key state, i.e. enabled/disabled.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LockKeyState {
    Enabled,
    Disabled,
}

impl From<bool> for LockKeyState {
    fn from(val: bool) -> Self {
        if val {
            LockKeyState::Enabled
        } else {
            LockKeyState::Disabled
        }
    }
}

impl From<LockKeyState> for bool {
    fn from(val: LockKeyState) -> Self {
        val == LockKeyState::Enabled
    }
}

impl fmt::Display for LockKeyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            LockKeyState::Enabled => "enabled",
            LockKeyState::Disabled => "disabled",
        })
    }
}

/// A specialized `Result` type lock key handling.
pub type LockKeyResult = io::Result<LockKeyState>;

/// The available lock keys for handling, i.e. Capital Lock, Number Lock and Scrolling Lock.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LockKeys {
    CapitalLock,
    NumberLock,
    ScrollingLock,
}

/// The lock ley object to hold the OS specific handle when it is required.
pub struct LockKey {
    handle: *mut LockKeyHandle,
}

impl fmt::Debug for LockKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.handle, f)
    }
}

/// A collection of methods that are required for lock key handling.
pub trait LockKeyWrapper {
    /// Creates a new lock key object.
    fn new() -> Self;
    /// Sets a new state for the lock key.
    fn set(&self, key: LockKeys, state: LockKeyState) -> LockKeyResult;
    /// Enables the lock key.
    fn enable(&self, key: LockKeys) -> LockKeyResult;
    /// Disables the lock key.
    fn disable(&self, key: LockKeys) -> LockKeyResult;
    /// Toggles the lock key state returning its previous one.
    fn toggle(&self, key: LockKeys) -> LockKeyResult;
    /// Retrieves the lock key state.
    fn state(&self, key: LockKeys) -> LockKeyResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set() {
        let lockkey = LockKey::new();
        let old_lockkey_state = lockkey.state(LockKeys::NumberLock).unwrap();
        assert_eq!(
            lockkey
                .set(LockKeys::NumberLock, LockKeyState::Disabled)
                .unwrap(),
            LockKeyState::Disabled
        );
        assert_eq!(
            lockkey.state(LockKeys::NumberLock).unwrap(),
            LockKeyState::Disabled
        );
        assert_eq!(
            lockkey
                .set(LockKeys::NumberLock, LockKeyState::Enabled)
                .unwrap(),
            LockKeyState::Enabled
        );
        assert_eq!(
            lockkey.state(LockKeys::NumberLock).unwrap(),
            LockKeyState::Enabled
        );
        lockkey
            .set(LockKeys::NumberLock, old_lockkey_state)
            .unwrap();
    }

    #[test]
    fn enable() {
        let lockkey = LockKey::new();
        let old_lockkey_state = lockkey.state(LockKeys::CapitalLock).unwrap();
        lockkey.disable(LockKeys::CapitalLock).unwrap();
        assert_eq!(
            lockkey.state(LockKeys::CapitalLock).unwrap(),
            LockKeyState::Disabled
        );
        assert_eq!(
            lockkey.enable(LockKeys::CapitalLock).unwrap(),
            LockKeyState::Enabled
        );
        assert_eq!(
            lockkey.state(LockKeys::CapitalLock).unwrap(),
            LockKeyState::Enabled
        );
        lockkey
            .set(LockKeys::CapitalLock, old_lockkey_state)
            .unwrap();
    }

    #[test]
    fn disable() {
        let lockkey = LockKey::new();
        let old_lockkey_state = lockkey.state(LockKeys::CapitalLock).unwrap();
        lockkey.enable(LockKeys::CapitalLock).unwrap();
        assert_eq!(
            lockkey.state(LockKeys::CapitalLock).unwrap(),
            LockKeyState::Enabled
        );
        assert_eq!(
            lockkey.disable(LockKeys::CapitalLock).unwrap(),
            LockKeyState::Disabled
        );
        assert_eq!(
            lockkey.state(LockKeys::CapitalLock).unwrap(),
            LockKeyState::Disabled
        );
        lockkey
            .set(LockKeys::CapitalLock, old_lockkey_state)
            .unwrap();
    }

    #[test]
    fn toggle() {
        let lockkey = LockKey::new();
        let old_lockkey_state = lockkey.state(LockKeys::ScrollingLock).unwrap();
        lockkey.enable(LockKeys::ScrollingLock).unwrap();
        assert_eq!(
            lockkey.toggle(LockKeys::ScrollingLock).unwrap(),
            LockKeyState::Enabled
        );
        assert_eq!(
            lockkey.toggle(LockKeys::ScrollingLock).unwrap(),
            LockKeyState::Disabled
        );
        lockkey
            .set(LockKeys::ScrollingLock, old_lockkey_state)
            .unwrap();
    }

    #[test]
    fn state() {
        let lockkey = LockKey::new();
        let old_lockkey_state = lockkey.state(LockKeys::NumberLock).unwrap();
        lockkey.enable(LockKeys::NumberLock).unwrap();
        assert_eq!(
            lockkey.state(LockKeys::NumberLock).unwrap(),
            LockKeyState::Enabled
        );
        lockkey.disable(LockKeys::NumberLock).unwrap();
        assert_eq!(
            lockkey.toggle(LockKeys::NumberLock).unwrap(),
            LockKeyState::Disabled
        );
        lockkey
            .set(LockKeys::ScrollingLock, old_lockkey_state)
            .unwrap();
    }
}
