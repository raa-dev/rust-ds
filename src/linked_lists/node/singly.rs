//! Node for singly linked list.
//!
//!  `SNode` is a node in the linear linked list, containing a custom-type value and a pointer to the next node.
use std::fmt::Debug;
#[derive(Debug, Clone)]
pub struct SNode<T> {
    value: T,
    next: Option<Box<SNode<T>>>,
}

impl<T> SNode<T>
where
    T: Debug + PartialEq,
{
    pub fn new(value: T) -> Self {
        SNode { value, next: None }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_next(&self) -> &Option<Box<Self>> {
        &self.next
    }

    pub fn get_next_mut(&mut self) -> &mut Option<Box<Self>> {
        &mut self.next
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    pub fn set_next(&mut self, next: Option<Box<Self>>) {
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
        assert_eq!(*node.get_next().as_ref().unwrap().get_value(), 3);
        let next_node = Box::new(SNode::new(5));
        node.get_next_mut()
            .as_mut()
            .unwrap()
            .set_next(Some(next_node));
        let latest_node = node
            .get_next()
            .as_ref()
            .unwrap()
            .get_next()
            .as_ref()
            .unwrap();
        assert_eq!(*latest_node.get_value(), 5);
    }
}

// endregion: --- Tests
