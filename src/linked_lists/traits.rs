use std::fmt::Debug;
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