use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::error::SendError;

type Storage = Arc<RwLock<HashMap<String, String>>>;

struct SingularUpdateQueue {
    sender: Sender<Command>,
}

#[derive(Debug)]
enum Command {
    Put {
        key: String,
        value: String,
        respond_back: oneshot::Sender<Status>,
    },
    Delete {
        key: String,
        respond_back: oneshot::Sender<Status>,
    },
}

#[derive(Debug, Eq, PartialEq)]
enum Status {
    Ok
}

impl SingularUpdateQueue {
    async fn init(handler: Storage) -> SingularUpdateQueue {
        return SingularUpdateQueue::spin_receiver(handler);
    }

    //loop pending
    fn spin_receiver(mut handler: Storage) -> SingularUpdateQueue {
        let (sender, mut receiver): (Sender<Command>, Receiver<Command>) = mpsc::channel(1);
        let mut singular_update_queue = SingularUpdateQueue { sender };
        let mut cloned = handler.clone();

        tokio::spawn(async move {
            while let Some(command) = receiver.recv().await {
                match command {
                    Command::Put { key, value, respond_back } => {
                        cloned.write().unwrap().insert(key, value);
                        respond_back.send(Status::Ok).unwrap();
                    }
                    Command::Delete { key, respond_back } => {
                        cloned.write().unwrap().remove(&key);
                        respond_back.send(Status::Ok).unwrap();
                    }
                }
            }
        });
        return singular_update_queue;
    }

    async fn execute(&self, command: Command) {
        return self.sender.send(command).await.unwrap();
    }
}

#[tokio::test]
async fn test_get_with_insert_by_a_single_task() {
    let mut storage = Arc::new(RwLock::new(HashMap::new()));
    let cloned_storage = storage.clone();
    let singular_update_queue = Arc::new(SingularUpdateQueue::init(storage.clone()).await);
    let cloned_queue = singular_update_queue.clone();

    let handle = tokio::spawn(async move {
        let (sender, receiver) = oneshot::channel();
        let execution = cloned_queue.execute(Command::Put {
            key: String::from("key1"),
            value: String::from("value1"),
            respond_back: sender,
        });
        execution.await;
        assert_eq!(Status::Ok, receiver.await.unwrap());
    });

    let _ = handle.await;
    let read_storage = cloned_storage.read().unwrap();
    assert_eq!("value1", read_storage.get("key1").unwrap());
}

#[tokio::test]
async fn test_get_with_insert_by_multiple_tasks() {
    let mut storage = Arc::new(RwLock::new(HashMap::new()));
    let cloned_storage = storage.clone();
    let singular_update_queue = Arc::new(SingularUpdateQueue::init(storage.clone()).await);
    let cloned_queue_one = singular_update_queue.clone();
    let cloned_queue_two = singular_update_queue.clone();

    let handle_one = tokio::spawn(async move {
        let (sender, receiver) = oneshot::channel();
        let execution = cloned_queue_one.execute(Command::Put {
            key: String::from("key1"),
            value: String::from("value1"),
            respond_back: sender,
        });
        execution.await;
        assert_eq!(Status::Ok, receiver.await.unwrap());
    });
    let handle_two = tokio::spawn(async move {
        let (sender, receiver) = oneshot::channel();
        let execution = cloned_queue_two.execute(Command::Put {
            key: String::from("key2"),
            value: String::from("value2"),
            respond_back: sender,
        });
        execution.await;
        assert_eq!(Status::Ok, receiver.await.unwrap());
    });

    let _ = tokio::join!(handle_one, handle_two);
    let read_storage = cloned_storage.read().unwrap();
    assert_eq!("value1", read_storage.get("key1").unwrap());
    assert_eq!("value2", read_storage.get("key2").unwrap());
}

#[tokio::test]
async fn test_get_with_insert_and_delete_by_multiple_tasks() {
    let mut storage = Arc::new(RwLock::new(HashMap::new()));
    let cloned_storage = storage.clone();
    let singular_update_queue = Arc::new(SingularUpdateQueue::init(storage.clone()).await);
    let cloned_queue_one = singular_update_queue.clone();
    let cloned_queue_two = singular_update_queue.clone();

    let handle_one = tokio::spawn(async move {
        let (sender, receiver) = oneshot::channel();
        let execution = cloned_queue_one.execute(Command::Put {
            key: String::from("key1"),
            value: String::from("value1"),
            respond_back: sender,
        });
        execution.await;
        assert_eq!(Status::Ok, receiver.await.unwrap());
    });
    let handle_two = tokio::spawn(async move {
        let (sender, receiver) = oneshot::channel();
        let execution = cloned_queue_two.execute(Command::Delete {
            key: String::from("key1"),
            respond_back: sender,
        });
        execution.await;
        assert_eq!(Status::Ok, receiver.await.unwrap());
    });

    let _ = tokio::join!(handle_one, handle_two);
    let read_storage = cloned_storage.read().unwrap();
    assert_eq!(None, read_storage.get("key1"));
}