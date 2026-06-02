use std::mem;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn invert_tree<T>(mut root: Option<Box<TreeNode<T>>>) -> Option<Box<TreeNode<T>>> {
    let node = root.take();
    if let Some(mut node) = node {
        mem::swap(&mut node.left, &mut node.right);
        node.left = invert_tree(node.left.take());
        node.right = invert_tree(node.right.take());
        return Some(node);
    }
    node
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(val: i32) -> Box<TreeNode<i32>> {
        Box::new(TreeNode::new(val))
    }

    fn node(
        val: i32,
        left: Option<Box<TreeNode<i32>>>,
        right: Option<Box<TreeNode<i32>>>,
    ) -> Box<TreeNode<i32>> {
        Box::new(TreeNode { val, left, right })
    }

    #[test]
    fn inverts_balanced_tree() {
        let tree = Some(node(
            4,
            Some(node(2, Some(leaf(1)), Some(leaf(3)))),
            Some(node(7, Some(leaf(6)), Some(leaf(9)))),
        ));

        let inverted = invert_tree(tree);

        assert_eq!(
            inverted,
            Some(node(
                4,
                Some(node(7, Some(leaf(9)), Some(leaf(6)))),
                Some(node(2, Some(leaf(3)), Some(leaf(1)))),
            ))
        );
    }

    #[test]
    fn inverts_tree_with_one_child_on_each_side() {
        let tree = Some(node(
            1,
            Some(node(2, None, Some(leaf(3)))),
            Some(node(4, Some(leaf(5)), None)),
        ));

        let inverted = invert_tree(tree);

        assert_eq!(
            inverted,
            Some(node(
                1,
                Some(node(4, None, Some(leaf(5)))),
                Some(node(2, Some(leaf(3)), None)),
            ))
        );
    }

    #[test]
    fn keeps_single_node_tree_unchanged() {
        let tree = Some(leaf(42));

        assert_eq!(invert_tree(tree), Some(leaf(42)));
    }

    #[test]
    fn returns_none_for_empty_tree() {
        assert_eq!(invert_tree::<i32>(None), None);
    }
}
