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
            right: None,
        }
    }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Write as _;

struct Codec;

impl Codec {
    fn new() -> Self {
        Self {}
    }


    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut out = String::new();
        Self::preorder_serialize(root.as_ref().map(|v| v.borrow()).as_deref(), &mut out);
        out
    }

    fn preorder_serialize(node: Option<&TreeNode>, out: &mut String) {
        if let Some(node) = node {
            writeln!(out, "{}", node.val).unwrap();
            Self::preorder_serialize(node.left.as_ref().map(|v| v.borrow()).as_deref(), out);
            Self::preorder_serialize(node.right.as_ref().map(|v| v.borrow()).as_deref(), out);
        } else {
            writeln!(out, "x").unwrap();
        }
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut itr = data.lines().map(|v| v.parse().ok());
        Self::preorder_deserialize(&mut itr)
    }

    fn preorder_deserialize(data: &mut impl Iterator<Item = Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let val = data.next()??;
        let left = Self::preorder_deserialize(data);
        let right = Self::preorder_deserialize(data);
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

}
fn main() {
  let root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  if let Some(ref node) = root {
      node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
      node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
  }

  let left_root = root.as_ref().unwrap().borrow_mut().left.clone();
  if let Some(ref node) = left_root {
      node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
      node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  }

  let codec = Codec::new();
  let data = codec.serialize(root);
  codec.deserialize(data);
}