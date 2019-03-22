use leetcode_prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

mod average_of_levels_in_binary_tree;

fn test_637() {
    println!("answer is {:?}", Solution::average_of_levels(None));

    println!("answer is {:?}", Solution::average_of_levels(btree![4]));

    println!("answer is {:?}", Solution::average_of_levels(btree![4, 5, 6]));
}

mod validate_binary_search_tree;

fn test_98() {
    println!("answer is {:?}", Solution::is_valid_bst(None));
    println!("answer is {:?}", Solution::is_valid_bst(btree![3, 3, 2]));
}

fn main() {
    // test_637();

    test_98();
}
