// Sun Mar 24 12:56:11 CST 2019
// Finished at Sun Mar 24 13:34:19 CST 2019
// spent 38 min

use crate::Solution;
use crate::ListNode;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        let mut p = head;

        while p.is_some() {
            let mut a_head = p.unwrap();
            let next_head = a_head.next.take();
            a_head.next = result;
            result = Some(a_head);
            p = next_head;
        }

        result
    }
}
