/* Make it work by change the trait bound, in two ways*/
fn fn_once<F>(func: F)
where
// todo 也可以加 copy 语义
    F: Fn(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z| {z == x.len()})
}
