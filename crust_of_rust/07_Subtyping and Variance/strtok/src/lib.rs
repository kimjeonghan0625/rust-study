// lifetime을 2가지로 분리하면, 처음의 테스트도 정상적으로 작동한다
pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    // pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

struct TouchDrop<T: std::fmt::Debug>(T);
impl<T: std::fmt::Debug> Drop for TouchDrop<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}
fn main() {
    let x = String::new();
    /*
       vec![&x] 일때는 컴파일 에러가 안나고, TouchDrop일때는 컴파일 에러가 발생
       vector는 inner type이 drop을 구현할때, vector의 drop시 inner type에 대한 drop을 호출
        - drop을 구현한 TouchDrop은 컴파일 에러 발생
        - &T는 drop을 구현 X
    */
    // let z = vec![&x];
    let z = vec![TouchDrop(&x)];
    drop(x);
    // drop(z);
}

use std::marker::PhantomData;
struct Deserializer<T> {
    // phantomData<T>는 컴파일러에게 Deserializer가 T를 소유하고 있는것으로 알림
    // 그래서 T를 drop 하는 것으로 간주
    _t: PhantomData<T>,
}
struct Deserializer2<T> {
    // 이 signiature는 T를 소유하지 않기 때문에, T에 대한 drop이 발생 X
    // 그럼 Deserializer3과 같지 않냐?
    // fn() -> T: covariant, fn(T): contravariant
    _t: PhantomData<fn() -> T>,
}
struct Deserializer3<T> {
    // contravariant이기 때문에 쓰기 더 어려움
    _t: PhantomData<fn(T)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_now() {
        let mut x = "hello world";
        /*
            다음줄이 에러가 안나는 이유는 z의 lifetime이 줄어들었기 때문인데,
            이게 가능한 이유는 mutable reference는 lifetie에 대해서 covariance이기 때문
        */
        let z = &mut x;

        // strtok<'a,'b>(&'a mut &'b      str) -> &'b      str
        //               &'? mut &'static str  -> &'static str
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }

    #[test]
    fn it_works() {
        let mut x = "hello world";
        check_is_static(x);

        /*
            x:          &'?      mut &'static str
            strtok arg: &'a      mut &'a      str

            &'a mut T는 T에 대해서 invariant이기 때문에, 'a는 'static이 되어야 한다
            그러면 &'static mut에 &'a mut을 전달하는 상황이기 때문에,
                - `x` does not live long enough
        */
        strtok(&mut x, ' ');
        // assert_eq!(x, "world");

        fn check_is_static(_: &'static str) {}
    }
}
