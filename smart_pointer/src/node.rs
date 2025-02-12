use std::cell::RefCell;
use std::rc::Rc;

// 实现 Drop 打印 id
#[derive(Debug)]
pub struct Node {
    id: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        print!("{:?}", self);
    }
}

// n > 0
// 返回一个循环n次引用的智能指针
// 数字从 1 - n
// 1 -> 2 -> 3 -> 4 -> ... -> n -> 1
pub fn generate_n_loop_pointer(n: usize) -> Node {
    let mut head = Node { id: 0, next: None };
    let mut ptr = Rc::new(RefCell::new(Node {
        id: n as i32,
        next: None,
    }));
    head.next = Some(Rc::clone(&ptr));
    for i in (1..n).rev() {
        let mut new_node = Rc::new(RefCell::new(Node {
            id: i as i32,
            next: None,
        }));
        let mut old_node = std::mem::replace(&mut ptr, new_node);
        ptr.clone().borrow_mut().next = Some(old_node);
    }
    if let Some(h) = &mut head.next {
        h.clone().borrow_mut().next = Some(ptr);
    }
    head
}
