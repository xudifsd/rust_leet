// Sun Mar 24 14:44:03 CST 2019
// Finished at Sun Mar 24 20:52:38 CST 2019
// spent 6 hour 8 min

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

struct MyListNode {
    val: i32,
    prev: Option<Rc<RefCell<MyListNode>>>,
    next: Option<Rc<RefCell<MyListNode>>>,
}

impl std::fmt::Debug for MyListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.repr())
    }
}

impl MyListNode {
    fn new(v: i32) -> Self {
        MyListNode {val: v, prev: None, next: None}
    }

    fn repr(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("{}", self.val).as_str());

        let mut p = self.next.clone();

        loop {
            if p.is_none() {
                break;
            }
            s.push_str(format!("->{}", p.as_ref().unwrap().borrow().val).as_str());
            p = p.as_ref().unwrap().clone().borrow().next.clone();
        }

        s
    }
}

#[derive(Debug)]
pub struct MyLinkedList {
    head: Rc<RefCell<MyListNode>>,
    tail: Rc<RefCell<MyListNode>>,
    count: i32,
}

impl MyLinkedList {
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(MyListNode::new(0)));
        let tail = Rc::new(RefCell::new(MyListNode::new(0)));

        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        MyLinkedList {
            head: head,
            tail: tail,
            count: 0
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.count {
            return -1;
        }

        MyLinkedList::get_at_index(self, index).borrow().val
    }

    fn get_at_index(&self, index: i32) -> Rc<RefCell<MyListNode>> {
        let mut p = self.head.clone();
        let mut i = -1;

        while i < index {
            p = p.clone().borrow().next.as_ref().unwrap().clone();
            i += 1;
        }

        p.clone()
    }

    pub fn add_at_head(&mut self, val: i32) {
        MyLinkedList::add_at_index(self, 0, val);
    }

    pub fn add_at_tail(&mut self, val: i32) {
        MyLinkedList::add_at_index(self, self.count, val);
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index > self.count {
            return;
        }
        self.count += 1;

        let next = MyLinkedList::get_at_index(self, index);
        let prev = next.borrow().prev.as_ref().unwrap().clone();
        let node = Rc::new(RefCell::new(MyListNode::new(val)));

        node.borrow_mut().prev = Some(prev.clone());
        node.borrow_mut().next = Some(next.clone());
        prev.borrow_mut().next = Some(node.clone());
        next.borrow_mut().prev = Some(node.clone());
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.count {
            return;
        }
        self.count -= 1;

        let node = MyLinkedList::get_at_index(self, index);
        let prev = node.borrow().prev.as_ref().unwrap().clone();
        let next = node.borrow().next.as_ref().unwrap().clone();

        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;

        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev.clone());
    }
}
