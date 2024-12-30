use std::rc::{Rc, Weak};
use std::fmt::Debug;
use std::cell::RefCell;
use super::types::{Result,Error};

pub trait List<T>
where
    T: PartialEq + Debug,
{
    /// Creates a new empty list
    fn new() -> Self;

    /// Inserts a value at the end of the list
    fn insert(&mut self, value: T);

    /// Removes the first occurrence of a value from the list
    /// 
    /// # Returns
    /// - `Ok(true)` if the value was found and removed
    /// - `Ok(false)` if the list is empty
    /// - `Err(ListError::ValueNotFound)` if the value wasn't found
    fn remove(&mut self, value: &T) -> Result<bool,Error<T>>;

    /// Searches for a value in the list
    /// 
    /// # Returns
    /// - `Ok(true)` if the value was found
    /// - `Err(ListError::ValueNotFound)` if the value wasn't found
    fn search(&self, value: &T) -> Result<bool,Error<T>>;

    /// Updates the first occurrence of a value with a new value
    /// 
    /// # Returns
    /// - `Ok(true)` if the value was found and updated
    /// - `Err(ListError::ValueNotFound)` if the value wasn't found
    fn update(&mut self, old_value: T, new_value: T) -> Result<bool,Error<T>>;

    /// Creates a new list from a vector of values
    fn from_vec(values: Vec<T>) -> Self;

    /// Removes and returns the last element from the list
    /// 
    /// # Returns
    /// - `Some(value)` if the list is not empty
    /// - `None` if the list is empty
    fn pop(&mut self) -> Option<T>;

    /// Prints the list contents for debugging purposes
    fn print(&self);

    /// Checks if the list is empty
    fn is_empty(&self) -> bool;

    /// Returns the length of the list
    fn len(&self) -> usize;

    /// Get the value of a node at a given index
    fn get(&self, index: usize) -> Result<Option<&T>, Error<T>>;
}

pub trait SinglyNode<T>
where
    T: PartialEq + Debug,
{
    /// Creates a new node with a value
    fn new(value: T) -> Self;

    /// Returns the value of the node
    fn get_value(&self) -> &T;

    /// Returns the next node
    fn get_next(&self) -> Option<&Self>;

    /// Returns a mutable reference to the next node
    fn get_next_mut(&mut self) -> Option<&mut Self>;
    
    /// Sets the value of the node
    fn set_value(&mut self, value: T);

    /// Sets the next node
    fn set_next(&mut self, next: Option<Box<Self>>);
}

pub trait ExtendedNode<T>
where
    T: PartialEq + Debug,
{
    /// Creates a new node with a value
    fn new(value: T) -> Self;

    /// Returns the value of the node
    fn get_value(&self) -> &T;

    /// Sets the value of the node
    fn set_value(&mut self, value: T);

    /// Returns the next node
    fn get_next(&self) -> Option<&Rc<RefCell<Self>>>;

    /// Returns the value of the previous node
    fn get_previous(&self) -> Option<&Weak<RefCell<Self>>>;

    /// Returns a mutable reference to the next node
    fn get_next_mut(&mut self) -> Option<&mut Rc<RefCell<Self>>>;

    /// Returns a mutable reference to the previous node
    fn get_previous_mut(&mut self) -> Option<&mut Weak<RefCell<Self>>>;

    /// Sets the next node
    fn set_next(&mut self, next: Option<Rc<RefCell<Self>>>);
    
    /// Sets the previous node
    fn set_previous(&mut self, previous: Option<Weak<RefCell<Self>>>);
}