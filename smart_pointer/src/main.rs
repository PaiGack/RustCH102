pub mod node;
pub mod weak_node;

use crate::node::generate_n_loop_pointer;
use crate::weak_node::generate_n_loop_weak_pointer;

fn main() {
    {
        let list1 = generate_n_loop_pointer(1);
        println!("-- list1 --")
    }
    println!("\n-- list1 end --");
    {
        let list2 = generate_n_loop_weak_pointer(1);
        println!("-- list2 --")
    }
    println!("\n-- list2 end --");
}
