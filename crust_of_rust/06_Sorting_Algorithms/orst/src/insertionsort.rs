use super::Sorter;

pub struct InsertionSort {
    smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] and place in sorted location in slice[..=unsorted]
            // [1 3 4 | 2]
            // [1 3 4 2 | ]
            // [1 3 2 4 | ]
            // [1 2 3 4 | ]
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // unsorted 이전까지는 이전 과정에서 정렬되었으므로,
                // unsorted를 어디에 삽입할 지 결정하는 과정이 필요.
                // binary search를 사용하면 O(n) -> O(logn)으로 시간복잡도를 개선할 수 있다.

                // slice 타입에 정의된 binary_search 메서드는
                // 찾는 요소가 있으면 정확히 그 요소의 인덱스를 Ok에 담아 반환한다.
                // 찾는 요소가 없으면 찾는 요소가 삽입된다면 들어가야 하는 인덱스를 Err에 담아 반환한다.
                // 예제 코드
                // let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
                // assert_eq!(s.binary_search(&13),  Ok(9));
                // assert_eq!(s.binary_search(&4),   Err(7));

                // let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                //     Ok(i) => i,
                //     Err(i) => i,
                // };

                let i = slice[..unsorted]
                    .binary_search(&slice[unsorted])
                    .unwrap_or_else(|i| i);

                // slice 타입에 정의된 rotate_right 메서드는
                // 인수로 받은 수만큼 오른쪽으로 밀어낸 것과 같은 변화를 만들어낸다.
                // 밀려난 것들은 앞으로 이동한다.
                slice[i..=unsorted].rotate_right(1); // ex) [1 2 4 5 | 3] -> [4 5 3] -> [3 4 5]
            }
        }
    }
}

#[test]
fn not_smart() {
    let mut things = [4, 2, 3, 1];
    InsertionSort { smart: false }.sort(&mut things);
    assert_eq!(&things, &[1, 2, 3, 4]);
}

#[test]
fn smart() {
    let mut things = [4, 2, 3, 1];
    InsertionSort { smart: true }.sort(&mut things);
    assert_eq!(&things, &[1, 2, 3, 4]);
}
