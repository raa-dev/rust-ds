use std::fmt::Debug;
use crate::linked_lists::traits::List;
use crate::linked_lists::types::{Result,Error};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Singly<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> for Singly<T>
where
    T: Debug + PartialEq + Clone,
{
    fn new() -> Self {
        Singly { head: None }
    }

    fn insert(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: None,
        });

        match &mut self.head {
            None => {
                self.head = Some(new_node);
            }
            Some(current) => {
                let mut current = current;
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node);
            }
        }
    }

    fn remove(&mut self, value: &T) -> Result<bool,Error<T>> {
        if self.is_empty() {
            return Ok(false);
        }

        if self.head.as_ref().unwrap().value == *value {
            self.head = self.head.take().unwrap().next;
            return Ok(true);
        }

        let mut current = &mut self.head;
        while let Some(node) = current {
            if let Some(next) = &node.next {
                if next.value == *value {
                    let next_node = node.next.take().unwrap();
                    node.next = next_node.next;
                    return Ok(true);
                }
            }
            current = &mut node.next;
        }
        Err(Error::ValueNotFound { value: value.clone() })
    }

    fn search(&self, value: &T) -> Result<bool,Error<T>> {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == *value {
                return Ok(true);
            }
            current = &node.next;
        }
        Ok(false)
    }

    fn update(&mut self, old_value: T, new_value: T) -> Result<bool,Error<T>> {
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.value == old_value {
                node.value = new_value;
                return Ok(true);
            }
            current = &mut node.next;
        }
        Err(Error::ValueNotFound { value: old_value })
    }

    fn from_vec(values: Vec<T>) -> Self {
        let mut list = Self::new();
        for value in values {
            list.insert(value);
        }
        list
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if self.head.as_ref().unwrap().next.is_none() {
            return Some(self.head.take().unwrap().value);
        }

        let mut current = &mut self.head;
        while let Some(node) = current {
            if let Some(next) = &node.next {
                if next.next.is_none() {
                    let last_node = node.next.take().unwrap();
                    return Some(last_node.value);
                }
            }
            current = &mut node.next;
        }
        None
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{:?} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn len(&self) -> usize {
        let mut len = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            len += 1;
            current = &node.next;
        }
        len
    }

    fn get(&self, index: usize) -> Result<Option<&T>, Error<T>> {
        if self.is_empty() {
            return Err(Error::EmptyList);
        }

        let mut current = &self.head;
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                return Ok(Some(&node.value));
            }
            current = &node.next;
            current_index += 1;
        }

        Err(Error::IndexOutOfBounds { index, max_index: current_index - 1 })
    }
}