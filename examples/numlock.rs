//! How to turn on the Number Lock key.

extern crate lock_keys;

use lock_keys::*;

fn main() {
    let lockkey = LockKey::new();
    lockkey.enable(LockKeys::NumberLock).unwrap();
}
