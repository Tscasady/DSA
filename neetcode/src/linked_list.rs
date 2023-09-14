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

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;

    while let Some(curr) = head {
        prev = Some(Box::new(ListNode {
            next: prev,
            val: curr.val,
        }));
        head = curr.next
    }
    prev
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {

    let mut new_node;
    match mem::replace(&mut list1, None) {
        Some(list1_node) => {
            match mem::replace(&mut list2, None) {
                Some(list2_node) => {
                    if list1_node.val <= list2_node.val {
                        new_node = ListNode::new(list1_node.val);
                        new_node.next = merge_two_lists(list1_node.next, list2)
                    } else {
                        new_node = ListNode::new(list2_node.val);
                        new_node.next = merge_two_lists(list1, list2_node.next)
                    }
                },
                None => {
                    new_node = ListNode::new(list1_node.val);
                    new_node.next = merge_two_lists(list1_node.next, list2)
                }
            }
        },
        None => {
            println!("{:?}", list2);
            match mem::replace(&mut list2, None) {
                Some(list2_node) => {
                    new_node = ListNode::new(list2_node.val);
                    new_node.next = merge_two_lists(list1, list2_node.next)
                },
                None => {
                    return None
                }
            }
        }
    }
    Some(Box::new(new_node))
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
}
