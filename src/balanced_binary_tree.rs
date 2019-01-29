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
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (balanced, height): (bool, usize) = Solution::validate(&root);

        balanced
    }

    fn validate(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool, usize) {
        match root {
            Some(node) => {
                let (balanced, left_height): (bool, usize) = Solution::validate(&node.borrow().left);
                if balanced == false {
                    return (false, 0);
                }

                let (balanced, right_height): (bool, usize) = Solution::validate(&node.borrow().right);
                if balanced == false {
                    return (false, 0);
                }

                ((left_height as i32 - right_height as i32).abs() <= 1, left_height.max(right_height) + 1)
            }
            None => (true, 0),
        }
    }
}

pub struct Solution;
