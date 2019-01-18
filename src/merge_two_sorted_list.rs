#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(v1), None) => Some(v1),
            (None, Some(v2)) => Some(v2),
            (Some(mut v1), Some(mut v2)) => {
                if v1.val < v2.val {
                    let next = v1.next.take();
                    v1.next = Solution::merge_two_lists(next, Some(v2));
                    Some(v1)
                } else {
                    let next = v2.next.take();
                    v2.next = Solution::merge_two_lists(Some(v1), next);
                    Some(v2)
                }
            }
            _ => None,
        }
    }
}

pub struct Solution;
