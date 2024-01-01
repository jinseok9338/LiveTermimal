use core::fmt::Error;
use rand::Rng;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::web_fs::errors::ApiError;
const ROOT_NODE_ID: &str = "/";

type Buffer = Vec<u8>;

lazy_static::lazy_static! {
    static ref EMPTY_DIR_NODE: Arc<Mutex<Option<Buffer>>> = Arc::new(Mutex::new(None));
}

fn get_empty_dir_node() -> Buffer {
    // Lock the mutex to access or modify the shared state
    let mut empty_dir_node = EMPTY_DIR_NODE.lock().unwrap();

    // Check if the empty directory node has been created
    if let Some(ref buffer) = *empty_dir_node {
        buffer.clone()
    } else {
        // If not created, initialize it with an empty JSON object
        let new_empty_dir_node = Buffer::from(b"{}");
        *empty_dir_node = Some(new_empty_dir_node.clone());
        new_empty_dir_node
    }
}

/// Generates a random ID.
///
/// # Returns
///
/// A `String` representing the randomly generated ID.
///
/// # Example
///
/// ```rust
/// // Example usage of generate_random_id function.
/// let random_id = generate_random_id();
/// println!("Random ID: {}", random_id);
/// ```
fn generate_random_id() -> String {
    const CHARS: [char; 36] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut rng = rand::thread_rng();
    let mut result = String::with_capacity(36);

    for c in "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".chars() {
        match c {
            'x' => result.push(CHARS[rng.gen_range(0..16)]),
            'y' => result.push(CHARS[(rng.gen_range(0..4) | 0x8) as usize]),
            _ => result.push(c),
        }
    }

    result
}

// SyncKeyValueStore trait definition
pub trait SyncKeyValueStore {
    /// Returns the name of the key-value store.
    fn name(&self) -> String;

    /// Empties the key-value store completely.
    fn clear(&self);

    /// Begins a new read-only transaction.
    fn begin_transaction_ro(&self) -> Box<dyn SyncKeyValueROTransaction>;

    /// Begins a new read-write transaction.
    fn begin_transaction_rw(&self) -> Box<dyn SyncKeyValueRWTransaction>;

    /// Begins a new transaction based on the specified type.
    fn begin_transaction(&self, transaction_type: &str) -> Box<dyn SyncKeyValueROTransaction>;
}

pub trait SyncKeyValueROTransaction {
    /// Retrieves the data at the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look under for data.
    ///
    /// # Returns
    ///
    /// The data stored under the key, or `None` if not present.
    fn get(&self, key: &str) -> Option<Buffer>;
}

// SyncKeyValueRWTransaction trait definition
pub trait SyncKeyValueRWTransaction {
    /// Adds the data to the store under the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to add the data under.
    /// * `data` - The data to add to the store.
    /// * `overwrite` - If `true`, overwrite any existing data. If `false`,
    ///   avoids storing the data if the key exists.
    ///
    /// # Returns
    ///
    /// `true` if storage succeeded, `false` otherwise.
    fn get(&mut self, key: &str) -> Option<Buffer>;
    fn put(&mut self, key: &str, data: Buffer, overwrite: bool) -> Result<bool, ApiError>;

    /// Deletes the data at the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to delete from the store.
    fn del(&mut self, key: &str) -> Result<(), ApiError>;
    fn commit(&mut self) -> Result<(), ApiError>;
    fn abort(&mut self) -> Result<(), ApiError>;
}

pub trait SimpleSyncStore {
    /// Retrieves the data at the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look under for data.
    ///
    /// # Returns
    ///
    /// The data stored under the key, or `None` if not present.
    fn get(&self, key: &str) -> Option<Buffer>;

    /// Adds the data to the store under the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to add the data under.
    /// * `data` - The data to add to the store.
    /// * `overwrite` - If `true`, overwrite any existing data. If `false`,
    ///   avoids storing the data if the key exists.
    ///
    /// # Returns
    ///
    /// `true` if storage succeeded, `false` otherwise.
    fn put(&mut self, key: &str, data: Buffer, overwrite: bool) -> bool;

    /// Deletes the data at the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to delete from the store.
    fn del(&mut self, key: &str);
}

// class LRUNode {
// 	public prev: LRUNode | null = null;
// 	public next: LRUNode | null = null;
// 	constructor(public key: string, public value: string) {}
// }

#[derive(Debug, PartialEq, Clone)]
struct LRUNode {
    pub prev: Option<Box<LRUNode>>,
    pub next: Option<Box<LRUNode>>,
    pub key: String,
    pub value: String,
}

impl LRUNode {
    fn new(key: &str, value: &str) -> LRUNode {
        LRUNode {
            prev: None,
            next: None,
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

pub struct LRUCache {
    size: usize,
    map: HashMap<String, Box<LRUNode>>,
    head: Option<Box<LRUNode>>,
    tail: Option<Box<LRUNode>>,
    limit: usize,
}

impl LRUCache {
    pub fn new(limit: usize) -> LRUCache {
        LRUCache {
            size: 0,
            map: HashMap::new(),
            head: None,
            tail: None,
            limit,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let node = Box::new(LRUNode::new(key, value));
        if let Some(existing_node) = self.map.get_mut(key) {
            existing_node.value = node.value.clone();
            self.remove(key);
        } else if self.size >= self.limit {
            if let Some(tail) = self.tail.take() {
                self.map.remove(&tail.key);
                self.size -= 1;
                self.tail = tail.prev;
                if let Some(ref mut tail) = self.tail {
                    tail.next = None;
                }
            }
        }
        self.set_head(node);
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        if let Some(node) = self.map.remove(key) {
            let value = node.value.clone();
            self.remove(&key);
            self.set_head(node);
            Some(value)
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: &str) {
        if let Some(mut node) = self.map.remove(key) {
            if let Some(mut prev) = node.prev.take() {
                if let Some(mut next) = node.next.take() {
                    // Reconstruct the structure
                    let prev_value = prev.clone();
                    let next_value = next.clone();
                    prev.next = Some(next_value);
                    next.prev = Some(prev_value);
                } else {
                    // If there is no next node, update the tail
                    let prev = Box::new(*prev);
                    self.tail = Some(prev);
                }
            } else {
                // If there is no previous node, update the head
                self.head = node.next.take();
                if let Some(ref mut head) = self.head {
                    head.prev = None;
                }
            }
            self.size -= 1;
        }
    }

    pub fn remove_all(&mut self) {
        self.size = 0;
        self.map.clear();
        self.head = None;
        self.tail = None;
    }

    fn set_head(&mut self, mut node: Box<LRUNode>) {
        let node_key = node.clone().key.clone();
        node.next = self.head.take();
        node.prev = None;
        if let Some(ref mut head) = self.head {
            let node = node.clone();
            head.prev = Some(node);
        }
        let node_value_for_head = node.clone();
        self.head = Some(node_value_for_head);
        if self.tail.is_none() {
            let node = node.clone();
            self.tail = Some(node);
        }
        self.size += 1;
        self.map.insert(node_key, node);
    }
}

pub struct SimpleSyncRWTransaction {
    original_data: HashMap<String, Option<Buffer>>,
    modified_keys: Vec<String>,
    store: Box<dyn SimpleSyncStore>,
}

impl SimpleSyncRWTransaction {
    fn new(store: Box<dyn SimpleSyncStore>) -> Self {
        SimpleSyncRWTransaction {
            original_data: HashMap::new(),
            modified_keys: Vec::new(),
            store,
        }
    }

    // private _has(key: string) {
    // 	return Object.prototype.hasOwnProperty.call(this.originalData, key);
    // }

    fn has(&self, key: &str) -> bool {
        self.original_data.contains_key(key)
    }

    fn stash_old_value(&mut self, key: &str, value: Option<Buffer>) -> Result<(), ApiError> {
        if !self.has(key) {
            self.original_data.insert(key.to_string(), value);
            return Ok(());
        }
        let path = PathBuf::from("/");
        Err(ApiError::enoent(path))
    }
    fn mark_modified(&mut self, key: &str) -> Result<(), ApiError> {
        if !self.modified_keys.contains(&key.to_string()) {
            self.modified_keys.push(key.to_string());
            if !self.has(key) {
                if let Some(original_data) = self.store.get(key) {
                    self.original_data
                        .insert(key.to_string(), Some(original_data));
                } else {
                    let path = PathBuf::from("/");
                    return Err(ApiError::enoent(path));
                }
            }
        }
        Ok(())
    }
}

impl SyncKeyValueRWTransaction for SimpleSyncRWTransaction {
    fn get(&mut self, key: &str) -> Option<Buffer> {
        self.store.get(key).and_then(|val| {
            self.stash_old_value(key, Some(val.clone())).ok()?;
            Some(val)
        })
    }

    fn put(&mut self, key: &str, data: Buffer, overwrite: bool) -> Result<bool, ApiError> {
        self.mark_modified(key).ok();
        Ok(self.store.put(key, data, overwrite))
    }

    fn del(&mut self, key: &str) -> Result<(), ApiError> {
        self.mark_modified(key)?;
        Ok(self.store.del(key))
    }

    fn commit(&mut self) -> Result<(), ApiError> {
        Ok(())
    }

    fn abort(&mut self) -> Result<(), ApiError> {
        for key in &self.modified_keys {
            if let Some(value) = self.original_data.get(key) {
                match value {
                    Some(original_value) => {
                        // Key existed. Restore old value.
                        self.store.put(key, original_value.clone(), true);
                    }
                    None => {
                        // Key didn't exist originally. Delete it.
                        self.store.del(key);
                    }
                }
            }
        }
        Ok(())
    }
}
