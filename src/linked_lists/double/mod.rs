// use super::ExtNode as Node;
// use super::{Error, Result};
// use crate::linked_lists::node::ExtNode;
// use std::cell::RefCell;
// use std::clone::Clone;
// use std::fmt::Debug;
// use std::rc::Rc;

// /// A double linked list referencing the head node.
// pub struct Double<T: Clone> {
//     head: Option<Rc<RefCell<ExtNode<T>>>>,
//     tail: Option<Rc<RefCell<ExtNode<T>>>>,
//     len: usize,
// }

// impl<T> Double<T>
// where
//     T: Debug + PartialEq + Clone,
// {
//     fn new() -> Self {
//         Double {
//             head: None,
//             tail: None,
//             len: 0,
//         }
//     }
//     fn insert(&mut self, value: T) {
//         let new_node = Rc::new(RefCell::new(Node::new(value)));

//         match &mut self.head {
//             None => {
//                 self.head = Some(new_node);
//             }
//             Some(current) => {
//                 let mut current = current;
//                 while let Some(ref mut next) = current.get_mut().get_next() {
//                     current = next;
//                 }
//                 current.get_mut().set_next(Some(new_node));
//                 current
//                     .borrow_mut()
//                     .set_previous(Some(Rc::downgrade(current)));
//             }
//         }
//     }
//     fn remove(&mut self, value: T) -> Result<bool, Error<T>> {
//         if self.is_empty() {
//             return Err(Error::EmptyList);
//         }

//         if *self.head.as_ref().unwrap().get_mut().get_value() == value {
//             self.head = self.head.take().unwrap().get_mut().get_next();
//             return Ok(true);
//         }
//         let mut current = &mut self.head;
//         while let Some(node) = current {
//             if let Some(mut next) = &node.get_mut().get_next() {
//                 if *next.get_mut().get_value() == value {
//                     let next_node = node.get_next().take().unwrap();
//                     node.get_mut().set_next(next_node.get_next());
//                     return Ok(true);
//                 }
//             }
//             current = &mut node.get_mut().get_next();
//         }

//         Ok(false)
//     }
//     fn search(&self, value: T) -> Result<bool, Error<T>> {
//         let mut current = &self.head;
//         while let Some(mut node) = current {
//             if *node.get_mut().get_value() == value {
//                 return Ok(true);
//             }
//             current = &node.get_mut().get_next();
//         }
//         Ok(false)
//     }
//     fn update(&mut self, old_value: T, new_value: T) -> Result<bool, Error<T>> {
//         let mut current = &mut self.head;
//         while let Some(mut node) = current {
//             if *node.get_mut().get_value() == old_value {
//                 node.get_mut().set_value(new_value);
//                 return Ok(true);
//             }
//             current = &mut node.get_mut().get_next();
//         }
//         Err(Error::ValueNotFound { value: old_value })
//     }

//     fn from_vec(values: Vec<T>) -> Self {
//         let mut list = Self::new();
//         for value in values {
//             list.insert(value);
//         }
//         list
//     }

//     fn pop(&mut self) -> Result<Option<T>, Error<T>> {
//         if self.is_empty() {
//             return Err(Error::EmptyList);
//         }
//         if self.head.as_ref().unwrap().get_mut().get_next().is_none() {
//             return Ok(Some(self.head.take().unwrap().get_mut().get_value()));
//         }

//         let mut current = &mut self.head;
//         while let Some(node) = current {
//             if let Some(mut next) = &node.get_mut().get_next() {
//                 if next.get_mut().get_next().is_none() {
//                     let mut last_node = node.get_mut().get_next().take().unwrap();
//                     node.get_mut().set_next(None);
//                     node.get_mut()
//                         .get_previous_mut()
//                         .unwrap()
//                         .upgrade()
//                         .unwrap()
//                         .get_mut()
//                         .set_next(None);
//                     return Some(last_node.get_mut().get_value());
//                 }
//             }
//             current = &mut node.get_mut().get_next();
//         }
//         Ok(None)
//     }
//     fn print(&self) {
//         let mut current = &self.head;
//         while let Some(mut node) = current {
//             println!("{:?}", node.get_mut().get_value());
//             current = &node.get_mut().get_next();
//         }
//     }
//     fn is_empty(&self) -> bool {
//         self.head.is_none()
//     }

//     fn get(&self, index: usize) -> Result<Option<&T>, Error<T>> {
//         if self.is_empty() {
//             return Err(Error::EmptyList);
//         }
//         let mut current = &self.head;
//         let mut i = 0;
//         while let Some(mut node) = current {
//             if i == index {
//                 return Ok(Some(&node.get_mut().get_value()));
//             }
//             i += 1;
//             current = &node.get_mut().get_next();
//         }
//         Err(Error::IndexOutOfBounds {
//             index,
//             max_index: i - 1,
//         })
//     }
// }
