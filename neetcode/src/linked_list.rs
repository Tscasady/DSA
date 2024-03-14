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

    fn new_list(mut nums: Vec<i32>) -> Self {
        let mut prev = None;
        while let Some(num) = nums.pop() {
            let mut node = Self::new(num);
            node.next = prev;
            prev = Some(Box::new(node));
        }
        *prev.unwrap()
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut result = vec![];
        let mut curr = self;
        result.push(curr.val);
        while let Some(node) = &curr.next {
            result.push(node.val);
            curr = node;
        }
        result
    }
}

// pub fn add_two_numbers(
//     l1: Option<Box<ListNode>>,
//     l2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     let mut carry = 0;
//     let mut result = ListNode::new(0);
//     match (l1, l2) {
//         (Some(node1), Some(node2)) => {
//             let val = (node1.val + node2.val) % 10;
//             carry = (node1.val + node2.val) / 10;
//             result = ListNode::new(val);
//             result.next = add_two_numbers(node1.next, node2.next);
//             match result.next {
//                 Some(ref mut node) => {
//                     node.val += carry;
//                 }
//                 None => {
//                     if carry > 0 {
//                         result.next = Some(Box::new(ListNode::new(carry)));
//                     }
//                 }
//             }
//             carry = 0
//         }
//         (None, Some(node)) | (Some(node), None) => {
//             let val =  % 10;
//             carry = (node1.val + node2.val) / 10;
//             result = ListNode::new(node.val)},
//         (None, None) => {
//             if carry != 0 {
//                 result = ListNode::new(carry)
//             } else {
//                 return None;
//             }
//         }
//     }
//
//     Some(Box::new(result))
// }
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_two_numbers_recursive(l1, l2, 0)
}

pub fn add_two_numbers_recursive(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(node1), Some(node2)) => {
            let val = (node1.val + node2.val + carry) % 10;
            carry = (node1.val + node2.val + carry) / 10;
            let mut new_node = ListNode::new(val);
            new_node.next = add_two_numbers_recursive(node1.next, node2.next, carry);
            Some(Box::new(new_node))
        }
        (Some(node), None) | (None, Some(node)) => {
            let val = (carry + node.val) % 10;
            carry = (carry + node.val) / 10;
            let mut new_node = ListNode::new(val);
            new_node.next = add_two_numbers_recursive(node.next, None, carry);
            Some(Box::new(new_node))
        }
        (None, None) => {
            if carry != 0 {
                Some(Box::new(ListNode::new(carry)))
            } else {
                None
            }
        }
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
    //is this really needed? the operations of iterating through the list and counting and dividing
    //and walking through again seems similar

    let end = while let Some(node) = head {
        head = &mut node.next
    };
}

pub fn remove_at(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    todo!()
}

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_list_test() {
        let list1 = ListNode::new_list(vec![2, 4, 3]);
        let list2 = ListNode::new_list(vec![5, 6, 4]);

        assert_eq!(list1.val, 2);
        assert_eq!(list1.next.unwrap().val, 4);
        assert_eq!(list2.val, 5);
        assert_eq!(list2.next.unwrap().val, 6);
    }

    #[test]
    fn add_two_test() {
        let list1 = Some(Box::new(ListNode::new_list(vec![2, 4, 3])));
        let list2 = Some(Box::new(ListNode::new_list(vec![5, 6, 4])));

        assert_eq!(
            add_two_numbers(list1.clone(), list2.clone())
                .unwrap()
                .to_vec(),
            vec![7, 0, 8]
        );
    }

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
