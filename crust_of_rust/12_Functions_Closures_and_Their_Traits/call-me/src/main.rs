fn main() {
    let mut x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));
    // x = bar::<u32>;
    baz(bar::<u32>);
    baz(bar::<i32>);
    quox(&bar::<u32>);
}

fn bar<T>() {}

fn baz(f: fn()) {}

fn quox<F>(f: &F)
where
    F: Fn(),
{
}
