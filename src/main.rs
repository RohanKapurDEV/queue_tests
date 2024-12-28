//! The tests here are all for shared memory spsc queues sending messages between threads. I don't
//! care about process private memory, so I'm not going to test that.

use message::IpcMessage;
use std::thread;

mod message;

fn main() {
    let messages = IpcMessage::create_dummy_messages(10000);
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
