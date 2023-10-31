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

pub fn append(mut head: Option<Box<ListNode>>, elem: i32) -> Option<Box<ListNode>> {
    let mut elem = ListNode::new(elem);
    match head {
        Some(node) => {
            elem.next = head;
            Some(Box::new(elem))
        },
        None => {
            head = Some(Box::new(elem));
            head
        }
    }
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
    //is this really needed? the operations of iterating through the list and counting and dividing
    //and walking through again seems similar 
        

    let end = while let Some(node) = head {
        head = &mut node.next
    };
}

pub fn remove_at(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let count = 1;
    let mut prev = None;
    let mut curr = head.clone();
    while count < n {
        while let Some(node) = curr {
            prev = curr; //why does this work? shouldn't this move curr?
            curr = node.next; // maybe if we tried to do something other than reassign curr here it
            // would err
            count += 1
        }
    }
    match prev {
        Some(mut node) => {
            match curr {
                Some(thing) => node.next = thing.next,
                None => node.next = None
            }
        },
        None => {
            head = head.next
        }
    }
    head
}

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    //solution 1: reverse, remove, reverse again
    reverse_list(&mut head);
    remove_at(head, n);
    reverse_list(&mut head)
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

    #[test]
    fn remove_at_test() {
        let mut list = Some(Box::new(ListNode::new(1)));
        append(list, 2);
        append(list, 3);
        remove_at(list, 1);
        assert_eq!(list.unwrap().val, 3);
        assert_eq!(list.unwrap().next.unwrap().val, 2);
        assert!(list.unwrap().next.unwrap().next.is_none())
    }
}
