use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cmp::{min, max};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type Node = Option<Rc<RefCell<TreeNode>>>;

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

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref node) = root {
        let mut left = node.borrow_mut().left.clone();
        let mut right = node.borrow_mut().right.clone();
        node.borrow_mut().left = invert_tree(right.take());
        node.borrow_mut().right = invert_tree(left.take())
    }
    root
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut total = 0;
    if let Some(ref node) = root {
        total += 1;
        total += std::cmp::max(
            max_depth(node.borrow().left.clone()),
            max_depth(node.borrow().right.clone()),
        )
    }
    total
}

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut diameter = 0;
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let left_height = node_diameter(left, &mut diameter);
        let right = node.borrow().right.clone();
        let right_height = node_diameter(right, &mut diameter);
        let height = 1 + std::cmp::max(left_height, right_height);
        diameter = std::cmp::max(left_height + right_height + 2, diameter);
    }
    diameter
}

pub fn node_diameter(root: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
    match root {
        Some(node) => {
            let left = node.borrow().left.clone();
            let left_depth = node_diameter(left, diameter);
            let right = node.borrow().right.clone();
            let right_depth = node_diameter(right, diameter);
            let depth = 1 + std::cmp::max(left_depth, right_depth);
            *diameter = std::cmp::max(left_depth + right_depth + 2, *diameter);
            depth
        }
        None => -1,
    }
}

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p_node), Some(q_node)) => {
            if p_node.borrow().val == q_node.borrow().val {
                is_same_tree(p_node.borrow().left.clone(), q_node.borrow().left.clone())
                    && is_same_tree(p_node.borrow().right.clone(), q_node.borrow().right.clone())
            } else {
                false
            }
        }
        (None, None) => true,
        _ => false,
    }
}

pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (root.clone(), sub_root.clone()) {
        (Some(node), Some(sub_node)) if node.borrow().val == sub_node.borrow().val => {
            if is_subtree_check(node.borrow().left.clone(), sub_node.borrow().left.clone())
                && is_subtree_check(node.borrow().right.clone(), sub_node.borrow().right.clone())
            {
                return true;
            }
        }
        _ => {}
    }

    if let Some(node) = root {
        return is_subtree(node.borrow().left.clone(), sub_root.clone())
            || is_subtree(node.borrow().right.clone(), sub_root.clone());
    }

    false
}

pub fn is_subtree_check(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (root, sub_root) {
        (Some(node), Some(sub_node)) => {
            if node.borrow().val == sub_node.borrow().val {
                is_subtree_check(node.borrow().left.clone(), sub_node.borrow().left.clone())
                    && is_subtree_check(
                        node.borrow().right.clone(),
                        sub_node.borrow().right.clone(),
                    )
            } else {
                false
            }
        }
        (None, None) => true,
        _ => false,
    }
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let (left, right) = (
                lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone()),
                lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone()),
            );
            match (left, right) {
                (Some(_), Some(_)) => return Some(node),
                (Some(left_node), None) => {
                    if node.borrow().val == p.as_ref().unwrap().borrow().val
                        || node.borrow().val == q.as_ref().unwrap().borrow().val
                    {
                        return Some(node);
                    } else {
                        return Some(left_node);
                    }
                }
                (None, Some(right_node)) => {
                    if node.borrow().val == p.as_ref().unwrap().borrow().val
                        || node.borrow().val == q.as_ref().unwrap().borrow().val
                    {
                        return Some(node);
                    } else {
                        return Some(right_node);
                    }
                }
                (None, None) => {
                    if node.borrow().val == p.as_ref().unwrap().borrow().val
                        || node.borrow().val == q.as_ref().unwrap().borrow().val
                    {
                        return Some(node);
                    } else {
                        return None;
                    }
                }
            }
        }
        None => None,
    }
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut solution = vec![];
    let mut queue = VecDeque::new();
    let mut next_level = VecDeque::new();
    queue.push_back(root);
    loop {
        let mut level_values = Vec::new();
        while let Some(node) = queue.pop_front().flatten() {
            level_values.push(node.borrow().val);
            next_level.push_back(node.borrow().left.clone());
            next_level.push_back(node.borrow().right.clone())
        }
        if next_level.is_empty() {
            if !level_values.is_empty() {
                solution.push(std::mem::take(&mut level_values))
            }
            break;
        } else {
            queue = std::mem::take(&mut next_level);
            solution.push(std::mem::take(&mut level_values))
        }
    }
    solution
}

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = root.as_ref().unwrap().borrow().val;
    good_nodes_recursion(root, max)
}

pub fn good_nodes_recursion(root: Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
    match root {
        Some(node) => {
            if node.borrow().val < max {
                good_nodes_recursion(node.borrow().left.clone(), max)
                    + good_nodes_recursion(node.borrow().right.clone(), max)
            } else {
                let max = node.borrow().val;
                1 + good_nodes_recursion(node.borrow().left.clone(), max)
                    + good_nodes_recursion(node.borrow().right.clone(), max)
            }
        }
        None => 0,
    }
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_recursion(root, i64::MIN, i64::MAX)
}

pub fn is_valid_bst_recursion(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
    match root {
        Some(node) => {
            if (node.borrow().val as i64) > min && (node.borrow().val as i64) < max {
                is_valid_bst_recursion(node.borrow().left.clone(), min, node.borrow().val as i64) && 
                    is_valid_bst_recursion(node.borrow().right.clone(), node.borrow().val as i64, max) 
            } else { false }
        },
        None => {
            true
        }
    }
}

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let (value, _) = kth_recursion(root, k, 0);
    return value.unwrap()
}

pub fn kth_recursion(root: Option<Rc<RefCell<TreeNode>>>, k: i32, mut count: i32) -> (Option<i32>, i32) {
    match root {
        Some(node) => {
            let (value, count) = kth_recursion(node.borrow().left.clone(), k, count);
            if value.is_some() {
                return (value, count)
            }

            if count + 1 == k {
                return (Some(node.borrow().val), 0)
            }

            return kth_recursion(node.borrow().right, k, count + 1);
        },
        None => { (None, count) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invert_test() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        if let Some(ref binary_tree) = tree {
            binary_tree.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            binary_tree.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        }
        let inverted_tree = invert_tree(tree);
        assert_eq!(
            inverted_tree
                .as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            1
        );
        assert_eq!(
            inverted_tree
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            3
        );
    }

    #[test]
    fn max_depth_test() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        if let Some(ref binary_tree) = tree {
            binary_tree.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            binary_tree.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        }

        assert_eq!(max_depth(tree), 2);
    }

    #[test]
    fn is_same_tree_test() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        if let Some(ref binary_tree) = tree {
            binary_tree.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            binary_tree.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        }

        let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        if let Some(ref binary_tree) = tree2 {
            binary_tree.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            binary_tree.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        }

        assert!(is_same_tree(tree, tree2))
    }
}
