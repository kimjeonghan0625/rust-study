pub struct Boks<T> {
    p: *mut T,
}

impl<T> Drop for Boks<T> {
    // without drop, Boks will memory leak
    fn drop(&mut self) {
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
}
