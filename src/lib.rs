pub fn selection_sort<T: Ord>(mut items: &mut [T]) {
    // Find the smallest element left in our (shrinking) items
    while let Some((min, _elem)) = items.iter().enumerate().min_by_key(|(_i, k)| *k) {
        // Place it at the front
        // This is where it belongs in the final sorted list, because it's
        // the smallest element in our list now. Everything smaller is outside
        // of "items", in the part that we lobbed off.
        items.swap(0, min);

        // and then lob off the freshly sorted item from our list.
        items = &mut items[1..];
    }
}

pub fn insertion_sort<T: Ord>(items: &mut [T]) {
    // Walk the list, leaving everything on the left sorted.
    // We start with a "sorted" list of 1 element, which is trivially sorted.
    for i in 1..items.len() {
        // All elements < i are sorted, but slot i is not.
        // We split one past i to introduce our unsorted element into `sorted`.
        let (sorted, _) = items.split_at_mut(i + 1);

        // And then we walk backwards in sorted, until our element is in place
        for j in (1..sorted.len()).rev() {
            if sorted[j] < sorted[j - 1] {
                // If we're not sorted, move it down and continue
                sorted.swap(j, j - 1);
            } else {
                // If we are sorted, we're done!
                break;
            }
        }
    }
}

pub fn merge_sort<T: Ord + Clone>(items: &mut [T]) {
    // If our slice is trivially sorted, we can stop recursing.
    if items.len() == 1 {
        return;
    }

    // 1. Pick a pivot point and split the items into two sub arrays
    let items_len = items.len();
    let pivot = items.len() / 2;
    let (left, right) = items.split_at_mut(pivot);

    // 2. Recurse to sort the sub arrays as smaller problems
    merge_sort(left);
    merge_sort(right);

    // 3. Merge the two sorted sub-arrays using our scratch memory
    let mut scratch: Vec<T> = Vec::with_capacity(items_len);
    for thing in itertools::merge(left, right) {
        scratch.push(thing.clone());
    }

    // 4. Replace the old ordering with the new one
    for (old, new) in items.iter_mut().zip(scratch.into_iter()) {
        *old = new;
    }
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
        fn check_std_sort => |v| v.sort(),
        fn check_selection_sort => selection_sort,
        fn check_insertion_sort => insertion_sort,
        fn check_merge_sort => merge_sort
    }
}
