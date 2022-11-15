use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

type Storage = Arc<RwLock<HashMap<String, String>>>;

#[derive(Clone)]
struct SingularUpdateQueue {
    sender: Sender<Command>,
}

#[derive(Debug)]
enum Command {
    Put {
        key: String,
        value: String,
        respond_back: Sender<Status>,
    },
    Delete {
        key: String,
        respond_back: Sender<Status>,
    },
}

#[derive(Debug, Eq, PartialEq)]
enum Status {
    Ok
}

impl SingularUpdateQueue {
    fn init(storage: Storage) -> SingularUpdateQueue {
        return SingularUpdateQueue::spin_receiver(storage);
    }

    //loop pending
    fn spin_receiver(mut storage: Storage) -> SingularUpdateQueue {
        let (sender, receiver): (Sender<Command>, Receiver<Command>) = mpsc::channel();
        let mut singular_update_queue = SingularUpdateQueue { sender };
        let mut cloned = storage.clone();

        thread::spawn(move || {
            for (_, command) in receiver.into_iter().enumerate() {
                match command {
                    Command::Put { key, value, respond_back } => {
                        cloned.write().unwrap().insert(key, value);
                        respond_back.clone().send(Status::Ok).unwrap();
                    }
                    Command::Delete { key, respond_back } => {
                        cloned.write().unwrap().remove(&key);
                        respond_back.clone().send(Status::Ok).unwrap();
                    }
                }
            }
        });
        return singular_update_queue;
    }

    fn execute(&self, command: Command) {
        return self.sender.clone().send(command).unwrap();
    }
}

#[test]
fn test_get_with_insert_by_a_single_task() {
    let mut storage = Arc::new(RwLock::new(HashMap::new()));
    let cloned_storage = storage.clone();
    let singular_update_queue= SingularUpdateQueue::init(storage.clone());
    let cloned_queue = singular_update_queue.clone();

    let (sender, receiver) = mpsc::channel();
    let respond_back = sender.clone();

    let handle = thread::spawn(move || {
        cloned_queue.execute(Command::Put {
            key: String::from("key1"),
            value: String::from("value1"),
            respond_back,
        });
        assert_eq!(Status::Ok, receiver.recv().unwrap());
    });

    let _ = handle.join();
    let read_storage = cloned_storage.read().unwrap();
    assert_eq!("value1", read_storage.get("key1").unwrap());
}

#[test]
fn test_get_with_insert_by_multiple_tasks() {
    let mut storage = Arc::new(RwLock::new(HashMap::new()));
    let cloned_storage = storage.clone();
    let singular_update_queue = SingularUpdateQueue::init(storage.clone());
    let cloned_queue_one = singular_update_queue.clone();
    let cloned_queue_two = singular_update_queue.clone();

    let (sender, receiver) = mpsc::channel();
    let respond_back = sender.clone();

    let handle_one = thread::spawn( move || {
        cloned_queue_one.execute(Command::Put {
            key: String::from("key1"),
            value: String::from("value1"),
            respond_back,
        });
        assert_eq!(Status::Ok, receiver.recv().unwrap());
    });

    let (sender, receiver) = mpsc::channel();
    let respond_back = sender.clone();

    let handle_two = thread::spawn( move || {
        cloned_queue_two.execute(Command::Put {
            key: String::from("key2"),
            value: String::from("value2"),
            respond_back,
        });
        assert_eq!(Status::Ok, receiver.recv().unwrap());
    });

    let _ = handle_one.join();
    let _ = handle_two.join();

    let read_storage = cloned_storage.read().unwrap();
    assert_eq!("value1", read_storage.get("key1").unwrap());
    assert_eq!("value2", read_storage.get("key2").unwrap());
}

#[test]
fn test_get_with_insert_and_delete_by_multiple_tasks() {
    let mut storage = Arc::new(RwLock::new(HashMap::new()));
    let cloned_storage = storage.clone();
    let singular_update_queue = SingularUpdateQueue::init(storage.clone());
    let cloned_queue_one = singular_update_queue.clone();
    let cloned_queue_two = singular_update_queue.clone();

    let (sender, receiver) = mpsc::channel();
    let respond_back = sender.clone();

    let handle_one = thread::spawn( move || {
        cloned_queue_one.execute(Command::Put {
            key: String::from("key1"),
            value: String::from("value1"),
            respond_back,
        });
        assert_eq!(Status::Ok, receiver.recv().unwrap());
    });

    thread::sleep(Duration::from_millis(30));

    let (sender, receiver) = mpsc::channel();
    let respond_back = sender.clone();

    let handle_two = thread::spawn( move || {
        cloned_queue_two.execute(Command::Delete {
            key: String::from("key1"),
            respond_back,
        });
        assert_eq!(Status::Ok, receiver.recv().unwrap());
    });

    let _ = handle_one.join();
    let _ = handle_two.join();

    let read_storage = cloned_storage.read().unwrap();
    assert_eq!(None, read_storage.get("key1"));
}