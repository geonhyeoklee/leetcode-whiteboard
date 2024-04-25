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

use std::cell::RefCell;
use std::rc::Rc;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let temp = String::from("");
        let mut sum = 0;

        Self::dfs(root, temp, &mut sum);

        sum
    }

    pub fn dfs(node: OptNode, mut temp: String, sum: &mut i32) {
        if let Some(node) = node {
            let val = node.borrow().val;
            temp.push_str(&val.to_string());

            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                let temp_i32: Result<i32, _> = temp.parse();
                match temp_i32 {
                    Ok(number) => {
                        *sum += number;
                    },
                    Err(err) => println!("Error parsing number: {}", err),
                }
            }

            if node.borrow().left.is_some() {
                Self::dfs(node.borrow().left.clone(), temp.clone(), sum)
            }

            if node.borrow().right.is_some() {
                Self::dfs(node.borrow().right.clone(), temp.clone(), sum)
            }
        }
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

    let result = Solution::sum_numbers(root);
    println!("{}", result);
}
