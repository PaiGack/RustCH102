use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    id: i32,
    next: Option<Rc<RefCell<Node>>>,
}

// Drop 打印 id
#[derive(Debug)]
pub struct WeakNode {
    id: i32,
    next: Weak<RefCell<Node>>,
}

impl Drop for WeakNode {
    fn drop(&mut self) {
        print!("-> {:?}", self.next.upgrade());
    }
}

// n > 0
// 返回一个循环n次引用的智能指针
// 数字从 1 - n
// 1 -> 2 -> 3 -> 4 -> ... -> n -> 1
pub fn generate_n_loop_weak_pointer(n: usize) -> WeakNode {
    let mut head = WeakNode {
        id: 0,
        next: Weak::new(),
    };
    let mut ptr = Rc::new(RefCell::new(Node { id: 1, next: None }));
    head.next = Weak::clone(&Rc::downgrade(&ptr));
    for i in (2..n + 1).rev() {
        let mut new_node = Rc::new(RefCell::new(Node {
            id: i as i32,
            next: None,
        }));
        let old_node = std::mem::replace(&mut ptr, new_node);
        ptr.clone().borrow_mut().next = Some(old_node);
    }
    head.next.upgrade().unwrap().borrow_mut().next = Some(ptr);
    head
}
