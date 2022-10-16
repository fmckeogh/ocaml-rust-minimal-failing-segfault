#![deny(missing_docs)]

//! Rust interface to `Sail` compiler library

use {
    ocaml::{List, Runtime, ToValue, Value},
    once_cell::sync::Lazy,
    parking_lot::Mutex,
};

/// OCaml runtime handle, initialised on first access
///
/// *Every* function referencing RT must either begin with RT.write() or it will possibly not be
/// initialised and cause a "boxroot is not setup" error. This error will be hard to diagnose as
/// it will be dependent on the order that other (correctly dereferencing RT and thus initialising
/// the runtime) functions are called. Need to investigate how this can be prevented.
static RT: Lazy<Mutex<Runtime>> = Lazy::new(|| Mutex::new(ocaml::runtime::init()));

ocaml::import! {
    fn internal_util_dedup(l: List<Value>) -> List<i32>;
}

/// Removes duplicate values in the supplied Vec
pub fn dedup(list: Vec<i32>) -> Vec<i32> {
    Lazy::force(&RT);

    let mut l = List::empty();

    for element in list {
        let rt = RT.lock();
        l = unsafe { l.add(&*rt, &element.to_value(&*rt)) };
    }

    unsafe { internal_util_dedup(&*RT.lock(), l) }
        .unwrap()
        .into_vec()
}

#[cfg(test)]
mod tests {
    use crate::dedup;

    #[test]
    fn fail() {
        for _ in 0..10 {
            std::thread::spawn(|| loop {
                let i = vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5];
                let o = dedup(i.clone());
                assert_eq!(i, o);
            });
        }
        loop {}
    }
}
