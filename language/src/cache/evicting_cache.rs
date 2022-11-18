use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use std::time::Duration;

use crate::cache::ttl::{Expiry, ValueRef};

struct EvictingCache {
    elements: Vec<RwLock<HashMap<String, Rc<ValueRef>>>>,
    buckets: usize,
}

impl EvictingCache {
    fn new(buckets: usize) -> EvictingCache {
        let mut elements = Vec::with_capacity(buckets);
        for _ in 0..buckets {
            elements.push(RwLock::new(HashMap::new()));
        }
        return EvictingCache { elements, buckets };
    }

    fn put(&mut self, key: String, value: String) {
        let key_index = self.index_of(&key);
        let mut value_by_key = self.elements[key_index].write().unwrap();
        value_by_key.insert(key, Rc::new(ValueRef::new(value, Expiry::never())));
    }

    fn get(&self, key: String) -> Option<Rc<ValueRef>> {
        let key_index = self.index_of(&key);
        let mut value_by_key = self.elements[key_index].read().unwrap();

        return match value_by_key.get(&key) {
            None => { None }
            Some(rc_value) => { Some(rc_value.clone()) }
        };
    }

    fn index_of(&self, key: &String) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let hash = hasher.finish() as usize;
        return hash % self.buckets;
    }
}

#[test]
fn test_get_value_by_an_existing_key() {
    let mut evicting_cache = EvictingCache::new(64);
    evicting_cache.put(String::from("disk_type"), String::from("SSD"));

    let value: Option<Rc<ValueRef>> = evicting_cache.get(String::from("disk_type"));
    assert_eq!(&String::from("SSD"), value.unwrap().value());
}

#[test]
fn test_get_value_by_an_non_existing_key() {
    let mut evicting_cache = EvictingCache::new(64);
    evicting_cache.put(String::from("disk_type"), String::from("SSD"));

    let value: Option<Rc<ValueRef>> = evicting_cache.get(String::from("non_existing"));
    assert!(value.is_none());
}