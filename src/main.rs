use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub struct Solution {}

mod average_of_levels_in_binary_tree;

fn test_637() {
    println!("answer for None is {:?}", Solution::average_of_levels(None));

    let root = Rc::new(RefCell::new(TreeNode::new(4)));
    println!("answer for None is {:?}", Solution::average_of_levels(Some(Rc::clone(&root))));

    {
        let mut root_ref = root.borrow_mut();
        root_ref.left.replace(Rc::new(RefCell::new(TreeNode::new(5))));
        root_ref.right.replace(Rc::new(RefCell::new(TreeNode::new(6))));
    }
    println!("answer for None is {:?}", Solution::average_of_levels(Some(Rc::clone(&root))));
}

fn main() {
    test_637();
}
