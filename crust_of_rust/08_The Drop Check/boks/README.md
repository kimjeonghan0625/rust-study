# drop check

- drop을 할때, 안쪽에 있는 어떤 것을 drop을 해야하는지 체크 하는것?

## 컴파일러의 가정

- 제네릭 T가 Drop을 구현하면 drop 함수에서 T를 사용 하는 것으로 가정(실제로 사용하는 것과는 별개)
  ```rust
  let b = Boks::ny(&mut y);
  println!("{}", y);
  // drop(b)
  ```
  - 해당 코드에 T는 &mut i32, 즉 drop에서 &mut을 사용
  - 그래서 Drop impl을 없애버리면 컴파일은 통과

# 기타

- [drop을 구현하고 있으면, 내부의 요소를 부분적으로 밖으로 move 할 수 없다. 왜냐하면 drop에서 &mut을 사용하기 때문에](https://youtu.be/TJOFSMpJdzg?si=UoFQ5WzwHdywtHcj&t=1061)
