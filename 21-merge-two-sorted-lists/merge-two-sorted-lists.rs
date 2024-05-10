// Definition for singly-linked list.
use crate::list_node::ListNode;
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, second_list) => second_list,
            (first_list, None) => first_list,
            (Some(mut head1), Some(head2)) => {
                if head1.val <= head2.val {
                    head1.next = Self::merge_two_lists(head1.next, Some(head2));
                    Some(head1)
                } else {
                    Self::merge_two_lists(Some(head2), Some(head1))
                }
            }
        }
    }
}