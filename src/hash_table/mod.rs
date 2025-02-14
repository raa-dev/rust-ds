mod errors;

pub(super) use errors::{Error, Result};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

type KeyPointer<K, V> = Option<(K, V)>;

#[derive(Debug)]
pub struct Table<K, V>
where
    K: Clone,
    V: Clone,
{
    pub elements: Vec<KeyPointer<K, V>>,
    capacity: usize,
}

impl<K, V> Table<K, V>
where
    K: Hash + Eq + Debug + Clone,
    V: Debug + Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            elements: vec![None; capacity],
            capacity,
        }
    }

    fn hash<Q>(&self, key: &Q) -> usize
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        self.elements[index] = Some((key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);
        self.elements[index].as_ref().map(|(_, v)| v)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash(key);
        if let Some((_, value)) = self.elements[index].take() {
            Some(value)
        } else {
            None
        }
    }

    pub fn update(&mut self, key: &K) -> Option<&mut V> {
        let index = self.hash(key);
        self.elements[index].as_mut().map(|(_, v)| v)
    }

    pub fn resize(&mut self, new_capacity: usize) {
        let mut new_table = Table::new(new_capacity);
        for element in self.elements.iter().filter_map(|e| e.as_ref()) {
            new_table.insert(element.0.clone(), element.1.clone());
        }
        self.elements = new_table.elements;
        self.capacity = new_capacity;
    }
}

impl<K, V> Default for Table<K, V>
where
    K: Hash + Eq + Debug + Clone,
    V: Debug + Clone,
{
    fn default() -> Self {
        Self::new(64)
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table_ops() {
        let mut table = Table::new(16);
        table.insert("key1", "value1");
        assert_eq!(table.get(&"key1"), Some(&"value1"));
        table.remove(&"key1");
        assert_eq!(table.get(&"key1"), None);
        table.insert("key2", "value2");
        if let Some(v) = table.update(&"key2") {
            *v = "value3";
        }
        assert_eq!(table.get(&"key2"), Some(&"value3"));
    }
}
// endregion: --- Tests
