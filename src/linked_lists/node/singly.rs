use std::fmt::Debug;
use super::{SinglyNode};

#[derive(Debug, Clone)]
pub struct SNode<T>
where T: Clone {
    value: T,
    next: Option<Box<SNode<T>>>,
}

impl<
    T: Debug
    + PartialEq
    + Clone
> SinglyNode<T> for SNode<T> {
    fn new(value: T) -> Self {
        SNode {
            value,
            next: None,
        }
    }

    fn get_value(&self) -> &T {
        &self.value
    }

    fn get_next(&self) -> Option<&Self> {
        Some(self.next.as_ref()?)
    }

    fn get_next_mut(&mut self) -> Option<&mut Self> {
        Some(self.next.as_mut()?)
    }

    fn set_value(&mut self, value: T) { self.value = value; }

    fn set_next(&mut self, next: Option<Box<Self>>) {
        self.next = next;
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_node_ops() {
        let mut node = SNode::new(1);
        assert_eq!(*node.get_value(), 1);
        node.set_value(2);
        assert_ne!(*node.get_value(), 1);
        assert_eq!(*node.get_value(), 2);
        let next_node = SNode::new(3);
        node.set_next(Some(Box::new(next_node)));
        assert_eq!(*node.get_next().unwrap().get_value(), 3);
        let next_node = Box::new(SNode::new(5));
        node.get_next_mut().unwrap().set_next(Some(next_node));
        let latest_node = node.get_next().unwrap().get_next().unwrap();
        assert_eq!(*latest_node.get_value(), 5);
    }
}

// endregion: --- Tests