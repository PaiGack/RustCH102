fn call_on_ref_zero<F>(f: F)
where
    for<'a> F: Fn(&'a i32),
{
    let zero = 0;
    f(&zero);
}

fn call_on_ref_zero2<F>(f: F)
where
    F: for<'a> Fn(&'a i32),
{
    let zero = 0;
    f(&zero);
}

fn main() {
    println!("Success!")
}
