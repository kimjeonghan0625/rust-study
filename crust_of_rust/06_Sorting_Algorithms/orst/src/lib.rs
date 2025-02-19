pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

pub mod bubblesort;
pub mod insertionsort;
pub mod quicksort;
pub mod selectionsort;

pub use bubblesort::Bubblesort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort1;
pub use quicksort::QuickSort2;
pub use selectionsort::SelectionSort;
pub struct StdSort;

impl Sorter for StdSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSort.sort(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4]);
    }
}
