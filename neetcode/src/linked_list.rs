use std::mem;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_list(mut head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;

    while let Some(curr) = head {
        prev = Some(Box::new(ListNode {
            next: prev,
            val: curr.val,
        }));
        head = &mut curr.next
    }
    prev
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(list1_node), None) => return Some(list1_node),
        (None, Some(list2_node)) => return Some(list2_node),
        (Some(list1_node), Some(list2_node)) => {
            if list1_node.val < list2_node.val {
                return Some(Box::new(ListNode {
                    val: list1_node.val,
                    next: merge_two_lists(list1_node.next, Some(list2_node)),
                }));
            } else {
                return Some(Box::new(ListNode {
                    val: list2_node.val,
                    next: merge_two_lists(Some(list1_node), list2_node.next),
                }));
            }
        }
    }
}

pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {

    //split the list at the midpoint using s/f pointers
        

    let end = while let Some(node) = head {
        head = &mut node.next
    };

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_test() {
        let list1 = Some(Box::new(ListNode::new(1)));
        let list2 = Some(Box::new(ListNode::new(2)));
        assert_eq!(1, merge_two_lists(list1, list2).unwrap().val)
    }

    #[test]
    fn lists_test_2() {
        let list1 = Some(Box::new(ListNode::new(1)));
        let list2 = Some(Box::new(ListNode::new(2)));
        assert_eq!(2, merge_two_lists(list1, list2).unwrap().next.unwrap().val)
    }

    #[test]
    fn empty_list() {
        let list1 = None;
        let list2 = None;
        assert!(merge_two_lists(list1, list2).is_none())
    }
    
    #[test]
    fn reorder_list_test() {
        let mut list = Some(Box::new(ListNode::new(1)));
        if let Some(ref mut node) = list {
            node.next = Some(Box::new(ListNode::new(2)))
        };
        reorder_list(&mut list);
        assert_eq!(list.unwrap().as_ref().val, 1);
    }
}
