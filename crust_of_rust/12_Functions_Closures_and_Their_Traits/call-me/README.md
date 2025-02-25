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
## 클로저 파고들기
- FnMut와 Fn은 일반적으로 참조자를 캡처하므로, 여러 번 호출 가능하지만 FnMut의 경우에는 순차적 호출이 보장되어야 한다.  
  다시말해 exclusive ref는 동시에 단 하나의 참조자만 유효할 수 있기 때문에,  
  여러 스레드에서 동시에 동일한 FnMut 클로저를 호출하는 것은 허용되지 않는다.
- 환경을 캡처한 클로저는 함수 포인터 타입에 대입할 수 없다.
> 🌟 아래와 같은 방법으로 클로저를 모델링해볼 수도 있다.
```rust
let f = || {
    println!("{}", z);
};

// 위 클로저를 아래와 같이 모델링 해볼 수 있다.
// 필드로 상태를 관리하고 Fn 트레이트의 call 메서드 구현부에서 관리하는 상태를 조회한다.

// 이 경우에 클로저는 공유 참조자를 캡처하고 있다.
// 여러 번 호출 가능하다.
struct FClosure<'scope> {
    z: &'scope String,
}

impl<'scope> Fn() for FClosure<'scope> {
    fn call(&self) {
        // copy-paste from closure definition
        println!("{}", self.z)
    }
}

// 이 경우에 클로저는 유일 참조자를 캡처하고 있다.
// 여러 번 호출 가능하지만 동시 호출은 보장되지 않는다.
struct FClosure<'scope> {
    z: &'scope mut String,
}

impl<'scope> FnMut() for FClosure<'scope> {
    fn call(&mut self) {
        // copy-paste from closure definition
        self.clear();
    }
}

// 이 경우에 클로저는 값을 참조하는 것이 아닌 값을 소유하고 있다.
// 클로저가 종료될 때, 소유하고 있던 힙 메모리는 정리될 뿐만 아니라,
// 이미 환경은 소유권을 가지고 있지 않기 때문에 단 한 번만 사용될 수 있다.
struct FClosure<'scope> {
    z: String,
}

impl<'scope> FnMut() for FClosure<'scope> {
    fn call(self) {
        // copy-paste from closure definition
        drop(self);
    }
}
```
- move의 downside:
  - move 키워드를 클로저 앞에 붙이면 기본적으로 캡처하는 모든 데이터를 소유권 이동으로 간주한다.
  - 즉 어떤 데이터는 소유권 이동하고 어떤 데이터는 참조자 캡처하는 것이 불가능하다.
      - 이 문제는 외부 데이터에 대한 참조자를 캡처하도록 만드는 방식으로 우회하여 해결할 수도 있다.
> 왜 하나는 FnOnce 만 구현하고 하나는 Fn, FnMut, FnOnce를 전부 구현할까?
```rust
fn main() {
    let z = String::new();

    let f = move || {
        println!("{z}");
        // drop(z);
    };
}

fn make_fn() -> impl FnOnce() {
    let z = String::new();
    move || {
        println!("{z}");
    }
}
```