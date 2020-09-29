pub fn selection_sort<T: Ord>(_items: &mut [T]) {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! make_test {
        ($(fn $test_name:ident => $sort_fn:expr),+) => {
            $(
                #[test]
                fn $test_name () {
                    // Poor man's type checking
                    let sort: &dyn Fn(&mut [i32]) = &$sort_fn;
                    let sorted: Vec<i32> = (0..10).collect();

                    let mut v: Vec<i32>  = (0..10).collect();
                    sort(&mut v);
                    assert_eq!(v, sorted);

                    let mut v: Vec<i32>  = (0..10).rev().collect();
                    sort(&mut v);
                    assert_eq!(v, sorted);

                    let mut v: Vec<i32>  = vec![0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
                    sort(&mut v);
                    assert_eq!(v, sorted);
                }
            )+
        }
    }

    make_test! {
        fn check_std => |v| v.sort(),
        fn check_selection => selection_sort
    }
}
