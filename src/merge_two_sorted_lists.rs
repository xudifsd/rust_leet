// Sun Mar 24 13:35:41 CST 2019
// Finished at Sun Mar 24 14:36:59 CST 2019
// spent 61 min

use crate::Solution;
use crate::ListNode;

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;

        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;

        fn helper(mut head: Box<ListNode>, mut tail: &mut Box<ListNode>) -> (Option<Box<ListNode>>, &mut Box<ListNode>) {
            let r_head = head.as_mut();
            let next_head = r_head.next.take();
            tail.next.replace(head);
            tail = tail.next.as_mut().unwrap();
            (next_head, tail)
        }

        loop {
            if l1.is_some() && l2.is_some() {
                if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                    let result = helper(l1.unwrap(), tail);
                    l1 = result.0;
                    tail = result.1;
                } else {
                    let result = helper(l2.unwrap(), tail);
                    l2 = result.0;
                    tail = result.1;
                }
            } else if l1.is_some() {
                tail.next = l1;
                break;
            } else {
                tail.next = l2;
                break;
            }
        }

        head.next
    }
}
