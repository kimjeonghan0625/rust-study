## 논의

1. memory 해제는 언제 발생하는가
   - [stackoverflow](https://stackoverflow.com/questions/48227347/does-rust-free-up-the-memory-of-overwritten-variables)
   ```rs
    fn main() {
         let mut a = String::from("hello");
         // 변수가 업데이트 될 때, 이전 값은 메모리에서 해제된다.
         // "hello"라는 값의 String 타입이 업데이트되기 직전에 메모리에서 해제
         a = String::from("world");
         println!("{}", a); // Error!!
    }
   ```
2. `if let`은 `else if` 가 되는가

   - 가능함
   - if let은 기존의 `if` 시스템을 활용해서 구현된건 아닌것 같음.
     - 그럴려면 `let Some(x) = a` 가 boolean expression이어야 하는데, let은 기본적으로 statement임

3. module system
   - module 만들기
     1. ./lib/<module_name>/mod.rs
        - mod.rs: module_name의 entry point
        - <module_name>/\*.rs: module_name의 하위 모듈 구현
     2. ./lib/<module_name>.rs
        - <module_name>.rs: module_name의 entry point
   - module은 `Cargo.toml`에 추가되면 별도의 `use` 없이 사용 가능
     - `use`를 사용하면 해당 모듈을 shortcut으로 사용할 수 있음
     - `pub use`를 사용하면 해당 모듈을 가져오면서도, 해당 모듈을 외부로 보여줄 수 있다
       - 내부 모듈 구조를 숨길 수도 있음(하위의 모듈을 상위에서 pub하면, 중간 구조는 숨길 수 있음)

## 여담

- currying

  - currying maker

    ```js
        const curry =
            (f) =>
            (a, ..._) =>
            _.length ? f(a, ..._) : (..._) => f(a, ...\_);

        const mul = curry((a, b) => a \* b);
        const mul3 = mul(3);
        console.log(mul3(10));
    ```

    - rust로는 currying maker 구현이 쉽지 않다(타입..)

- rxjs
- pure script

## 다음 미팅

- 01.31 21:00: 8,9,10 챕터까지
