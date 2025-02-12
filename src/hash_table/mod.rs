use std::fmt::Debug;

type KeyPointer<T> = Option<T>;

#[derive(Debug)]
pub struct Table<T> {
    elements: Vec<KeyPointer<T>>,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table_ops() {
        // let table = Table::new();
    }
}
// endregion: --- Tests
