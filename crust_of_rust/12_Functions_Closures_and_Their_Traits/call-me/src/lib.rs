mod dyn_fn {
    fn execute_fn_once(f: Box<dyn FnOnce()>) {
        f();
    }
    #[test]
    fn test_fn_once() {
        let greeting = String::from("Hello");
        let closure = Box::new(move || {
            println!("{}", greeting);
            drop(greeting);
        });
        execute_fn_once(closure);
    }
    fn execute_fn_mut<'a>(f: &'a mut dyn FnMut()) {
        f();
    }
    #[test]
    fn test_fn_mut() {
        let mut counter = 0;
        {
            let mut closure = || {
                counter += 1;
            };
            execute_fn_mut(&mut closure);
            execute_fn_mut(&mut closure);
        }
        assert_eq!(counter, 2);
    }
    fn execute_fn(f: &dyn Fn()) {
        f();
    }
    #[test]
    fn test_fn() {
        let message = "Hello, world!";
        let closure = || {
            println!("{}", message);
        };
        execute_fn(&closure);
        execute_fn(&closure);
    }
}

mod const_fn {
    const fn square(x: i32) -> i32 {
        x * x
    }

    fn not_const() -> String {
        println!("{}", square(2));
        String::from("hello")
    }

    const fn new_const_fn() {
        // not_const();
    }
}
