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

## PhantomData

- `fn() -> T`가 유용한 경우

  - PhantomData<T>로 구성을 하면 T(Oisann<&mut i32>)에 대해서 접근하는 것으로 생각하기 때문에 컴파일이 안됨

    - Deserializer

      ```rust
      struct Deserializer<T> {
        _t: PhantomData<T>
        // _t: PhantomData<fn() -> T>
      }
      Deserializer<Oisann<&mut i32>>
      ```

    - EmptyIterator

      ```rust
      struct EmptyIterator<T> {
        _t: PhantomData<fn() -> T>
      }
      impl<T> Iterator for EmptyIterator {
        type Item = T;
        fn next(&mut self) -> <Option::Item> { None }
      }
      ```

    - [Empty](https://doc.rust-lang.org/std/iter/struct.Empty.html)

      - [영상이랑 지금이랑 코드가 다른디...?](https://youtu.be/TJOFSMpJdzg?si=6fNa3sze731MkyO4&t=3443)
      - [맞게 이해한게 맞나?](https://youtu.be/TJOFSMpJdzg?si=MepVFH2DHM00MPxT&t=4053)

      ```rust
      use std::iter::Empty;
      // struct Empty<T>(marker::PhantomData<T>);

      use std::marker;
      // Empty struct at that time(using PhantomData<T>, currently PhantomData<fn()->T>)
      struct Empty<T>(marker::PhantomData<T>);
      impl<T> Iterator for Empty<T> {
          type Item = T;

          fn next(&mut self) -> Option<T> { None }
      }
      impl<T> Default for Empty<T> {
          fn default() -> Empty<T> {
              Empty(marker::PhantomData)
          }
      }

      let mut a = 42;
      let mut it: Empty<Oisann<&mut i32>> = Empty::default();
      let mut o: Option<Oisann<&mut i32>> = Some(Oisann(&mut a));
      {
        // ...&'a mut i32 = ...&'static mut i32
        o = it.next();
        // make it Iterate over <Oisann<&mut i32>>
        // make drop(it) touch &mut
        // but Empty doesn't impl Drop, so it compile
        // 근데 영상에서 PhatomData<T>는 T를 가진것으로 판단해서 drop check가 발생해야 된다고 생각
        // drop check는 발생하는데, it의 Oisaan<&mut i32>는 &'static이기 때문에 컴파일 가능
      }
      drop(o);
      println!("{:?}", a);
      let _ = it.next();
      ```
