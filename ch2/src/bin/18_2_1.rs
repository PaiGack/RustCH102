/* Refactoring the following code using iterators */
fn main() {
    let arr = [0; 10];
    for val in arr.into_iter() {
        println!("{}", val)
    }
}
