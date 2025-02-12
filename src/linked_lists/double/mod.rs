use super::ExtNode as Node;
use super::{Error, Result};
use std::cell::RefCell;
use std::clone::Clone;
use std::fmt::Debug;
use std::rc::Rc;

/// `Double` is a double linked list referencing the head, the tail node node and the length of the list.
#[derive(Debug)]
pub struct Double<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> Default for Double<T>
where
    T: Debug + PartialEq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Double<T>
where
    T: Debug + PartialEq + Clone,
{
    /// Create a new instance of the `Double` linked list.
    pub fn new() -> Self {
        Double {
            head: None,
            tail: None,
            len: 0,
        }
    }
    /// Append a new value to the end of the list.
    pub fn append(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        match self.head.as_mut() {
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
            Some(current) => {
                let mut current = current.clone();
                while current.borrow().get_next().is_some() {
                    current = unsafe {
                        current
                            .as_ptr()
                            .as_ref()
                            .unwrap()
                            .get_next()
                            .clone()
                            .unwrap()
                    };
                }
                new_node
                    .borrow_mut()
                    .set_previous(Some(Rc::downgrade(&current)));
                self.tail = Some(new_node.clone());
                current.borrow_mut().set_next(Some(new_node));
            }
        }
        self.len += 1;
    }
    /// Remove a node from the list.
    /// Returns true if the value is found and removed.
    /// If the value is not found, it returns an error.
    pub fn remove(&mut self, value: T) -> Result<bool> {
        if self.is_empty() {
            return Err(Error::EmptyList);
        }

        match self.head.as_mut() {
            None => return Err(Error::EmptyList),
            Some(current) => {
                let mut current = current.clone();
                if *current.borrow().get_value() == value {
                    let next = current.borrow_mut().get_next_mut().take();
                    self.head = std::mem::replace(&mut self.head, next);
                    if let Some(next) = self.head.as_mut() {
                        next.borrow_mut().set_previous(None);
                    }
                    self.len -= 1;
                    return Ok(true);
                }

                while current.borrow().get_next().is_some() {
                    if let Some(next) = current.borrow_mut().get_next_mut().take() {
                        if *next.borrow().get_value() == value {
                            if let Some(next_next) = next.borrow().get_next() {
                                next_next
                                    .borrow_mut()
                                    .set_previous(Some(Rc::downgrade(&current)));
                                unsafe {
                                    current
                                        .as_ptr()
                                        .as_mut()
                                        .unwrap()
                                        .get_next_mut()
                                        .replace(next_next.clone());
                                }
                            }
                            self.len -= 1;
                            return Ok(true);
                        }
                    }
                    current = unsafe {
                        current
                            .as_ptr()
                            .as_ref()
                            .unwrap()
                            .get_next()
                            .clone()
                            .unwrap()
                    };
                }
                if current.borrow().get_next().is_none() && *current.borrow().get_value() == value {
                    let previous = current
                        .borrow()
                        .get_previous()
                        .clone()
                        .unwrap()
                        .upgrade()
                        .unwrap();
                    previous.borrow_mut().set_next(None);
                    self.tail = Some(previous);
                    self.len -= 1;
                    return Ok(true);
                }
            }
        }
        Err(Error::ValueNotFound)
    }
    /// Search for a value in the list, returns true if the value is found.
    /// If the value is not found, it returns an error.
    pub fn search(&self, value: T) -> Result<bool> {
        match self.head.as_ref() {
            None => return Err(Error::EmptyList),
            Some(current) => {
                let mut current = current;
                if current.borrow().get_next().is_none() {
                    return Ok(*current.borrow().get_value() == value);
                }
                while current.borrow().get_next().is_some() {
                    if *current.borrow().get_value() == value {
                        return Ok(true);
                    }
                    current = unsafe {
                        current
                            .as_ptr()
                            .as_ref()
                            .unwrap()
                            .get_next()
                            .as_ref()
                            .unwrap()
                    };
                }
                if current.borrow().get_next().is_none() {
                    return Ok(*current.borrow().get_value() == value);
                }
            }
        }
        Err(Error::ValueNotFound)
    }
    /// Update a value in the list, returns true if the value is found and updated.
    /// If the value is not found, it returns an error.
    pub fn update(&mut self, old_value: T, new_value: T) -> Result<bool> {
        match self.head.as_ref() {
            None => return Err(Error::EmptyList),
            Some(current) => {
                let mut current = current;
                if current.borrow().get_next().is_none() {
                    return Ok(*current.borrow().get_value() == old_value);
                }
                while current.borrow().get_next().is_some() {
                    if *current.borrow().get_value() == old_value {
                        current.borrow_mut().set_value(new_value);
                        return Ok(true);
                    }
                    current = unsafe {
                        current
                            .as_ptr()
                            .as_ref()
                            .unwrap()
                            .get_next()
                            .as_ref()
                            .unwrap()
                    };
                }
                if current.borrow().get_next().is_none() {
                    current.borrow_mut().set_value(new_value);
                    return Ok(true);
                }
            }
        }
        Err(Error::ValueNotFound)
    }
    /// Create a new instance of the double linked list from a vector.
    pub fn from_vec(values: Vec<T>) -> Self {
        let mut list = Self::new();
        for value in values {
            list.append(value);
        }
        list
    }
    /// Remove the last node from the list.
    /// Returns the value of the removed node.
    /// If the list is empty, it returns an error.
    pub fn pop(&mut self) -> Result<Option<T>> {
        if self.is_empty() {
            return Err(Error::EmptyList);
        }

        match self.tail.as_ref() {
            None => Err(Error::EmptyList),
            Some(tail) => {
                let tail = tail.clone();
                let previous = tail
                    .borrow_mut()
                    .get_previous_mut()
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap();
                previous.borrow_mut().set_next(None);
                self.tail = Some(previous);
                self.len -= 1;
                return Ok(Some(tail.borrow().get_value().clone()));
            }
        }
    }

    pub fn print(&self) {
        if self.is_empty() {
            println!("{}", Error::EmptyList);
        }

        let mut index = 0;
        while index < self.len {
            let _ = self.get(index).map(|value| {
                println!("{:?}", value.unwrap());
                index += 1;
            });
        }
    }
    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    /// Get an element from the list by index.
    /// Returns an error if the list is empty or the index is out of bounds.
    /// If the index is valid, it returns the value of the node.
    pub fn get(&self, index: usize) -> Result<Option<T>> {
        if self.is_empty() {
            return Err(Error::EmptyList);
        }
        let mut current = &self.head;
        let mut current_index = 0;

        while current_index <= index {
            let current_ref = current.as_ref().unwrap();
            if current_index == index {
                return Ok(Some(current_ref.borrow().get_value().clone()));
            }
            current = unsafe { current_ref.as_ptr().as_ref().unwrap().get_next() };
            current_index += 1;
        }

        Err(Error::IndexOutOfBounds)
    }
}

// region:    --- Tests

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_double_linked_list_ops() {
        let mut list = Double::new();
        list.append(1);
        list.append(2);
        assert!(!list.is_empty());
        let value = list.pop().unwrap().unwrap();
        assert_eq!(value, 2);
        assert_eq!(list.len, 1);
        let mut list2 = Double::from_vec(vec!["hello", "world", "rust"]);
        assert_eq!(list2.get(1).unwrap().unwrap(), "world");
        assert!(!list2.is_empty());
        assert!(list2.search("rust").unwrap());
        assert!(list2.update("world", "earth").unwrap());
        assert_eq!(list2.get(1).unwrap().unwrap(), "earth");
        assert!(list2.remove("earth").unwrap());
        list2.print();
    }

    #[test]
    fn test_double_linked_list_errors() {
        let mut list: Double<i32> = Double::new();
        assert!(list.is_empty());
        assert!(list.search(6).is_err());
        assert!(list.update(6, 7).is_err());
        assert!(list.pop().is_err());
        assert!(list.remove(6).is_err());
    }
}

// endregion: --- Tests
