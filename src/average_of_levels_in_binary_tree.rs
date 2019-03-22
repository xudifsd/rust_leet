use crate::Solution;
use crate::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = Vec::new();
        if let None = root {
            return result;
        }

        if let Some(p) = root {
            let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

            queue.push_back(Some(p));
            queue.push_back(None);

            let mut sum: f64 = 0.0;
            let mut count: i64 = 0;

            while !queue.is_empty() {
                let n = queue.pop_front();
                match n {
                    Some(None) => {
                        result.push(sum / count as f64);
                        if queue.is_empty() {
                            return result;
                        } else {
                            sum = 0.0;
                            count = 0;
                            queue.push_back(None)
                        }
                    },
                    Some(Some(n_b)) => {
                        let refcell = n_b.borrow();
                        sum += refcell.val as f64;
                        count += 1;

                        if let Some(l) = &refcell.left {
                            queue.push_back(Some(Rc::clone(l)));
                        }
                        if let Some(r) = &refcell.right {
                            queue.push_back(Some(Rc::clone(r)));
                        }
                    },
                    None => {
                        panic!("impossible");
                    }
                }
            }
        }
        result
    }
}
