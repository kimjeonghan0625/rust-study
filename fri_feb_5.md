## 정리

1. Binary Search Tree

2. rust enum monad

   - monad: map, flatmap 를 구현한것
   - 확인해본 결과, rust의 Option은 map 뿐만 아니라, flatten이라는 메서드를 구현하고 있다.
   - flatten 메서드는 `Option<Option<T>>` 타입을 `Option<T>` 타입으로 풀어준다.
   - 다만, Iterator에 구현된 flat_map이건 flatten이건 Option 타입에 구현된 flatten이건, One-level depth에 대해서만 풀어준다.
   - 재귀적으로 중첩된 모든 것을 풀어헤치는 것이 아니다.
   - 이러한 구현은 자바스크립트 배열 메서드 flatMap 도 동일하게 One-level depth를 풀어헤치는 것으로 확인했다.
   - 다만 하나를 푸는 메서드를 통해 모든 depth를 푸는 메서드는 비교적 쉽게 구현해볼 수 있을 듯.

3. derive vs trait

   - [reference](https://www.reddit.com/r/rust/comments/h8bpj6/a_very_basic_question_in_regard_to_derive_and_impl/?rdt=65438)

   - derive: 컴파일러가 구현, `#[derive(Debug)]`
   - trait: 직접 구현, `impl`

4. 포인터란

   - 소유권: drop을 호출할 책임 & 권한이 있는것

   - 포인터: 저장된 값을 가리킴, `dref, drop`을 구현
     - 참조: 소유권이 없는 포인터, 컴파일러가 가리키는 값이 항상 유효함을 보장, `&, &mut`
       - 약한 참조: 가리키는 값이 항상 유효하지 않을 수 있음(Option<T>)
     - 스마트 포인터: 포인터 + 메타데이터 + 소유권, `String, Vec, Box ...`

5. lazy evaluation

   - map, filter => 반복 어댑터
   - iterator를 생성 후, 소비하기 전까지는 어떠한 연산도 하지않음

     ```rust
       let v = (0..)  // 0부터 무한대까지 범위
         .filter(|x| x % 2 == 0)
         .take(2)
         .collect::<Vec<i32>>();
        // collect, take를 통해 실제로 2개만 소비하기 때문에
        // filter에서 2개가 반환될때 까지만 연산을 수행

       println!("{:?}", v); // [0, 2]
     ```

   - javascirpt는 배열 method등은 lazy 하지 않음
     - [generator function을](https://ko.javascript.info/generators) 활용해 lazy evaluation을 구현할 수 있음

6. for문이 내부적으로 iterator(into_iter)를 호출한다

   - for문은 내부적으로 into_iter를 통해 iterator를 구현한 타입으로 변환하고, 순회한다.
     - 다음의 `let, while let` 문은 바로 다음의 `for`문과 동일하다

   ```rust
       let v = vec![1,2,3];

       let mut iter = (&v).into_iter();
       while let Some(val) = iter.next() {}
       //        ∧
       //        |(같음)
       //        ∨
       for val in &v {}

       // ------------------------------------

       let mut iter = (v.iter()).into_iter();
       while let Some(val) = iter.next() {}
       //        ∧
       //        |(같음)
       //        ∨
       for val in v.iter() {}

       // ------------------------------------

       let mut iter = v.into_iter();
       while let Some(val) = iter.next() {}
       //        ∧
       //        |(같음)
       //        ∨
       for val in v {}
   ```

## 질문

### 창주

- javascript에서 배열은 기본적으로 iterateable 이거 조금더 설명
- collection data type이란?
  - 그냥 유용한거 모아놓은 거인듯?
  - [reference](https://doc.rust-lang.org/std/collections/index.html)
- .map 함수의 구현, `.iter().map(|x| x + 1)` 이런 식으로 쓰던데, 여기서 x는 Iterator type인지, 아니면 내부적으로 value?를 가져와서 해당 value를 넣는 것인지
  - iter는 vec이 아니라, slice에 선언된 함수
- 책 p.344에서 Counter에 대해서 Iterator를 구현 할 때, next의 return이 Self::Item인데, 여기서 어떻게 다시 .next를 호출 할 수 있는지
  - 이건 연관 타입이랑 관련 있는건가?
  - 이게 아닌가? iterator 변수는 그대로 있고, next의 결과랑은 별개이구만
  - vec<i32> 같은건 어떻게 구현되어 있어서, 다음값을 가져올 수 있는거지
- 책 p.389 첫줄
  - deref 메서드가 값에 대한 참조를 리턴하는 이유는 소유권 시스템이 \*(y.deref()) 구문에서 `괄호 바깥의 역참조를 요구`하기 때문이다.
  - 참조가 아니라 값을 직접 리턴하면 리턴된 값이 `self 참조`로 이동해 버린다.
    - 무슨 self 참조?
- mutex p.439 println!은 왜 lock을 안걸로 값에 접근할 수 있는지
