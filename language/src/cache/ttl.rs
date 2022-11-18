use std::mem::transmute;
use std::thread;
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

    pub fn has_expired(&self) -> bool {
        return match self.expiry_after.instant {
            None => { false }
            Some(expire_after) => {
                let now = Instant::now();
                expire_after.saturating_duration_since(now) == Duration::from_nanos(0)
            }
        };
    }
}

pub struct Expiry {
    instant: Option<Instant>,
}

impl Expiry {
    pub fn never() -> Expiry {
        return Expiry { instant: None };
    }

    pub fn after_seconds(time: u64) -> Expiry {
        return Expiry { instant: Some(Instant::now() + Duration::from_secs(time)) };
    }
}

#[test]
fn test_a_non_expiry_with_a_never_expiring_value() {
    let value_ref = ValueRef::new(String::from("some value"), Expiry::never());
    let has_expired = value_ref.has_expired();

    assert_eq!(false, has_expired);
}

#[test]
fn test_a_non_expiry_with_an_expiring_value() {
    let value_ref = ValueRef::new(String::from("some value"), Expiry::after_seconds(10));
    let has_expired = value_ref.has_expired();

    assert_eq!(false, has_expired);
}

#[test]
fn test_an_expiry() {
    let value_ref = ValueRef::new(String::from("some value"), Expiry::after_seconds(1));
    thread::sleep(Duration::from_secs(2));

    let has_expired = value_ref.has_expired();
    assert_eq!(true, has_expired);
}