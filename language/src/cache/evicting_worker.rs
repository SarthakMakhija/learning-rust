use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use crate::cache::evicting_cache::{BucketIndex, ShardedLockedStorage};
use crate::cache::expiry::{Expiry, ValueRef};

struct EvictingWorker {
    storage: ShardedLockedStorage,
    current_bucket: BucketIndex,
    buckets: usize,
}

impl EvictingWorker {
    const SLEEP_FOR_SECONDS: Duration = Duration::from_micros(5);

    fn run(buckets: usize, storage: ShardedLockedStorage) {
        Self::run_from(buckets, 0, storage);
    }

    fn run_from(buckets: usize, current_bucket: BucketIndex, storage: ShardedLockedStorage) {
        let mut worker = EvictingWorker { storage, current_bucket, buckets };
        thread::spawn(move || {
            loop {
                worker.evict();
                worker.current_bucket = (worker.current_bucket + 1) % worker.buckets;
                thread::sleep(EvictingWorker::SLEEP_FOR_SECONDS);
            }
        });
    }

    fn evict(&self) {
        let mut locked_storage = self.storage[self.current_bucket].write().unwrap();
        locked_storage.retain(|_, value_ref| {
            !value_ref.has_expired()
        });
    }
}

#[test]
fn test_eviction() {
    let key_value_pairs = HashMap::from(
        [
            (String::from("expired"),
             Arc::new(ValueRef::new(String::from("expired_value"), Expiry::immediate()))
            ),
            (String::from("living"),
             Arc::new(ValueRef::new(String::from("living_value"), Expiry::never()))
            )
        ],
    );
    let storage: ShardedLockedStorage = Arc::new(vec![RwLock::new(key_value_pairs)]);
    EvictingWorker::run(1, storage.clone());

    thread::sleep(Duration::from_secs(5));

    let read_map = storage[0].read().unwrap();
    let expired_value = read_map.get("expired");
    let living_value = read_map.get("living");

    assert_eq!(1, read_map.len());
    assert_eq!(true, expired_value.is_none());
    assert_eq!(&String::from("living_value"), living_value.unwrap().value());
}