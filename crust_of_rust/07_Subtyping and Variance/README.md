[rust nomicon](https://doc.rust-lang.org/nomicon/subtyping.html)
[rust reference](https://doc.rust-lang.org/reference/subtyping.html)

왜 신경을 써야 하는지 알기 위해서 C++의 strtok을 구현

- [c++ strtok reference](https://cplusplus.com/reference/cstring/strtok/)

  - char* strtok(char *str, const char\* delimiter);
  - delimiter와 str을 받아서 delimiter 다음의 string을 반환하고, 기존의 str을 변형시킴

- SubType이란

  - T가 U의 subtype일때, T가 최소한 U만큼 유용할 때
  - 'static은 'a의 subtype

## Variance 종류

- [variance table](https://doc.rust-lang.org/reference/subtyping.html#r-subtyping.variance.builtin-types)

1.  covariance

    - rust에서 대부분은 covariance
    - 다음의 코드가 가능한 이유: 'static이 'a의 subtype이기 때문에

    ```rust
    fn foo(&'a str) {}

    foo(&'a str)
    foo(&'static str)

    lex mut x: &str;
    x = &'a str
    x = &'static str
    ```

2.  contravariance

    - 함수가 요청하는 lifetime보다 더 긴 lifetime을 인자로 넘겨주는 것은 괜찮지만, 짧은 lifetime을 인자로 갖는 함수를 인자로 받는 함수에, 긴 lifetime을 인자로 받는 함수를 넘겨주는 것은 안된다

    ```rust
    fn foo(bar: Fn(&'a str) -> ()) {
    	bar("" /* 'a */);
    }

    // foo에 인자로 넘긴 함수는 'static을 요구하는데,
    // foo의 활용에서 'a를 넘겨주기 때문에 컴파일되면 안됨
    foo(fn(&'static str){});
    ```

    ```rust
    &'static str  // more useful
    &'a str

    Fn(&'static str)  // require more strict lifetime
    Fn(&'a str)  // more useful
    ```

3.  invariance: 주어진 것과 완전히 동일한 것을 제공해야 됨

    ```rust
    fn foo(s: &mut &'a str, x: &'a str) {
    	*s = x;
    }

    let mut x: &'static str = "hello";
    let z = String::new();
    foo(&mut x, &z);
    drop(z);
    // 이 상황에서 x는 'static인데, local string을 가리키는 상황이 됨
    ```

- 왜 &'a mut T가 'a에 대해서 covariant인가

  ```rust
  let x = Box::new(true);
  let x: &'static mut bool  = Box::leak(x);

  let mut y = true;
  let mut z: 'y mut bool = &mut y;

  z = x;  // 'y mut bool = 'static mut bool
  ```

## NonNull

- \*mut T, but non-zero, covariant
  - because of this NonNull<T> can be used at making Covariant type
    - [Explane](https://doc.rust-lang.org/std/ptr/struct.NonNull.html)
    - [NonNull Doc](https://doc.rust-lang.org/std/ptr/struct.NonNull.html)

```rust
pub struct NonNull<T: ?Sized> {
    pointer: *const T,
}

NonNull::new(ptr: \*mut T) -> NonNull<T>
```
