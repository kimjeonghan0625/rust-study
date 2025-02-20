/*
    permanently unstable feature(stable 에서 안돌아감)
        - rustup override set nightly
    drop check의 메커니즘을 파악 중
*/
#![feature(dropck_eyepatch)]

use std::marker::PhantomData;

pub struct Boks<T> {
    p: *mut T,
    _t: PhantomData<T>,
}

/*
   T에 대해서 사용하지 않겠다고 했지만, drop을 할건지에 대해서는 설정한적 없음
   컴파일러는 T에 대한 제네릭이 Drop을 impl하면 T를 사용하는 것으로 간주
        - '사용'은 'drop' 보다는 빡빡한 제약이어서 사용만 강제하는 방식으로 컴파일러가 동작

    tell compiler not access T, but drop T
        -> PhantomData

    borrow check에서 drop을 구현하는 struct를 볼 때 2가지를 물음
    1. access type parameter(may_dangle)
    2. drop type parameter(phantomdata)
*/
unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.p) };
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t)),
            _t: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        /*
           Safety
           is valid since it was constructed from valid T, and turned into a pointer
           through Box which creates algined pointers, and hasn't been freed since
           self is alive
        */
        unsafe { &*self.p }
    }
}
impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        /*
           Safety
           same as above + we have &mut, no other reference has been givend out
        */
        unsafe { &mut *self.p }
    }
}

use std::fmt::Debug;
struct Oisann<T: Debug>(T);
impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        // access inner T, when drop
        println!("{:?}", self.0)
    }
}

fn main() {
    let x = 42;
    let b = Boks::ny(x);
    println!("{:?}", *b);

    // --------------------------------  //
    let mut y = 42;

    // Boks not work, Box work
    let b = Boks::ny(&mut y);
    // let b = Box::new(&mut y);
    println!("{:?}", y);
    /*
       drop에서 T에 접근하지 않기 때문에, &mut의 lifetime을 줄일 수 있고
       그러면 &y에 대한 사용이 가능해진다
    */

    // --------------------------------  //
    let mut z = 42;

    let b = Boks::ny(Oisann(&mut z));
    // let b = Box::new(Oisann(&mut z));
    println!("{:?}", z);
    /*
       이 코드는 컴파일이 되지만, 되면 안됨
       Boks::drop은 inner value에 access 하지 안지만,
       Oisann::drop은 inner value에 access
            => 암묵적 drop에서 &mut z에 접근

        but Box는 컴파일 불가(좋은 것)
    */
}
