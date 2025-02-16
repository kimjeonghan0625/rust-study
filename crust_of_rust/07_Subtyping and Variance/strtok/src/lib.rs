// return의 lifetime을 static으로 바꿔도 여전히 에러
pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) {
    // pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) -> &'static str {
    // pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        // prefix
        // ""
    } else {
        let prefix = *s;
        *s = "";
        // prefix
        // ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let mut x = "hello world";
    //     {
    //         let hello = strtok(&mut x, ' ');
    //         // assert_eq!(hello, "hello");
    //         /*
    //            scope를 추가해도 여전히 컴파일이 안됨
    //            scope내에서 assert_eq를 사용하지 않아도
    //            fn strtok의 반환 타입을 'static으로 바꿔도 안됨
    //            fn strtok의 반환을 없애도 안돔
    //         */
    //     }
    //     assert_eq!(x, "world");
    // }

    #[test]
    fn it_works() {
        /*
           x: &'static str
           &mut x: &'? mut &'static str
           strtok의 첫번째 인자: &'a mut &'a str

           함수의 인자와 파라미터가 같아야 하고, 'a는 공통이어야 하기 때문에
           &'static mut 'static str 이 되어버림
           그래서 assert_eq에서도 &mut이 살아있다고 인식
        */
        let mut x = "hello world";
        // check_is_static(x);
        strtok(&mut x, ' ');
        // assert_eq!(x, "world");
        /*
            assert를 주석 처리 하면 컴파일이 되는 이유
            "hello world"는 여전히 'static 이지만, 임의로 scope를 제한시킴
            lifetime이 긴 것을 짧은것에서도 사용 가능하기 때문

            fn main(){
                let s = String::new();
                let x: &'static str = "hello";
                let mut y/*  y: &'a */ = &*s;
                y = x; // 'static -> 'a

                // static is subtype of any lifetime of 'a
            }
        */

        fn check_is_static(_: &'static str) {}
    }
}
