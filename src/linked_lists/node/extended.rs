use std::rc::{Rc, Weak};
use std::fmt::Debug;
use std::cell::RefCell;
use super::{ExtendedNode};
#[derive(Debug, Clone)]
pub struct ExtNode<T>
where T: Clone {
    value: T,
    next: Option<Rc<RefCell<ExtNode<T>>>>,
    previous: Option<Weak<RefCell<ExtNode<T>>>>,
}

impl<
    T: Debug
    + PartialEq
    + Clone
> ExtendedNode<T> for ExtNode<T> {
    fn new(value: T) -> Self {
        ExtNode {
            value,
            next: None,
            previous: None,
        }
    }

    fn get_value(&self) -> &T { &self.value }

    fn set_value(&mut self, value: T) { self.value = value; }

    fn get_next(&self) -> Option<&Rc<RefCell<Self>>> {
        Some(self.next.as_ref()?)
    }

    fn get_previous(&self) -> Option<&Weak<RefCell<Self>>> {
        Some(self.previous.as_ref()?)
    }

    fn get_next_mut(&mut self) -> Option<&mut Rc<RefCell<Self>>> {
        Some(self.next.as_mut()?)
    }

    fn get_previous_mut(&mut self) -> Option<&mut Weak<RefCell<Self>>> {
        Some(self.previous.as_mut()?)
    }

    fn set_next(&mut self, next: Option<Rc<RefCell<Self>>>) {
        self.next = next;
    }

    fn set_previous(&mut self, previous: Option<Weak<RefCell<Self>>>) {
        self.previous = previous;
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use crate::linked_lists::node::extended;

    use super::*;

    #[test]
    fn test_double_node_ops() {
        let mut head_node = ExtNode::new(1);
        assert_eq!(*head_node.get_value(), 1);
        head_node.set_value(2);
        assert_ne!(*head_node.get_value(), 1);
        assert_eq!(*head_node.get_value(), 2);
        let next_node = Some(Rc::new(RefCell::new(ExtNode::new(3))));
        head_node.set_next(next_node);
        assert_eq!(*head_node.get_next().unwrap().borrow_mut().get_value(), 3);
        let next_node = Rc::new(RefCell::new(ExtNode::new(5)));
        let middle_node = head_node.get_next_mut().unwrap();
        middle_node.borrow_mut().set_next(Some(next_node));
        let last_node = middle_node.borrow_mut().get_next_mut().unwrap();
        let cloned_middle_node = Some(Rc::downgrade(middle_node));
        middle_node.borrow_mut().get_next_mut().unwrap().borrow_mut().set_previous(cloned_middle_node);
        assert_eq!(*middle_node.borrow().get_next().unwrap().borrow().get_value(), 5);
    }
}

// endregion: --- Tests