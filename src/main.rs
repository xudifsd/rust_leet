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

mod reverse_linked_list;

#[allow(dead_code)]
fn test_206() {
    println!("answer is {:?}", Solution::reverse_list(linkedlist![1, 2, 3]));
}

mod merge_two_sorted_lists;

#[allow(dead_code)]
fn test_21() {
    println!("answer is {:?}", Solution::merge_two_lists(linkedlist![1, 2, 3], linkedlist![2, 3, 4]));
}

fn main() {
    // test_637();
    // test_98();
    // test_876();
    // test_206();
    test_21();
}
