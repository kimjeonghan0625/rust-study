# [Nomicon Drop Check](https://doc.rust-lang.org/nomicon/dropck.html)

- variables are dropped reverse order of definition

## Dropcheck Rule

- 제네릭 타입이 Drop을 구현하면, T는 명확히 오래 살아야 한다
- 아직 정확한 규칙을 찾는 중

## Escape Hatch

- may_dangle: compiler `guaranteed` to compiler not to access expired data

  - unstable feature

  ```rust
  #![feature(dropck_eyepatch)]

  unsafe impl<#[may_danlge] T> Some for OtherSome{}
  ```

## Edge Case

```rust
// function call might access T when drop StructA
struct StructA<T>(T, Box<for <'r> fn(&'r T) -> String>);

// trait method call
// println! will call <T as Display>::fmt
struct StructB<T: fmt::Display>(T);
```
