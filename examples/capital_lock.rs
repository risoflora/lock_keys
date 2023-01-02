//! How to toggle the state of the Capital Lock key.

use lock_keys::*;

fn main() {
    let lock_key = LockKey::new();
    lock_key.toggle(LockKeys::CapitalLock).unwrap();
}
