use std::fmt::Debug;

fn quicksort<T: Ord + Debug>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return, // nothing to sort
        2 => { // "base case" / final sort
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return
        },
        _ => {}
    }

    // just pick first element as pivot for simplicity
    let (pivot, rest) = slice.split_first_mut().unwrap();
    let mut left = 0;
    let mut right = rest.len() - 1;

    // grow left and right so that we end up with a vec that
    // looks something like this:
    // [ left | pivot | right ]
    // where left contains elements less (or eq) than pivot and right contains elements greater (or eq) than pivot
    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            right -= 1;
        } else {
            rest.swap(left, right);
            // inc both left & right because right contains
            // an element that is less than pivot and
            // right contains an element greater than pivot
            left += 1;
            right -= 1;
        }
    }

    // swap pivot so that it ends up in "center"
    slice.swap(0, left);

    // sort both partitions (left & right) recursively
    quicksort(&mut slice[..left]);
    quicksort(&mut slice[left + 1..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quicksort_works() {
        let mut v = vec![3,5,2,4,1];
        quicksort(&mut v);
        assert_eq!(v, vec![1,2,3,4,5]);

        let mut v = vec![8, 14, 32, 11, 27, 9, 4, 6, 3];
        quicksort(&mut v);
        assert_eq!(v, vec![3, 4, 6, 8, 9, 11, 14, 27, 32]);
    }

    #[test]
    fn empty_or_one_element() {
        let mut v: Vec<u32> = vec![];
        quicksort(&mut v);
        assert_eq!(v, vec![]);

        let mut v = vec![8];
        quicksort(&mut v);
        assert_eq!(v, vec![8]);
    }
    #[test]
    fn str_sort() {
        let mut v = vec!["oscar", "linde", "a", "b", "x", "d"];
        quicksort(&mut v);
        assert_eq!(v, vec!["a", "b", "d", "linde", "oscar", "x"]);
    }
}
