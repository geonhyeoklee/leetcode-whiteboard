use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let l = root.as_ref().unwrap().borrow().left.clone();
        let r = root.as_ref().unwrap().borrow().right.clone();

        let l = Self::max_depth(l);
        let r = Self::max_depth(r);

        l.max(r) + 1
    }
}

fn main() {
    let expected = 1;
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let result = Solution::max_depth(root);

    assert_eq!(expected, result);
}
