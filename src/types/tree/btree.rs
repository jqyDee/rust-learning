use std::{fmt, fmt::Display};

use super::{bnode::BNode, math_token::MathToken};

#[derive(Debug)]
pub struct BTree<T> {
    root: Option<Box<BNode<T>>>,
    pub size: usize,
}

impl<T> BTree<T> {
    pub fn from(root: Box<BNode<T>>) -> Self {
        let mut size: usize = 0;
        root.preorder(&mut |_| {
            size += 1;
        });

        Self {
            size: size,
            root: Some(root),
        }
    }

    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }

    pub fn add_root(&mut self, element: T) -> &mut BNode<T> {
        let new_root = Box::new(BNode::new(element));
        self.root = Some(new_root);
        self.size += 1;
        self.root.as_deref_mut().unwrap()
    }

    pub fn find(&self, element: &T) -> Option<&BNode<T>>
    where
        T: PartialOrd,
    {
        if let Some(ref r) = self.root {
            return r.find(element);
        }
        None
    }
}

/// Traversal methods
impl<T> BTree<T> {
    pub fn preorder<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        if let Some(ref root) = self.root {
            root.preorder(&mut f);
        }
    }

    pub fn inorder<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        if let Some(ref root) = self.root {
            root.inorder(&mut f);
        }
    }

    pub fn postorder<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        if let Some(ref root) = self.root {
            root.postorder(&mut f);
        }
    }
}

impl Display for BTree<MathToken> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(r_node) = &self.root {
            write!(f, "{}", r_node.to_math_string())
        } else {
            write!(f, "nil")
        }
    }
}

impl BTree<MathToken> {
    pub fn evaluate(&self) -> f64 {
        if let Some(root) = &self.root {
            root.evaluate()
        } else {
            0.0
        }
    }
}
