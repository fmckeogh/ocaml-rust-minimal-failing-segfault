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
    use {
        crate::dedup,
        proptest::{bits, collection::vec, prelude::*},
    };

    proptest! {
        /// Checks equivalence between libsail dedup function and Rust stdlib dedup.
        ///
        /// Used as smoke test that OCaml interop is functioning correctly (intentionally doing a lot of allocating, many function calls, etc).
        #[test]
        fn smoke_test0(v in vec(bits::i32::ANY, 0..10000)) {
            let mut v_d = v.clone();
            v_d.sort();
            v_d.dedup();

            let mut out = dedup(v);
            out.sort();
            assert_eq!(out, v_d);
        }
    }

    proptest! {
        /// Checks equivalence between libsail dedup function and Rust stdlib dedup.
        ///
        /// Used as smoke test that OCaml interop is functioning correctly (intentionally doing a lot of allocating, many function calls, etc).
        #[test]
        fn smoke_test1(v in vec(bits::i32::ANY, 0..10000)) {
            let mut v_d = v.clone();
            v_d.sort();
            v_d.dedup();

            let mut out = dedup(v);
            out.sort();
            assert_eq!(out, v_d);
        }
    }

    proptest! {
        /// Checks equivalence between libsail dedup function and Rust stdlib dedup.
        ///
        /// Used as smoke test that OCaml interop is functioning correctly (intentionally doing a lot of allocating, many function calls, etc).
        #[test]
        fn smoke_test2(v in vec(bits::i32::ANY, 0..10000)) {
            let mut v_d = v.clone();
            v_d.sort();
            v_d.dedup();

            let mut out = dedup(v);
            out.sort();
            assert_eq!(out, v_d);
        }
    }

    proptest! {
        /// Checks equivalence between libsail dedup function and Rust stdlib dedup.
        ///
        /// Used as smoke test that OCaml interop is functioning correctly (intentionally doing a lot of allocating, many function calls, etc).
        #[test]
        fn smoke_test3(v in vec(bits::i32::ANY, 0..10000)) {
            let mut v_d = v.clone();
            v_d.sort();
            v_d.dedup();

            let mut out = dedup(v);
            out.sort();
            assert_eq!(out, v_d);
        }
    }
}
