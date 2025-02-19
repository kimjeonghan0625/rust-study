use super::Sorter;

pub struct QuickSort1;
pub struct QuickSort2;

fn quick_sort_1<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            // already on the correct side
            left += 1;
        } else if &rest[right] > pivot {
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            if right == 0 {
                // we must be done
                break;
            }
            right -= 1;
        } else {
            // left holds a right, and right holds a left, swap them.
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                // we must be done
                break;
            }
            right -= 1;
        }
    }

    // re-align left to account for the pivot at 0
    let left = left + 1;

    // place the pivot at its final location
    slice.swap(0, left - 1);

    // split_at_mut(mid: usize) -> (&mut [..mid), &mut [mid..])
    let (left, right) = slice.split_at_mut(left - 1);
    assert!(left.last() <= right.first());
    quick_sort_1(left);
    quick_sort_1(&mut right[1..]);
}

fn pivot<T: Ord>(slice: &mut [T], pivot_index: usize, end_index: usize) -> usize {
    let mut swap_index = pivot_index;
    for i in pivot_index..=end_index {
        if slice[i] < slice[pivot_index] {
            swap_index += 1;
            slice.swap(i, swap_index);
        }
    }
    slice.swap(pivot_index, swap_index);
    swap_index
}

fn quick_sort_2<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }
    let left = 0;
    let right = slice.len() - 1;
    if left < right {
        let pivot_index = pivot(slice, left, right);
        let (l, r) = slice.split_at_mut(pivot_index);
        quick_sort_2(l);
        quick_sort_2(&mut r[1..]);
    }
}

impl Sorter for QuickSort1 {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quick_sort_1(slice);
    }
}

impl Sorter for QuickSort2 {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quick_sort_2(slice);
    }
}

#[test]
fn quick_sort_test() {
    let mut s = [4, 6, 1, 7, 3, 2, 5];
    quick_sort_2(&mut s);
    println!("{s:?}");
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 7])
}

#[test]
fn pivot_test() {
    let mut s = [4, 6, 1, 7, 3, 2, 5];
    assert_eq!(pivot(&mut s, 0, 6), 3);
    println!("{s:?}");
}

#[test]
fn it_works() {
    // quicksort2 logic
    //    l                l
    // [4 2 5 3 1] -> [4 2 5 3 1]
    //          r
    // [4 2 5 3 1]
    //      l   r
    // [4 2 1 3 5]
    //        l                l
    // [4 2 1 3 5] -> [4 2 1 3 5]
    //        r
    // [4 2 1 3 5]
    //        p
    // [3 2 1 4 5]
    let mut things = vec![4, 2, 5, 3, 1];
    QuickSort1.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_sorted_list() {
    let mut things = vec![1, 2, 3, 4, 5];
    QuickSort1.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_reverse_sorted_list() {
    let mut things = vec![5, 4, 3, 2, 1];
    QuickSort1.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_single_element() {
    let mut things = vec![42];
    QuickSort1.sort(&mut things);
    assert_eq!(things, &[42]);
}

#[test]
fn test_two_elements() {
    let mut things = vec![2, 1];
    QuickSort1.sort(&mut things);
    assert_eq!(things, &[1, 2]);
}

#[test]
fn test_duplicates() {
    let mut things = vec![4, 2, 5, 3, 1, 2, 4, 3];
    QuickSort1.sort(&mut things);
    assert_eq!(things, &[1, 2, 2, 3, 3, 4, 4, 5]);
}

#[test]
fn test_large_range() {
    let mut things: Vec<i32> = (1..=1000).rev().collect();
    QuickSort1.sort(&mut things);
    assert_eq!(things, (1..=1000).collect::<Vec<i32>>().as_slice());
}

#[test]
fn test_all_same_elements() {
    let mut things = vec![7, 7, 7, 7, 7];
    QuickSort1.sort(&mut things);
    assert_eq!(things, &[7, 7, 7, 7, 7]);
}

#[test]
fn test_negative_numbers() {
    let mut things = vec![-5, -1, -3, -2, -4];
    QuickSort1.sort(&mut things);
    assert_eq!(things, &[-5, -4, -3, -2, -1]);
}

#[test]
fn test_mixed_numbers() {
    let mut things = vec![3, -1, 4, -2, 0];
    QuickSort1.sort(&mut things);
    assert_eq!(things, &[-2, -1, 0, 3, 4]);
}
