use std::collections::HashMap;

// The `KVStore` stores string key/value pairs.
//
// Example:
//
//```rust
//  let mut kvs = KVStore::new();
//  kvs.set("key", "value");
//  let value = kvs.get("key");
//  assert_eq!(value, Some("value"));
//```
pub struct KVStore {
    data: HashMap<String, String>
}

impl KVStore {
    // Creates a `KVStore`.
    pub fn new() -> KVStore {
        KVStore {
            data: HashMap::new(),
        }
    }

    // Sets the value of a string key to a string.
    //
    // If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    // Gets the value of a given string key.
    //
    // Returns `None` if the given key does not exist.
    pub fn get(&mut self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    // Removes a given key.
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}



