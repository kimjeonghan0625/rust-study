## 함수 아이템 vs 함수 포인터
- intro
    ```rust
    fn main() {
        // 아래 x의 타입이 뭘까?
        // 함수 포인터일까?
        // 그렇게 생각할 수 있지만 함수 포인터가 아니라 함수 "아이템"임
        // zero-sized value <- 컴파일 타임에만 분석되는 무언가 라는 것임
        // 이 둘은 꽤 다르다.
        let x = bar;
        assert_eq!(std::mem::size_of_val(&x), 0);
    }

    fn bar() {}
    ```
- 함수 아이템은 함수 포인터로 강요(coerce) 될 수 있다.
  - array 타입이 slice 타입으로 강요될 수 있는 것과 비슷한가?
  - 아래 코드를 보면,
  함수 아이템이 함수 포인터 타입의 매개변수 자리에 인수로 공급될 때,
  그제서야 물리 메모리 공간을 갖는 개체로 인스턴스화(instantiation) 되는 것으로 보인다.
    ```rust
    fn main() {
        // ...
        baz(bar::<u32>);
        baz(bar::<i32>);
    }

    fn bar<T>(_: u32) -> u32 {
        0
    }

    fn baz(f: fn(u32) -> u32) {}
    ```
  - 호출하기 전에는 함수 body를 생성하지 않는다.
## 클로저 트레이트
- 함수 포인터는 Fn, FnMut, FnOnce를 모두 구현한다.
따라서 어떤 클로저 트레이트를 구현한 타입에도 대입될 수 있음.
  ```rust
  // &mut self -> &self 는 쉽게 가능하기 때문에,
  // Fn을 구현하는 어떤 타입 F는 FnMut도 구현한다.
  impl<F> FnMut() for F
  where
      F: Fn(),
  {
      fn call(&mut self) {
          Fn::call(&*self)
      }
  }

  // 마찬가지 논리로, self -> &mut self는 쉽게 가능하기에,
  // FnMut를 구현하는 어떤 타입 F는 FnOnce도 구현한다.
  impl<F> FnOnce() for F
  where
      F: FnMut(),
  {
      fn call(mut self) {
          FnMut::call(&mut self)
      }
  }

  // 마찬가지 논리로, self -> &self는 쉽게 가능하기에,
  // Fn을 구현하는 어떤 타입 F는 FnOnce도 구현한다.
  impl<F> FnOnce() for F
  where
      F: Fn(),
  {
      fn call(self) {
          Fn::call(&self)
      }
  }

  // 그러나,
  // &self -> self, &self -> &mut self로 전환은 불가능하기에
  // FnOnce 또는 FnMut를 구현한 타입이라고 해서 Fn을 구현할 수 없다.
  // 또한
  // &mut self -> self로 전환도 불가능하기에
  // FnOnce를 구현한 타입이라고 해서 FnMut를 구현할 수는 없다.

  // 결론,
  // Fn을 구현하면 자동으로 FnMut, FnOnce 구현됨
  // FnMut를 구현하면 자동으로 FnOnce 구현됨
  ```
- 클로저 매개변수와 포인터
  ```rust
  // 아래 케이스는 불가능함
  // F는 캡처한 값의 소유권이 필요하지만, 인수로 exclusive reference를 주입하면
  // 참조 뒤에 있는 소유권을 이동시킬 수 없기 때문
  fn quox<F>(f: &mut F)
  where
      F: FnOnce(),
  {
      (f)()
  }

  // 아래 케이스는 불가능함
  // f는 shared ref 뒤에 있기 때문에,
  // shared ref 뒤에 있는 데이터를 exclusive ref로 borrow 할 수 없음.
  fn quox<F>(f: &F)
  where
      F: FnMut(),
  {
      (f)()
  }

  // 아래 케이스는 가능함
  // Fn은 환경에 대한 소유권이나 변경할 권리를 요구하지 않음.
  // exclusive ref는 shared ref로 변환될 수 있으므로 ok
  fn quox<F>(f: &mut F)
  where
      F: Fn(),
  {
      (f)()
  }
  ```
