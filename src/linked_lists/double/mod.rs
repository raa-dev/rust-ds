use std::clone::Clone;
use crate::linked_lists::traits::List;
use crate::linked_lists::types::{Result,Error};

/*
/ A node in the double linked list, containing a custom-type value and two pointers to the next and the previous node.
 */
use std::fmt::Debug;
#[derive(Debug, Clone)]
struct Node<T> where T: Clone {
    value: T,
    next: Option<Box<Node<T>>>,
    previous: Option<Box<Node<T>>>,
}

/*
/ A double linked list referencing the head node.
 */
pub struct Double<T: Clone> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug + PartialEq + Clone> List<T> for Double<T> {
    fn new() -> Self {
        Double { head: None }
    }
    /*
    / Inserts a new node to the end of the double linked list.
     */
     fn insert(&mut self, value: T) {
            let new_node = Box::new(Node {
                value,
                next: None,
                previous: None,
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
                    current.next.as_mut().unwrap().previous = Some(current.clone());
                }
            }
     }
     fn remove(&mut self, value: &T) -> Result<bool,Error<T>> {
        if self.is_empty() {
            return Err(Error::EmptyList);
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

        Ok(false)
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
            return self.head.take().map(|node| node.value);
         }

         let mut current = &mut self.head;
         while let Some(node) = current {
             if let Some(next) = &node.next {
                 if next.next.is_none() {
                     let last_node = node.next.take().unwrap();
                     node.next = None;
                     node.previous.as_mut().unwrap().next = None;
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
            println!("{:?}", node.value);
            current = &node.next;
        }
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
        let mut i = 0;
        while let Some(node) = current {
            if i == index {
                return Ok(Some(&node.value));
            }
            i += 1;
            current = &node.next;
        }
        Err(Error::IndexOutOfBounds { index, max_index: i - 1 })
    }
}