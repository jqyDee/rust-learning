use std::fmt;
use std::{cmp::max, mem};

pub struct AvlNode<K: Ord, T> {
    key: K,
    element: T,
    left: Option<Box<AvlNode<K, T>>>,
    right: Option<Box<AvlNode<K, T>>>,
    height: isize,
}

impl<K: Ord, T> AvlNode<K, T> {
    // ---------- PUBLIC ----------
    pub fn new(key: K, element: T, left: Option<Box<Self>>, right: Option<Box<Self>>) -> Self {
        Self {
            key,
            element,
            left,
            right,
            height: 1,
        }
    }

    pub fn get(&self, k: &K) -> Option<&T> {
        self.search(k).and_then(|node| Some(&node.element))
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut T> {
        self.search_mut(k).map(|node| &mut node.element)
    }

    pub fn put(&mut self, k: K, e: T) -> Option<T> {
        let mut ret: Option<T> = None;

        if k == self.key {
            let old = mem::replace(&mut self.element, e);
            return Some(old);
        } else if k < self.key {
            match &mut self.left {
                Some(node) => {
                    ret = node.put(k, e);
                }
                None => {
                    self.left = Some(Box::new(AvlNode::new(k, e, None, None)));
                }
            }
        } else {
            match &mut self.right {
                Some(node) => {
                    ret = node.put(k, e);
                }
                None => {
                    self.right = Some(Box::new(AvlNode::new(k, e, None, None)));
                }
            }
        }

        self.rebalance();
        ret
    }

    pub fn inorder<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        if let Some(ref l) = self.left {
            l.inorder(f);
        }
        f(&self.element);
        if let Some(ref r) = self.right {
            r.inorder(f);
        }
    }

    // ---------- PRIVATE ----------
    #[allow(dead_code)]
    fn search(&self, k: &K) -> Option<&Self> {
        if *k == self.key {
            Some(self)
        } else if *k < self.key {
            self.left.as_ref().and_then(|node| node.search(k))
        } else {
            self.right.as_ref().and_then(|node| node.search(k))
        }
    }

    fn search_mut(&mut self, k: &K) -> Option<&mut Self> {
        if *k == self.key {
            Some(self)
        } else if *k < self.key {
            self.left.as_mut().and_then(|node| node.search_mut(k))
        } else {
            self.right.as_mut().and_then(|node| node.search_mut(k))
        }
    }

    fn get_height(node: &Option<Box<Self>>) -> isize {
        node.as_ref().map_or(0, |exst| exst.height)
    }

    fn update_height(&mut self) {
        self.height = 1 + max(Self::get_height(&self.left), Self::get_height(&self.right));
    }

    fn balance_factor(&self) -> isize {
        Self::get_height(&self.left) - Self::get_height(&self.right)
    }

    fn rotate_left(&mut self) {
        if let Some(mut old_right) = self.right.take() {
            let new_right_left = old_right.left.take();
            // swap
            mem::swap(self, &mut *old_right);
            // relink
            old_right.right = new_right_left;
            old_right.update_height();

            self.left = Some(old_right);
            self.update_height();
        }
    }

    fn rotate_right(&mut self) {
        if let Some(mut old_left) = self.left.take() {
            let new_left_right = old_left.right.take();
            // swap
            mem::swap(self, &mut *old_left);
            // relink
            old_left.left = new_left_right;
            old_left.update_height();
            self.right = Some(old_left);
            self.update_height();
        }
    }

    fn rebalance(&mut self) {
        self.update_height();
        let balance = self.balance_factor();

        if balance > 1 {
            // Left heavy
            if let Some(left_node) = &mut self.left {
                if left_node.balance_factor() < 0 {
                    // Left-Right Case
                    left_node.rotate_left();
                }
            }
            self.rotate_right();
        } else if balance < -1 {
            // Right heavy
            if let Some(right_node) = &mut self.right {
                if right_node.balance_factor() > 0 {
                    // Right-Left Case
                    right_node.rotate_right();
                }
            }
            self.rotate_left();
        }
    }

    pub fn remove_node(node: &mut Option<Box<Self>>, k: &K) -> Option<T> {
        let mut current = node.take()?;
        let removed;

        if *k == current.key {
            if current.left.is_none() || current.right.is_none() {
                *node = current.left.or(current.right);
                return Some(current.element);
            } else {
                // find the immediate successor
                let min = Self::pop_min(&mut current.right);
                // swap element and key with min
                let old_element = mem::replace(&mut current.element, min.element);
                current.key = min.key;
                current.rebalance();
                *node = Some(current);
                return Some(old_element);
            }
        } else if *k < current.key {
            removed = Self::remove_node(&mut current.left, k);
            current.rebalance();
            *node = Some(current);
        } else {
            removed = Self::remove_node(&mut current.right, k);
            current.rebalance();
            *node = Some(current);
        }

        removed
    }

    fn pop_min(node: &mut Option<Box<Self>>) -> Self {
        // save, as we know already this exists
        let mut current = node.take().unwrap();
        // check if only right child
        if current.left.is_none() {
            *node = current.right.take();
            return *current;
        }
        // check left tree for immediate successor
        let min = Self::pop_min(&mut current.left);
        // rebalance after min is popped
        current.rebalance();
        *node = Some(current);
        min
    }
}

impl<K, T> AvlNode<K, T>
where
    K: Ord + fmt::Debug,
    T: fmt::Debug,
{
    pub fn print_tree(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: String,
        is_left: Option<bool>,
    ) -> fmt::Result {
        // 1. Visit Right Subtree first (so it renders on top in the terminal)
        if let Some(ref right) = self.right {
            let mut next_prefix = prefix.clone();
            next_prefix.push_str(if is_left == Some(true) {
                "│   "
            } else {
                "    "
            });
            right.print_tree(f, next_prefix, Some(false))?;
        }

        // 2. Print Current Node with structural connectors
        write!(f, "{}", prefix)?;
        match is_left {
            Some(true) => write!(f, "└── ")?,
            Some(false) => write!(f, "┌── ")?,
            None => write!(f, "─── ")?,
        }
        writeln!(
            f,
            "{:?}:{:?} (h:{}, b:{})",
            self.key,
            self.element,
            self.height,
            self.balance_factor()
        )?;

        // 3. Visit Left Subtree (renders on bottom)
        if let Some(ref left) = self.left {
            let mut next_prefix = prefix.clone();
            next_prefix.push_str(if is_left == Some(false) {
                "│   "
            } else {
                "    "
            });
            left.print_tree(f, next_prefix, Some(true))?;
        }

        Ok(())
    }
}
