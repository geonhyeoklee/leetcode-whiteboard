use std::cell::RefCell;
use std::i32;
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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut previous = None;
        Self::inorder(root, &mut previous)
    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, previous: &mut Option<i32>) -> i32 {
        if let Some(root) = root {
            let mut node = root.borrow_mut();
            Self::inorder(node.left.take(), previous)
                .min({
                    let current_min = match previous {
                        Some(previous_val) => (node.val - *previous_val).abs(),
                        None => i32::MAX,
                    };
                    *previous = Some(node.val);
                    current_min
                })
                .min(Self::inorder(node.right.take(), previous))
        } else {
            i32::MAX
        }
    }
}

fn main() {
    let expected = 0;
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let result = Solution::get_minimum_difference(root);

    assert_eq!(expected, result);
}
