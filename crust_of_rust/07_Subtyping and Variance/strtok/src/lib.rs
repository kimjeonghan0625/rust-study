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
