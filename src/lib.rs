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
    fn merge_helper<T: Ord + Clone>(scratch: &mut Vec<T>, items: &mut [T]) {
        // If our slice is trivially sorted, we can stop recursing.
        if items.len() <= 1 {
            return;
        }

        // 1. Pick a pivot point and split the items into two sub arrays
        let pivot = items.len() / 2;
        let (left, right) = items.split_at_mut(pivot);

        // 2. Recurse to sort the sub arrays as smaller problems
        merge_helper(scratch, left);
        scratch.clear();

        merge_helper(scratch, right);
        scratch.clear();

        // 3. Merge the two sorted sub-arrays using our scratch memory
        for thing in itertools::merge(left, right) {
            scratch.push(thing.clone());
        }

        // 4. Replace the old ordering with the new one
        for (old, new) in items.iter_mut().zip(scratch.iter_mut()) {
            std::mem::swap(old, new);
        }
    }

    // Re-use the scratch buffer across each recurse.
    // We can do this because the entire function is single-threaded, so only
    // a single recurse is using this at once.
    let mut scratch: Vec<T> = Vec::with_capacity(items.len());

    merge_helper(&mut scratch, items);
}

pub fn quick_sort<T: Ord>(items: &mut [T]) {
    /// Quicksort works by partitioning, and then recursing.

    // This helper function picks a pivot point and rearranges `items` so that
    // the pivot point is moved to the correct slot, everything less is on the
    // left, and everything greater is on the right.
    fn partition<T: Ord>(items: &mut [T]) -> usize {
        let pivot: usize = items.len() - 1;
        let mut first_high: usize = 0;

        for i in 0..items.len() {
            if items[i] < items[pivot] {
                items.swap(i, first_high);
                first_high += 1;
            }
        }
        items.swap(pivot, first_high);

        first_high
    }

    if items.len() > 1 {
        let pivot = partition(items);

        let (left, right) = items.split_at_mut(pivot);
        quick_sort(left);
        quick_sort(right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    macro_rules! make_test {
        ($(fn $test_name:ident => $sort_fn:expr),+) => {
            $(
                #[test]
                fn $test_name () {
                    // Poor man's type checking
                    let sort: &dyn Fn(&mut [i32]) = &$sort_fn;
                    let sorted: Vec<i32> = (0..10).collect();

                    // Empty slice should work
                    let mut v: Vec<i32> = vec![];
                    sort(&mut v);
                    assert_eq!(v, &[]);

                    // Trivially sorted list with a single element
                    let mut v: Vec<i32> = vec![1];
                    sort(&mut v);
                    assert_eq!(v, &[1]);

                    // Sometimes lists of two are too much trouble.
                    let mut v: Vec<i32> = vec![2, 1];
                    sort(&mut v);
                    assert_eq!(v, &[1, 2]);

                    let mut v: Vec<i32>  = (0..10).collect();
                    sort(&mut v);
                    assert_eq!(v, sorted);

                    let mut v: Vec<i32>  = (0..10).rev().collect();
                    sort(&mut v);
                    assert_eq!(v, sorted);

                    let mut v: Vec<i32>  = vec![0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
                    sort(&mut v);
                    assert_eq!(v, sorted);

                    // Let's make 10 shuffled arrays and sort each one.
                    for _ in 0..10 {
                        let mut v = sorted.clone();
                        v.shuffle(&mut thread_rng());

                        sort(&mut v);
                        assert_eq!(v, sorted);
                    }
                }
            )+
        }
    }

    make_test! {
        fn check_std_sort => |v| v.sort_unstable(),
        fn check_selection_sort => selection_sort,
        fn check_insertion_sort => insertion_sort,
        fn check_merge_sort => merge_sort,
        fn check_quick_sort => quick_sort
    }
}
