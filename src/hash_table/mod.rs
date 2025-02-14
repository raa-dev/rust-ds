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

    pub fn get(&self, key: &K) -> Result<&V> {
        let index = self.hash(key);
        self.elements[index]
            .as_ref()
            .map(|(_, v)| v)
            .ok_or(Error::KeyNotFound)
    }

    pub fn remove(&mut self, key: &K) -> Result<V> {
        if self.capacity == 0 {
            return Err(Error::EmptyTable);
        }
        let index = self.hash(key);
        if let Some((_, value)) = self.elements[index].take() {
            Ok(value)
        } else {
            Err(Error::KeyNotFound)
        }
    }

    pub fn update(&mut self, key: &K) -> Result<&mut V> {
        if self.capacity == 0 {
            return Err(Error::EmptyTable);
        }
        let index = self.hash(key);
        self.elements[index]
            .as_mut()
            .map(|(_, v)| v)
            .ok_or(Error::KeyNotFound)
    }

    pub fn resize(&mut self, new_capacity: usize) -> Result<()> {
        if new_capacity == 0 {
            return Err(Error::InvalidCapacity);
        }

        let mut new_table = Table::new(new_capacity);
        for element in self.elements.iter().filter_map(|e| e.as_ref()) {
            new_table.insert(element.0.clone(), element.1.clone());
        }
        self.elements = new_table.elements;
        self.capacity = new_capacity;
        Ok(())
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
        assert_eq!(table.get(&"key1").unwrap(), &"value1");
        table.remove(&"key1").unwrap();
        assert!(table.get(&"key1").is_err());
        table.insert("key2", "value2");
        if let Ok(v) = table.update(&"key2") {
            *v = "value3";
        }
        assert_eq!(table.get(&"key2").unwrap(), &"value3");
    }

    #[test]
    fn test_hash_table_errors() {
        let mut table: Table<&str, &str> = Table::new(16);
        assert!(table.get(&"key1").is_err());
        assert!(table.remove(&"key1").is_err());
        assert!(table.update(&"key1").is_err());
        assert!(table.resize(0).is_err());
    }
}
// endregion: --- Tests
