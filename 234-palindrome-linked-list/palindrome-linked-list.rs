use crate::list_node::ListNode;
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {

        let mut values = Vec::new();
        let mut current = &head; // reference to head of linked list
        // traverses the linked list.
        while let Some(node) = current {
            values.push(node.val);
            current = &node.next;
        }
        // Check if the values form a palindrome
        let len = values.len();//length of the vector 
        //loop over the 1st half
        for i in 0..len / 2 {
            if values[i] != values[len - 1 - i] {
            return false; // which mean that is not plindrome
        }
    }
    true   
    }

}