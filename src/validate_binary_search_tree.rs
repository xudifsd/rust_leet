use std::rc::Rc;
use std::cell::RefCell;

use crate::Solution;
use crate::TreeNode;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(root: Option<Rc<RefCell<TreeNode>>>, low: i64, high: i64) -> bool {
            match root {
                None => true,
                Some(r) => {
                    let cell = r.borrow();
                    let val = cell.val as i64;

                    if val >= high || val <= low {
                        return false;
                    }
                    if let Some(l) = &cell.left {
                        if !helper(Some(Rc::clone(l)), low, val) {
                            return false;
                        }
                    }
                    if let Some(r) = &cell.right {
                        if !helper(Some(Rc::clone(r)), val, high) {
                            return false;
                        }
                    }
                    true
                }
            }
        }

        helper(root, std::i64::MIN, std::i64::MAX)
    }
}
