/// This crate provides a binary tree structure for
/// this project. 

use std::rc::Rc;
use std::cell::RefCell;

/// Node reference type. Gets a new reference on a clone.
pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

/// The main node struct that allows for construction of
/// binary trees. Always contains a value T, and can 
/// have 0-2 child nodes, as indicated by the Option<>
/// type.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Node<T> {
    value: T,
    left: Option<NodeRef<T>>,
    right: Option<NodeRef<T>>,
}
impl<T> Node<T> {
    /// Create a single node node
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    /// Get a clonable, interior-mutable reference to self.
    pub fn noderef(self) -> NodeRef<T> {
        Rc::new(RefCell::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node() {
        assert!(Node::new(10) == Node{ value: 10, left: None, right: None })
    }

    #[test]
    fn noderef() {
        assert!(Node::new(10).noderef() == Rc::new(RefCell::new(Node::new(10))))
    }

}
