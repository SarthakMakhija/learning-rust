use std::time::Duration;

use tokio::time::Instant;

pub struct ValueRef {
    value: String,
    expiry_after: Expiry,
}

impl ValueRef {
    pub fn new(value: String, expiry: Expiry) -> ValueRef {
        return ValueRef { value, expiry_after: expiry };
    }

    pub fn value(&self) -> &String {
        return &self.value;
    }
}

pub struct Expiry {
    duration: Option<Instant>,
}

impl Expiry {
    pub fn never() -> Expiry {
        return Expiry { duration: None };
    }

    pub fn after_seconds(time: u64) -> Expiry {
        return Expiry { duration: Some(Instant::now() + Duration::from_secs(time)) };
    }
}