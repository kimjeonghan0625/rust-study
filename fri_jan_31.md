## 기타

- [The Dark Art of Unsafe Rust](https://doc.rust-lang.org/nomicon/intro.html)
- [Rust-Lang Cheat Sheet](https://cheats.rs/)
- mil(mid-level intermediate language)로 컴파일 하기
  - `rustc --emit=mir <source code>`

## 논의

1. str type

   - str type은 string slice를 의미한다
     - str에 slice 연산을 중복으로 가해도, str type이 유지된다
   - &str <-(&)- str(== string slice) <-(\*)- String <-(\*)- &String
     - &str: {ptr, len}
     - str: [u8], 길이 정보가 없는 slice
     - String: {ptr, len, capacity}
     - &String: {ptr}
   - 자동 defreferencing
     - &String에 slice(str에 대한 연산)연산이 가능한 이유

2. varient에 data는 어떻게 구현되는가

   - [`tag`와 `data`로 구성](https://cheats.rs/#custom-types)

3. hashmap

   - entry: key가 map에 있는지를 확인하는 함수
   - and_modify & or_insert_with으로 값이 있을 때, 없을때를 모두 처리 가능

     ```rust
        hashmap.entry(key).and_modify(|e| *e += 1).or_insert(1);

        // == javascript
        // hashmap.map(() => {
        //    if (hashmap[key]) hashmap[key] += 1;
        //    else hashmap[key] = 1;
        // })
     ```

4. trait

   - `기본 구현`에서 `비 기본 구현`을 호출할 수 있다
     - `Vec<T>`에서 `.next`만 구현하면, vector에서 제공하는 다른 모든 메서드를 사용할 수 있다
     - rust 컴파일러가 구현을 강제하기 때문에, 기본 구현에서 비 기본 구현을 호출 할 시점에는 해당 메서드가 구현되어 있다
   - 매개변수로써 trait
     - java의 interface와 비슷하지만, java는 runtime에서 동적으로 결정되는 반면, rust는 compile time에 결정된다
   - 반환값으로써 trait
     - 실제 반환되는 타입은 하나만 가능하다(trait를 구현하는 타입들 반환 불가)
     - Iterator 같이 장황하거나, 사용이 불가능한 타입을 반환하는 경우, 함수의 signature를 간결하게 작성 가능

5. 라이프타입

   - lifetime을 명시하면 안되는 경우도 있다

     ```rust
     // 정한이 코드
     ```

6. Drop은 언제 발생하는가

   1. 변수가 블럭을 벗어날 때
      ```rust
      {
        let a = String::from("hello");
        // 블럭을 벗어나기 직전에 drop
      }
      ```
   2. 소유권의 이전이 발생한 시점

      ```rust
        let mut a = String::from("hello");  // 이건 됨
        // "world"의 소유권이 a로 이전했기 때문에
        // "hello"에 대한 메모리는 drop된다
        a = String::from("world");  // 이건 안됨
      ```

      ```rust
        let a: &mut String = &mut String::from("hello");
        // "world"의 소유권이 a로 이전했기 때문에
        // 기존의 "hello"에 대한 메모리는 drop된다
        *a = String::from("world");
      ```

   - 소유권이 없는 참조 일때

     - let을 활용한 변수 선언 일 때, 내부적으로 임시변수를 생성해 소유권 규칙을 적용한다.
     - 변수의 선언이 아닐 경우, 즉시 drop

     ```rust
      // 변수 선언이기 때문에, 임시변수가 생성되어 "hello"의 소유권을 가진다
      let mut a = &String::from("hello");
      // 변수의 선언이 아니기 때문에 "world"는 즉시 drop된다
      a = &String::from("world");

      // 다음을 주석 해제하면 컴파일 에러
      // drop이 된 "world"에 대한 참조를 시도하기 때문
      // println!("{a}");
     ```
