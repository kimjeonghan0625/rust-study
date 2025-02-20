pub struct Boks<T> {
    p: *mut T,
}

impl<T> Drop for Boks<T> {
    // without drop, Boks will memory leak
    fn drop(&mut self) {
        // let _: u8 = unsafe { std::ptr::read(self.p as *const u8) };
        /*
            nothing stop code above
            compiler need to know whether to consider use anything inside type
        */
        unsafe { Box::from_raw(self.p) };
        // std::ptr::drop_in_place(self.p); // drop T, but not free Box
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
    /*
        b가 &mut을 쓰고있지만, 코드에서 b를 사용하는 곳이 없어서
        빠른 drop이 일어나, println이 되야 될것 같다

        근데 drop을 생각해보면 &mut을 받기 때문에 &mut y를 사용한다
    */
    println!("{:?}", y);
}
