use std::net::SocketAddr;
use crate::NodeRef;

mod find_successor;
mod join;
mod notify;
mod stabilize;
mod check_predecessor;

use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref MTX: Mutex<()> = Mutex::new(());
}

// When a test panics, it will poison the Mutex. Since we don't actually
// care about the state of the data we ignore that it is poisoned and grab
// the lock regardless.  If you just do `let _m = &MTX.lock().unwrap()`, one
// test panicking will cause all other tests that try and acquire a lock on
// that Mutex to also panic.
fn get_lock(m: &'static Mutex<()>) -> MutexGuard<'static, ()> {
    match m.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn node_ref(id: u64) -> NodeRef {
    let addr = SocketAddr::from(([127, 0, 0, 1], 42000 + id as u16));
    NodeRef::with_id(id, addr)
}

