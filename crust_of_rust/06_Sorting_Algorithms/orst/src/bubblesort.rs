use super::Sorter;

pub struct Bubblesort;

impl Sorter for Bubblesort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = [4, 2, 3, 1];
    super::sort::<_, Bubblesort>(&mut things);
    assert_eq!(&things, &[1, 2, 3, 4]);
}
