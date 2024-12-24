use std::fmt::Debug;

/*
/ A node in the linked list, containing a custom-type value and a pointer to the next node.
 */
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/*
/ A singly linked list referencing the head node.
 */
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    /*
    / Inserts a new node to the end of the linked list.
    / If the linked list is empty, the new node becomes the head
    / If the linked list is not empty, the new node is added to the end
     */
    pub fn insert(&mut self, value: T) {
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

    /*
   / Removes a node from the linked list by value.
    */
    pub fn remove(&mut self, value: &T) -> bool
    where T: PartialEq {
        if self.head.is_none() {
            return false;
        }

        if self.head.as_ref().unwrap().value == *value {
            self.head = self.head.take().unwrap().next;
            return true;
        }

        let mut current = &mut self.head;
        while let Some(node) = current {
            if let Some(next) = &node.next {
                if next.value == *value {
                    // Remove the next node by updating the link
                    let next_node = node.next.take().unwrap();
                    node.next = next_node.next;
                    return true;
                }
            }
            current = &mut node.next;
        }
        false
    }

    /*
    / Removes the last node from the linked list.
    */
    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
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

    /*
    / Create a linked list from a vector of values.
     */
    pub fn from_vec(values: Vec<T>) -> Self {
        let mut list = LinkedList::new();
        for value in values {
            list.insert(value);
        }
        list
    }

    /* Search for a value in the list
    / Returns true if the value is found
     */
    pub fn search(&self, value: &T) -> bool
    where T: PartialEq {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == *value {
                return true;
            }
            current = &node.next;
        }
        false
    }
    /*
    / Update a value in the list
     */
    pub fn update(&mut self, value: T, new_value: T) -> bool
    where T: PartialEq {
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.value == value {
                node.value = new_value;
                return true;
            }
            current = &mut node.next;
        }
        false
    }

    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{:?} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}