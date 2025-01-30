use super::SNode as Node;
use super::{Error, Result};
use std::fmt::Debug;

/// `Singly` is a singly linked list that contains a reference to the head node and the length of the list.
#[derive(Debug)]
pub struct Singly<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Singly<T>
where
    T: Debug + PartialEq + Clone,
{
    /// Creates a new `Singly` list.
    pub fn new() -> Self {
        Singly { head: None, len: 0 }
    }
    /// Append a new value to the end of the list.
    pub fn append(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));

        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
            Some(mut current) => {
                while current.get_next().is_some() {
                    current = current.get_next_mut().as_mut().unwrap();
                }
                current.set_next(Some(new_node));
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
            Some(mut current) => {
                if *current.get_value() == value {
                    self.head = current.get_next_mut().take();
                    self.len -= 1;
                    return Ok(true);
                }

                while current.get_next().is_some() {
                    if let Some(next) = current.get_next_mut() {
                        if *next.get_value() == value {
                            let next_next = next.get_next_mut().take();
                            current.set_next(next_next);
                            self.len -= 1;
                            return Ok(true);
                        }
                    }
                    current = current.get_next_mut().as_mut().unwrap();
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
                if current.get_next().is_none() {
                    return Ok(*current.get_value() == value);
                }
                while current.get_next().is_some() {
                    if *current.get_value() == value {
                        return Ok(true);
                    }
                    current = current.get_next().as_ref().unwrap();
                }
            }
        }
        Err(Error::ValueNotFound)
    }
    /// Update a value in the list, returns true if the value is found and updated.
    /// If the value is not found, it returns an error.
    pub fn update(&mut self, old_value: T, new_value: T) -> Result<bool> {
        match self.head.as_mut() {
            None => return Err(Error::EmptyList),
            Some(mut current) => {
                while current.get_next().is_some() {
                    if *current.get_value() == old_value {
                        current.set_value(new_value);
                        return Ok(true);
                    }
                    current = current.get_next_mut().as_mut().unwrap();
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

        match self.head.as_mut() {
            None => return Err(Error::EmptyList),
            Some(mut current) => {
                if current.get_next().is_none() {
                    let last_node = current.get_value().clone();
                    self.head = None;
                    self.len -= 1;
                    return Ok(Some(last_node));
                }

                while current.get_next().is_some() {
                    if let Some(next) = current.get_next_mut() {
                        if next.get_next().is_none() {
                            let last_node = next.get_value().clone();
                            current.set_next(None);
                            self.len -= 1;
                            return Ok(Some(last_node));
                        }
                    }
                    current = current.get_next_mut().as_mut().unwrap();
                }
            }
        }
        Ok(None)
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{:?} -> ", node.get_value());
            current = &node.get_next();
        }
        println!("None");
    }
    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    /// Get an element from the list by index.
    /// Returns an error if the list is empty or the index is out of bounds.
    /// If the index is valid, it returns the value of the node.
    pub fn get(&self, index: usize) -> Result<Option<&T>> {
        if self.is_empty() {
            return Err(Error::EmptyList);
        }

        let mut current = &self.head;
        let mut current_index = 0;

        while current_index <= index {
            if current_index == index {
                return Ok(Some(current.as_ref().unwrap().get_value()));
            }
            current = current.as_ref().unwrap().get_next();
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
    fn test_singly_list_ops() {
        let mut list = Singly::new();
        assert!(list.remove(99).is_err());
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.search(3).unwrap(), true);
        assert_eq!(list.update(3, 6).unwrap(), true);
        let list2 = Singly::from_vec(vec!["hello", "world", "rust"]);
        assert_eq!(list.pop().unwrap().unwrap(), 5);
        assert_eq!(list.pop().unwrap().unwrap(), 4);
        assert_eq!(list.pop().unwrap().unwrap(), 6);
        assert_eq!(list.pop().unwrap().unwrap(), 2);
        assert_eq!(list.pop().unwrap().unwrap(), 1);
        assert_eq!(list2.get(2).unwrap(), Some(&"rust"));
    }

    #[test]
    fn test_singly_list_errors() {
        let mut list = Singly::new();
        assert!(list.is_empty());
        assert!(list.remove(99).is_err());
        assert!(list.update(99, 100).is_err());
        assert!(list.pop().is_err());
        assert!(list.get(0).is_err());
        assert!(list.search(6).is_err());
    }
}

// endregion: --- Tests
