use std::fmt;

use super::AvlNode;

pub struct AvlTree<K: Ord, T> {
    root: Option<Box<AvlNode<K, T>>>,
}

impl<K: Ord, T> AvlTree<K, T> {
    /// create a new empty AVL tree
    pub fn new() -> Self {
        Self { root: None }
    }

    /// get a reference to the element at the given key, if it exists
    pub fn get(&self, k: &K) -> Option<&T> {
        self.root.as_ref().and_then(|node| node.get(k))
    }

    /// get a mutable reference to the element at the given key, if it exists
    pub fn get_mut(&mut self, k: &K) -> Option<&mut T> {
        self.root.as_mut().and_then(|node| node.get_mut(k))
    }

    /// insert a new element into the tree, returning the previous element if found
    pub fn put(&mut self, k: K, e: T) -> Option<T> {
        match &mut self.root {
            Some(node) => node.put(k, e),
            None => {
                self.root = Some(Box::new(AvlNode::new(k, e, None, None)));
                None
            }
        }
    }

    /// remove a node from the tree, returning the removed element if found
    pub fn remove(&mut self, k: &K) -> Option<T> {
        AvlNode::remove_node(&mut self.root, k)
    }

    pub fn inorder<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        if let Some(ref root) = self.root {
            root.inorder(&mut f);
        }
    }
}

impl<K, T> fmt::Display for AvlTree<K, T>
where
    K: Ord + fmt::Debug,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref root) = self.root {
            root.print_tree(f, String::new(), None)
        } else {
            write!(f, "[Empty Tree]")
        }
    }
}
