/// This crate provides a binary tree structure for
/// this project. 

use std::rc::Rc;
use std::cell::RefCell;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

/// This is the enum for the direction of the nodes. It
/// allows us to traverse using types that are named
/// rather than using magic values.
#[derive(Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
}

/// Node reference type. Gets a new reference on a clone.
pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

/// The main node struct that allows for construction of
/// binary trees. Always contains a value T, and can 
/// have 0-2 child nodes, as indicated by the Option<>
/// type.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
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

    /// Create a child node of self with a value in a certain direction, provided
    /// that the node does not have a child in that direction already.
    pub fn create_child(&mut self, value: T, direction: Direction) -> Result<()> {
        match direction {
            Direction::Left => {
                match self.left {
                    Some(_) => return Err(anyhow!("Failed to override occupied child.")),
                    None => {
                        self.left = Some(Node::new(value).noderef());
                        return Ok(()); 
                    }
                }
            },
            Direction::Right => {
                match self.right {
                    Some(_) => return Err(anyhow!("Failed to override occupied child.")),
                    None => {
                        self.right = Some(Node::new(value).noderef());
                        return Ok(()); 
                    }
                }
            },
        };
    }

    /// Returns whether the current node has a child node.
    pub fn is_leaf(&self) -> bool {
        match (self.left.clone(), self.right.clone()) {
            (None, None) => true,
            _ => false,
        }
    }

    /// Get an optional reference to a child of self, dependent on direction.
    pub fn get_child_reference(&self, direction: Direction) -> Option<NodeRef<T>> {
        match direction {
            Direction::Left => match &self.left {
                Some(child) => Some(child.clone()),
                None => None,
            },
            Direction::Right => match &self.right {
                Some(child) => Some(child.clone()),
                None => None,
            },
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

    #[test]
    fn create_child() {
        let mut a = Node::new(10);
        a.create_child(20, Direction::Left).expect("");

        let b = Node {
            value: 10,
            left: Some(Node {
                value: 20,
                left: None,
                right: None,
            }.noderef()),
            right: None,
        };

        assert!(a == b)
    }

    #[test]
    fn is_leaf() {
        let mut root = Node::new(1);
        root.create_child(2, Direction::Left).unwrap();
        assert!(root.is_leaf() == false);
        assert!(root.left.unwrap().borrow().is_leaf() == true);
    }
}
