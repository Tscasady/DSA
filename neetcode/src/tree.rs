use std::cell::RefCell;
use std::rc::Rc;

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
                    if node.borrow().val == p.as_ref().unwrap().borrow().val || node.borrow().val == q.as_ref().unwrap().borrow().val {
                        return Some(node)
                    } else {
                        return Some(left_node)
                    }
                },
                (None, Some(right_node)) => {
                    if node.borrow().val == p.as_ref().unwrap().borrow().val || node.borrow().val == q.as_ref().unwrap().borrow().val {
                        return Some(node)
                    } else {
                        return Some(right_node)
                    }
                },
                (None, None) => {
                    if node.borrow().val == p.as_ref().unwrap().borrow().val || node.borrow().val == q.as_ref().unwrap().borrow().val {
                        return Some(node)
                    } else {
                        return None
                    }
                },
            }
        }
        None => None,
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
