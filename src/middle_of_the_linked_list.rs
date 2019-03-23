// Sat Mar 23 20:20:35 CST 2019
// Finished at Sat Mar 23 21:56:18 CST 2019
// spent 96 min

use crate::Solution;
use crate::ListNode;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = &head;
        let mut count = 0;

        loop {
            if p.is_none() {
                break;
            }
            p = &p.as_ref().unwrap().next;
            count += 1;
        }

        let mut result = head;
        let mut i = 0;
        while i < count / 2 {
            result = result.unwrap().next;
            i += 1;
        }

        result
    }
}
