/* Fill in the blank using two approches,
and fix the errror */
fn create_fn() -> impl Fn(i32) -> i32 {
    let num = 5;

    // how does the following closure capture the evironment variable `num`
    // &T, &mut T, T ?
    move |x| x + num
}

fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}
