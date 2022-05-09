#![allow(unused)]

/// 链表数字顺序从小到大
/// push(3)
/// 3
/// push(1)
/// 1 -> 3
/// push(2)
/// 1 -> 2 -> 3
/// pop(2)
/// 1 -> 3

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Clone)]
struct Node {
    val: i32,
    next: Link,
}

impl List {
    fn new() -> Self {
        List { head: None }
    }

    // insert a value
    // 如果有相同的数字，仍然插入
    fn push(&mut self, v: i32) {
        println!("push {}", v);
        let mut new_node = Box::new(Node { val: v, next: None });
        match self.head.as_mut() {
            Some(mut node) => {
                if node.val >= v {
                    new_node.next = Some(self.head.take().unwrap());
                    self.head = Some(new_node);
                } else {
                    while node.next.is_some() && node.next.as_ref().unwrap().val < v {
                        node = node.next.as_mut().unwrap();
                    }
                    if node.next.is_some() {
                        new_node.next = node.next.take();
                    }
                    node.next = Some(new_node);
                }
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }

    // pop 有这个数字 就返回 Some(i32)
    // 没有 就返回None
    // 如果有相同的数字，就删除一个就好了
    fn pop(&mut self, v: i32) -> Option<i32> {
        match self.head.as_mut() {
            Some(mut node) => {
                if node.val == v {
                    let res = node.val;
                    self.head = node.next.take();
                    Some(res)
                } else {
                    while node.next.is_some() && node.next.as_ref().unwrap().val != v {
                        node = node.next.as_mut().unwrap();
                    }
                    match node.next.take() {
                        Some(v) => {
                            let res = v.val;
                            node.next = v.next;
                            Some(res)
                        }
                        None => None,
                    }
                }
            }
            None => None,
        }
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter(self)
    }
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter(self.head.as_deref())
    }
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a> {
        IterMut(self.head.as_deref_mut())
    }
}

// O(1)
// Iter, IterMut, IntoIterator
// 顺序是从小的数字到大的数字输出
pub struct IntoIter(List);

impl Iterator for IntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.head.as_mut() {
            Some(v) => {
                let num = v.val;
                self.0.pop(num)
            }
            None => None,
        }
    }
}

pub struct Iter<'a>(Option<&'a Node>);

impl<'a> Iterator for Iter<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            let num = node.val;
            self.0 = node.next.as_deref();
            num
        })
    }
}

pub struct IterMut<'a>(Option<&'a mut Node>);

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            let mut num = &mut node.val;
            self.0 = node.next.as_deref_mut();
            num
        })
    }
}

use std::option::Option;
fn main() {
    let mut list = List::new();
    list.push(2);
    list.push(3);
    list.push(3);
    list.push(4);
    list.push(1);
    list.push(1);
    list.push(2);

    assert_eq!(Option::Some(4), list.pop(4));
    assert_eq!(Option::Some(3), list.pop(3));
    // assert_eq!(Option::Some(3), list.pop(3));
    assert_eq!(Option::Some(2), list.pop(2));
    assert_eq!(Option::Some(2), list.pop(2));
    assert_eq!(Option::Some(1), list.pop(1));

    let mut v1 = Vec::new();
    for i in list.iter_mut() {
        print!("{} -> ", i);
        *i += 1;
        v1.push(*i);
    }

    println!("{:?}", v1);
    assert_eq!(v1, vec![2, 4]);

    for i in list.into_iter() {
        print!("{} -> ", i);
    }
    println!("\n\n--- main end ---");
}
