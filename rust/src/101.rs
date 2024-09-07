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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn compare(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (l, r) {
                (None, None) => true,
                (None, Some(_n)) | (Some(_n), None) => false,
                (Some(l), Some(r)) => {
                    if l.borrow().val != r.borrow().val {
                        return false;
                    }
                    return compare(l.borrow().left.clone(), r.borrow().right.clone())
                        && compare(l.borrow().right.clone(), r.borrow().left.clone());
                }
            }
        }
        match root {
            Some(r) => compare(r.borrow().left.clone(), r.borrow().right.clone()),
            None => true,
        }
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    if let Some(root) = root.as_ref() {
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    }

    // let expected = root.clone();
    let result = Solution::is_symmetric(root.clone());
    println!("{:?}", result);
}
