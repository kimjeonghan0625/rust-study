# [Rustnomicon - Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html)

# Subtype

- without subtype
  ```rust
      let mut x = "hello";        // &'static str
      let y = String::new("hi");
      let y = &*y;                // &'a      str
      x = y;                      // this can't work
  ```
- definition: one type can be used another place
  - Sub <: Super : sub is `subtype` of super, sub는 super를 만족한다(포함한다?)
    - 'long $<:$ 'short 'static can `donwgrade` to 'a

# Variance

## problem

```rust
  fn assign<T>(input: &mut T, val: T) {
      *input = val;
  }

  fn main() {
      let mut static_str: &'static str = "hello";
      {
          let a_str = String::from("world");
          assign(&mut static_str, &a_str);
      }
      println!("{static_str}"); // use after free 😿
  }
  // `a_str` does not live long enough
```

- &mut &'static str `should not subtype` of &mut &'a str

## What is Variance

- variance: relationships about subtypes through their generic parameters

  - Sub <: Super인 상황에서

  1. covariant: Generic\<Sub> <: Generic\<Super>, 관계가 유지
  2. contravariant: Generic\<Super> <: Generic\<sub>, 관계가 반대
  3. invariant: no subtype, 관계 없음

- variance hint

  - own pointer(Box\<T>, Vec\<T>): Covariant to T
  - mutabuility 계열(Cell\<T>, UnsafeCell\<T>, \*mut T, &mut T): Invariant to T

## problem explane

```rust
fn assign<T>(input: &mut T, val: T);
...
assign(&mut static_str, &a_str);
```

1. assign function require `input and val` as same type T
2. passed &'static str, &'a str
3. at first arg, &mut T is invariant(type must be same) to T, so T = &'static str
4. at second arg &'a T is covarient(same as inner type), but 'a is not subtype of 'static
5. compile error

## Function pointer

- return value

  - `fn(T) -> U` is covariant to `U`

  ```rust
  fn get_str() -> &'a str
  // this function can return &'static
  ```

- parameter

  - `fn(T)` is contravarient to `T`

  ```rust
    fn _(a: &'static str)
    // this function can't get &'a str

    fn _(a: &'a str)
    // this function can get &'static str
  ```
