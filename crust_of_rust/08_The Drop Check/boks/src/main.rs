/*
    permanently unstable feature(stable 에서 안돌아감)
        - rustup override set nightly
        - rustup default stable
    drop check의 메커니즘을 파악 중
*/
#![feature(dropck_eyepatch)]

use std::marker::PhantomData;
use std::ptr::NonNull;

/*
    invariant in T(by *mut T)

    can't treat Boks<&'static str> as Boks<&'a str>
    but &'static str -> &'a str
        Box<&'static str> -> Box<&'a str> can
*/
pub struct Boks<T> {
    // p: *mut T,
    p: NonNull<T>,
    /*
       PhatomData<T>는 T를 소유하고, covariant  -> T를 소유하기 때문에, T에 대한 drop check를 진행
       fn()->T는 T를 소유하지 않고, covariant    -> T를 소유하지 않기 때문에, 컴파일러가 T에 대해 drop check를 하지않음
    */
    // _t: PhantomData<T>,
    _t: PhantomData<fn() -> T>,
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
        unsafe { Box::from_raw(self.p.as_mut()) };
        // unsafe { Box::from_raw(self.p) };
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            // p: Box::into_raw(Box::new(t)),
            // box never create null pointer
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
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
        // unsafe { &*self.p }
        unsafe { &*self.p.as_ref() }
    }
}
impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        /*
           Safety
           same as above + we have &mut, no other reference has been givend out
        */
        // unsafe { &mut *self.p }
        unsafe { &mut *self.p.as_mut() }
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

// use std::iter::Empty;
fn main() {
    /*
       _t: PhantomData<fn() -> T>  가 되면 다음의 문제가 발생
       Oissan이 drop을 할 때, &mut에 접근을 하는데, Boks는 컴파일됨
    */
    let mut z = 42;
    let b = Boks::ny(Oisann(&mut z));
    // let b = Box::new(Oisann(&mut z));
    println!("{:?}", z);

    let s = String::from("hei");
    /*
       Box는 컴파일이 되고 -> covariant
       Boks는 컴파일 안됨 -> Invariant
    */
    // let mut boks1 = Boks::ny(&*s);
    // let boks2: Boks<&'static str> = Boks::ny("heisann");
    let mut boks1 = Box::new(&*s);
    let boks2 = Box::new("heisann");

    boks1 = boks2;
}
