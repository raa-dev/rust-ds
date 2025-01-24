//! Node for extended linked list.
//!
//!  `ExtNode` is a node in an extended linked list, such as a double one, containing a custom-type value and a pointer to the next and the previous node.
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct ExtNode<T> {
    value: T,
    next: Option<Rc<RefCell<ExtNode<T>>>>,
    previous: Option<Weak<RefCell<ExtNode<T>>>>,
}

impl<T> ExtNode<T>
where
    T: Debug + PartialEq,
{
    pub fn new(value: T) -> Self {
        ExtNode {
            value,
            next: None,
            previous: None,
        }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    pub fn get_next(&self) -> &Option<Rc<RefCell<Self>>> {
        &self.next
    }

    pub fn get_previous(&self) -> &Option<Weak<RefCell<Self>>> {
        &self.previous
    }

    pub fn get_next_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>> {
        &mut self.next
    }

    pub fn get_previous_mut(&mut self) -> &mut Option<Weak<RefCell<Self>>> {
        &mut self.previous
    }

    pub fn set_next(&mut self, next: Option<Rc<RefCell<Self>>>) {
        self.next = next;
    }

    pub fn set_previous(&mut self, previous: Option<Weak<RefCell<Self>>>) {
        self.previous = previous;
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
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
        assert_eq!(
            *head_node
                .get_next()
                .as_ref()
                .unwrap()
                .borrow_mut()
                .get_value(),
            3
        );
        let next_node = Rc::new(RefCell::new(ExtNode::new(5)));
        let middle_node = head_node.get_next_mut().as_ref().unwrap();
        middle_node.borrow_mut().set_next(Some(next_node));
        let cloned_middle_node = Some(Rc::downgrade(&middle_node));
        let mut borrowed_middle_node = middle_node.borrow_mut();
        assert_eq!(*borrowed_middle_node.get_value(), 3);
        let last_node = borrowed_middle_node.get_next_mut().as_ref().unwrap();
        unsafe {
            last_node
                .as_ptr()
                .as_mut()
                .unwrap()
                .set_previous(cloned_middle_node)
        };
        assert_eq!(
            unsafe { *last_node.as_ptr().as_mut().unwrap().get_value() },
            5
        );
        let cloned_last_node = Some(Rc::new(&last_node));
        assert_eq!(
            unsafe {
                *cloned_last_node
                    .as_ref()
                    .unwrap()
                    .as_ptr()
                    .as_mut()
                    .unwrap()
                    .get_previous()
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .as_ptr()
                    .as_mut()
                    .unwrap()
                    .get_value()
            },
            3
        );
    }
}

// endregion: --- Tests
