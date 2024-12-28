use message::IpcMessage;
use std::thread;

mod message;

fn main() {
    let messages = IpcMessage::create_dummy_messages(10000);

    let handle_1 = thread::spawn(move || {});
}
