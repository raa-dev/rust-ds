# Hash Table

A Hash Table orders the data as pairs of keys and values. This enables 
efficient data retrieval based on the key. Each key is hashed to produce 
an index at which the corresponding value is stored. This allows for 
average-case constant time complexity, O(1), for both insertion and 
lookup operations. 

## Operations
- **Insert**: Add a key-value pair to the hash table.
- **Remove**: Remove a key-value pair from the hash table.
- **Get**: Retrieve the value associated with a given key.
- **Update**: Modify the value associated with a given key.
- **Resize**: Adjust the size of the hash table to maintain efficient operations.

## Usage

```rust
use rust_ds::hash_table::Table;

fn main() {
    let mut table = Table::new(4); // You can use default instead to use a 64 length capacity

    // Insert key-value pairs
    table.insert("key1", "value1");
    table.insert("key2", "value2");

    // Retrieve a value
    if let Some(value) = table.get("key1") {
        println!("The value for 'key1' is {}", value);
    }

    // Update a value
    table.update("key1", "new_value1");

    // Remove a key-value pair
    table.remove("key2");

    // Resize the hash table
    table.resize(32);
}
```