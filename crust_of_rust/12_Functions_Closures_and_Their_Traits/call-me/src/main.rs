fn main() {
    let mut x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));
    // x = bar::<u32>;
    baz(bar::<u32>);
    baz(bar::<i32>);
    quox(&bar::<u32>);

    let z = String::new();

    let f = move || {
        println!("{z}");
        // drop(z);
    };

    quox(f);
}

fn bar<T>() {}

fn baz(f: fn()) {}

fn quox<F>(mut f: F)
where
    F: FnOnce(),
{
}

fn make_fn() -> impl FnOnce() {
    let z = String::new();
    move || {
        println!("{z}");
    }
}
