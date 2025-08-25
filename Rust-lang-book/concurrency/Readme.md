# Rust Concurrency Cheat Sheet

Threads allow running code simultaneously and can be waited on with `.join()`.  
Channels (`mpsc`) let threads safely send and receive data by transferring ownership.  
Shared data can be safely accessed with `Mutex` and shared ownership via `Arc`.  
`Send` allows types to move between threads safely, `Sync` allows safe shared references.
