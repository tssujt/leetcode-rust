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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::bst_helper(&nums[..])
    }

    fn bst_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None
        }
        Some(Rc::new(RefCell::new(TreeNode{
            val: nums[nums.len() / 2],
            left: Solution::bst_helper(&nums[0..(nums.len()/2)]),
            right: Solution::bst_helper(&nums[(nums.len()/2+1)..]),
        })))
    }
}

pub struct Solution;
