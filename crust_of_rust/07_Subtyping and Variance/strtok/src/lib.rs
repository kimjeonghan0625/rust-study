pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) -> &'a str {
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
    fn it_works() {
        let mut x = "hello world";
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
        /*
           strtok의 lifetime인 'a가 함수의 반환값과, &mut의 lifetime이 같다고
           명시했기 때문에, hello가 살아있는한, &mut도 살아있음
        */
    }
}
