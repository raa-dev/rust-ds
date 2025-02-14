# Linked List
A linked list is a linear data structure where each element is a separate object. Each element (we will call it a node) of a list is consisting of two items - the data and a reference to other nodes. The last node has a reference to `None`. The entry point into a linked list is called the head of the list. It should be noted that the head is not a separate node, but the reference to the first node. If the list is empty then the head is a `None` reference.

## Operations
The following operations are defined for a linked list:
- **Append**: Adds an element at the end of the list.
- **Pop**: Removes the last element from the list.
- **Remove**: Removes an element from the list.
- **Search**: Searches for an element in the list.
- **Update**: Updates an element in the list.
- **Get**: Returns the element at a given index.

## Usage 
### Singly Linked List Example

```rust
use linked_lists::Singly;

fn main() {
    let mut list = Singly::new();
    list.append(1);
    list.append(2);
    list.append(3);
    println!("List after appending elements: {:?}", list);

    list.pop();
    println!("List after popping an element: {:?}", list);

    list.remove(1);
    println!("List after removing an element: {:?}", list);

    let found = list.search(2);
    println!("Element 2 found: {}", found);

    list.update(0, 4);
    println!("List after updating an element: {:?}", list);

    let element = list.get(0);
    println!("Element at index 0: {:?}", element);
}
```

### Doubly Linked List Example

```rust
use linked_lists::Double;

fn main() {
    let mut list = Double::new();
    list.append(1);
    list.append(2);
    list.append(3);
    println!("List after appending elements: {:?}", list);

    list.pop();
    println!("List after popping an element: {:?}", list);

    list.remove(1);
    println!("List after removing an element: {:?}", list);

    let found = list.search(2);
    println!("Element 2 found: {}", found);

    list.update(0, 4);
    println!("List after updating an element: {:?}", list);

    let element = list.get(0);
    println!("Element at index 0: {:?}", element);
}
```
