[![RustBuild](https://github.com/SarthakMakhija/learning-rust/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/SarthakMakhija/learning-rust/actions/workflows/build.yml)

# learning-rust
Repository to learn rust before implementing pluggable replication mechanism for distributed systems

# contents
- basic example using struct and its impl
- an example of lifetime with struct
- an example of closure with struct
- sequential sum of a huge vector
- concurrent (or parallel depending on the number of core available) sum of a huge vector
- async sum of a huge vector (using tokio)
- singular update queue
  - based on tokio task
  - based on rust thread
- append-only linked list
- grpc using tonic and tokio
- naive implementation of an in-memory cache with eviction in the background
  - Need to understand the following: 
    - can this be simplified `type ShardedLockedStorage = Arc<Vec<RwLock<HashMap<String, Arc<ValueRef>>>>>`
    - testing `thread::spawn` code in rust