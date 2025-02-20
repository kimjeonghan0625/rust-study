/*
    permanently unstable feature(stable 에서 안돌아감)
        - rustup override set nightly
    drop check의 메커니즘을 파악 중
*/
#![feature(dropck_eyepatch)]

pub struct Boks<T> {
    p: *mut T,
}

/*
   may_dangle: coder promise compiler to not access T

   Boks의 어떤 trait는 unsafe하고 나머지는 safe 라기 보다는
   여기서 unsafe는 may_dangle에 연관된 unsafe
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
}
