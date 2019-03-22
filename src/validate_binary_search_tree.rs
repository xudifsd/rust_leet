use std::rc::Rc;
use std::cell::RefCell;

use crate::Solution;
use crate::TreeNode;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(root: &TreeNode, low: i64, high: i64) -> bool {
            let val = root.val as i64;

            if val >= high || val <= low {
                return false;
            }
            if let Some(l) = &root.left {
                if !helper(&l.borrow(), low, val) {
                    return false;
                }
            }
            if let Some(r) = &root.right {
                if !helper(&r.borrow(), val, high) {
                    return false;
                }
            }
            true
        }

        if let Some(root) = root {
            helper(&root.borrow(), std::i64::MIN, std::i64::MAX)
        } else {
            true
        }
    }
}
