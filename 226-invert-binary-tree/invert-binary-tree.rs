use crate::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            //clone left child and it leafs
            let left = Solution::invert_tree(node.borrow().left.clone());
            //clone the right child and it leafs
            let right = Solution::invert_tree(node.borrow().right.clone());
            //swapping 
            //borrow_mut is function using RefCall which allow access for mutable values
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
        }
        root 
    }
}