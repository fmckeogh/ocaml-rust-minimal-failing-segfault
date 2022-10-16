use ocaml::{List, ToValue, Value};

ocaml::import! {
    fn internal_util_dedup(l: List<Value>) -> List<i32>;
}

/// Removes duplicate values in the supplied Vec
pub fn dedup(list: Vec<i32>) -> Vec<i32> {
    let rt = ocaml::runtime::init();

    let mut l = List::empty();

    for element in list {
        l = unsafe { l.add(&rt, &element.to_value(&rt)) };
    }

    unsafe { internal_util_dedup(&rt, l) }.unwrap().into_vec()
}

#[cfg(test)]
mod tests {
    use crate::dedup;

    #[test]
    fn fail() {
        for _ in 0..100 {
            std::thread::spawn(|| {
                let i = vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5];
                let o = dedup(i.clone());
                assert_eq!(i, o);
            });
        }
        loop {}
    }
}
