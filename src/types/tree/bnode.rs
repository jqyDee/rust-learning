use super::math_token::MathToken::{self, Add, Divide, Multiply, Subtract};

#[derive(Debug)]
pub struct BNode<T> {
    pub element: T,
    pub left: Option<Box<BNode<T>>>,
    pub right: Option<Box<BNode<T>>>,
    pub leaf: bool,
}

impl<T> BNode<T> {
    pub fn new(element: T) -> Self {
        Self {
            element,
            left: None,
            right: None,
            leaf: true,
        }
    }

    pub fn insert_left(&mut self, element: T) -> &mut BNode<T> {
        let mut boxed = Box::new(BNode::new(element));
        if let Some(existing_node) = self.left.take() {
            boxed.left = Some(existing_node);
            boxed.leaf = false;
        }
        self.left = Some(boxed);
        self.leaf = false;

        self.left.as_deref_mut().unwrap()
    }

    pub fn insert_right(&mut self, element: T) -> &mut BNode<T> {
        let mut boxed = Box::new(BNode::new(element));
        if let Some(existing_node) = self.right.take() {
            boxed.right = Some(existing_node);
            boxed.leaf = false;
        }
        self.right = Some(boxed);
        self.leaf = false;

        self.right.as_deref_mut().unwrap()
    }

    pub fn find(&self, target: &T) -> Option<&Self>
    where
        T: PartialEq,
    {
        if &self.element == target {
            return Some(self);
        }

        if let Some(ref l) = self.left {
            if let Some(found) = l.find(target) {
                return Some(found);
            }
        }

        if let Some(ref r) = self.right {
            if let Some(found) = r.find(target) {
                return Some(found);
            }
        }

        None
    }
}

/// Traversal methods
impl<T> BNode<T> {
    pub fn preorder<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        f(&self.element);
        if let Some(ref l) = self.left {
            l.preorder(f);
        }
        if let Some(ref r) = self.right {
            r.preorder(f);
        }
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

    pub fn postorder<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        if let Some(ref l) = self.left {
            l.postorder(f);
        }
        if let Some(ref r) = self.right {
            r.postorder(f);
        }
        f(&self.element);
    }
}

impl BNode<MathToken> {
    pub fn to_math_string(&self) -> String {
        match &self.element {
            MathToken::Number(n) => n.to_string(),
            op => {
                let left_str = self.left.as_ref().unwrap().to_math_string();
                let right_str = self.right.as_ref().unwrap().to_math_string();
                format!("({} {} {})", left_str, op, right_str)
            }
        }
    }

    pub fn evaluate(&self) -> f64 {
        match &self.element {
            MathToken::Number(n) => *n,
            op => {
                let left_val = self.left.as_ref().unwrap().evaluate();
                let right_val = self.right.as_ref().unwrap().evaluate();

                match op {
                    Add => left_val + right_val,
                    Multiply => left_val * right_val,
                    Subtract => left_val - right_val,
                    Divide => left_val / right_val,
                    _ => unreachable!("This should not be possible"),
                }
            }
        }
    }
}
