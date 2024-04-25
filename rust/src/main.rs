// Definition for a binary tree node.
pub struct Solution;

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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
  pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = i32::MIN;

    if let Some(ref node) = root {
      if node.borrow().left.is_none() && node.borrow().right.is_none() {
        max = node.borrow().val;
        return max;
      }
    }

    
    Self::find_sum(root, &mut max);

    max
  }

  pub fn find_sum (node: OptNode, max: &mut i32) -> i32 {
    match node {
      Some(node) => {
        let left  = Self::find_sum(node.borrow().left.clone(), max);
        let right  = Self::find_sum(node.borrow().right.clone(), max);
        let all_sum = left + right + node.borrow().val;
        let left_node_sum = left + node.borrow().val;
        let right_node_sum = right + node.borrow().val;

        *max = i32::max(*max, left);
        *max = i32::max(*max, right);
        *max = i32::max(*max, all_sum);
        *max = i32::max(*max, left_node_sum);
        *max = i32::max(*max, right_node_sum);

        let current_cycle_max = i32::max(left_node_sum,right_node_sum);
        i32::max(current_cycle_max, node.borrow().val)
      },
      None => 0
    }
  }
}

fn main() {
  let root = Some(Rc::new(RefCell::new(TreeNode::new(-10))));
  if let Some(ref node) = root {
    node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
  }

  let right_root = root.as_ref().unwrap().borrow_mut().right.clone();
  if let Some(ref node) = right_root {
    node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
  }

  let result = Solution::max_path_sum(root);
  println!("{}", result);
}