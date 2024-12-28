//! The tests here are all for shared memory spsc queues sending messages between threads. I don't
//! care about process private memory, so I'm not going to test that

use message::IpcMessage;
use std::thread;

mod message;

fn main() {
    // Preallocate the messages
    let small_messages = IpcMessage::create_dummy_messages(10000);
    let medium_messages = IpcMessage::create_dummy_messages(100000);
}

fn test_temporal_que() {
    todo!()
}

fn test_omango() {
    todo!()
}

fn test_rtrb() {
    todo!()
}
