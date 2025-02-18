use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for unsorted in 0..slice.len() {
            // let smallest_in_rest = slice[unsorted..]
            //     .iter()
            //     .enumerate()
            //     .min_by_key(|&(_, b)| b)
            //     .map(|(i, _)| unsorted + i)
            //     .expect("slice is non-empty"); // 비어 있는 슬라이스에 대해서는 for 문이 돌지 않음

            let mut smallest_in_rest = unsorted;
            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallest_in_rest] {
                    smallest_in_rest = i;
                }
            }

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[test]
fn it_works() {
    // [1, 2, 5, 3, 4] -> [1, 2, 5, 3, 4] -> [1, 2, 3, 5, 4] -> [1, 2, 3, 4, 5] -> [1, 2, 3, 4, 5]
    let mut things = vec![4, 2, 5, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
