/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => {
                // 如果根节点存在，使用节点的insert方法
                node.insert(value);
            }
            None => {
                // 如果根节点不存在，创建新的根节点
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        self.search_recursive(&self.root, value)
    }
    
    // 递归搜索辅助方法
    fn search_recursive(&self, node: &Option<Box<TreeNode<T>>>, value: T) -> bool {
        match node {
            Some(n) => {
                match value.cmp(&n.value) {
                    Ordering::Equal => true,        // 找到了
                    Ordering::Less => self.search_recursive(&n.left, value),   // 在左子树中搜索
                    Ordering::Greater => self.search_recursive(&n.right, value), // 在右子树中搜索
                }
            }
            None => false, // 节点为空，未找到
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 值小于当前节点，插入到左子树
                match self.left {
                    Some(ref mut left_node) => {
                        left_node.insert(value);
                    }
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
            Ordering::Greater => {
                // 值大于当前节点，插入到右子树
                match self.right {
                    Some(ref mut right_node) => {
                        right_node.insert(value);
                    }
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
            Ordering::Equal => {
                // 值等于当前节点，不插入（避免重复）
                // BST通常不允许重复值
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


