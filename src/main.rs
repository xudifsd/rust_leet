use leetcode_prelude::*;

pub struct Solution {}

mod average_of_levels_in_binary_tree;

#[allow(dead_code)]
fn test_637() {
    println!("answer is {:?}", Solution::average_of_levels(None));
    println!("answer is {:?}", Solution::average_of_levels(btree![4]));
    println!("answer is {:?}", Solution::average_of_levels(btree![4, 5, 6]));
}

mod validate_binary_search_tree;

#[allow(dead_code)]
fn test_98() {
    println!("answer is {:?}", Solution::is_valid_bst(None));
    println!("answer is {:?}", Solution::is_valid_bst(btree![3, 3, 2]));
}

mod middle_of_the_linked_list;

#[allow(dead_code)]
fn test_876() {
    println!("answer is {:?}", Solution::middle_node(linkedlist![1, 2, 3]));
}

fn main() {
    // test_637();
    // test_98();
    test_876();
}
