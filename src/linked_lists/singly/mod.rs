use super::SNode as Node;
use super::{Error, Result};
use super::{List, SinglyNode};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Singly<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> List<T> for Singly<T>
where
    T: Debug + PartialEq + Clone,
{
    fn new() -> Self {
        Singly { head: None, len: 0 }
    }

    fn insert(&mut self, value: T) {
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

    fn remove(&mut self, value: T) -> Result<bool> {
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

    fn search(&self, value: T) -> Result<bool> {
        let mut current = &self.head;
        while let Some(node) = current {
            if *node.get_value() == value {
                return Ok(true);
            }
            current = &node.get_next();
        }
        Ok(false)
    }

    fn update(&mut self, old_value: T, new_value: T) -> Result<bool> {
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

    fn from_vec(values: Vec<T>) -> Self {
        let mut list = Self::new();
        for value in values {
            list.insert(value);
            list.len += 1;
        }
        list
    }

    fn pop(&mut self) -> Result<Option<T>> {
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

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn get(&self, index: usize) -> Result<Option<&T>> {
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
        assert_eq!(list.is_empty(), true);
        assert!(list.remove(99).is_err());
        list.insert(1);
        list.insert(2);
        list.insert(3);
        list.insert(4);
        list.insert(5);
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.search(3).unwrap(), true);
        assert_eq!(list.search(6).unwrap(), false);
        assert_eq!(list.update(3, 6).unwrap(), true);
        assert_eq!(list.update(7, 8).is_err(), true);
        let list2 = Singly::from_vec(vec!["hello", "world", "rust"]);
        assert_eq!(list2.is_empty(), false);
        assert_eq!(list.remove(4).unwrap(), true);
        assert_eq!(list.remove(7).is_err(), true);
        assert_eq!(list.pop().unwrap().unwrap(), 5);
        assert_eq!(list.pop().unwrap().unwrap(), 6);
        assert_eq!(list.pop().unwrap().unwrap(), 2);
        assert_eq!(list.pop().unwrap().unwrap(), 1);
        assert_eq!(list.pop().is_err(), true);
        assert_eq!(list2.get(2).unwrap(), Some(&"rust"));
        assert_eq!(list.is_empty(), true);
    }
}

// endregion: --- Tests
