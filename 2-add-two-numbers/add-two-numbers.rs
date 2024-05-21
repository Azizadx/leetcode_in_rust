use crate::list_node::ListNode;
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut current_list1 = l1;
    let mut current_list2 = l2;
    let mut dummy_head = ListNode::new(0);
    let mut current_node = &mut dummy_head;
    let mut carry = 0;

    while current_list1.is_some() || current_list2.is_some() || carry != 0 {
        let sum = carry
            + current_list1.as_ref().map_or(0, |node| node.val)
            + current_list2.as_ref().map_or(0, |node| node.val);

        carry = sum / 10;
        current_node.next = Some(Box::new(ListNode::new(sum % 10)));
        current_node = current_node.next.as_mut().unwrap();

        current_list1 = current_list1.and_then(|node| node.next);
        current_list2 = current_list2.and_then(|node| node.next);
    }
    dummy_head.next
    }
}