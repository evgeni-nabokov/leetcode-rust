#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::cell::RefCell;

use crate::common::tree_node::TreeNode;

struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(some) = root {
            let left = RefCell::borrow_mut(&some).left.clone();
            let right = RefCell::borrow_mut(&some).right.clone();
            if left.is_none() && right.is_some() {
                RefCell::borrow_mut(&some).left = right;
                RefCell::borrow_mut(&some).right = None;
                Solution::invert_tree(RefCell::borrow_mut(&some).left.clone());
            } else if left.is_some() && right.is_none() {
                RefCell::borrow_mut(&some).right = left;
                RefCell::borrow_mut(&some).left = None;
                Solution::invert_tree(RefCell::borrow_mut(&some).right.clone());
            } else if left.is_some() && right.is_some() {
                let node = RefCell::borrow(&some);
                node.left.as_ref().unwrap().swap(node.right.as_ref().unwrap());
                Solution::invert_tree(node.left.clone());
                Solution::invert_tree(node.right.clone());
            }
            Some(some)
        } else {
            None
        }
    }
}