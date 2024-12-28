/// This is a simple IPC message struct that we will use to demonstrate the IPC communication
#[derive(Debug, Clone, Copy)]
pub struct IpcMessage {
    pub header: u8,
    pub data: [u8; 255],
}

impl IpcMessage {
    const SIZE_PER_MESSAGE: usize = std::mem::size_of::<IpcMessage>();

    pub fn new(header: u8, data: [u8; 255]) -> Self {
        IpcMessage { header, data }
    }

    pub fn create_dummy_messages(vec_length: usize) -> Vec<IpcMessage> {
        let vec_size = vec_length * IpcMessage::SIZE_PER_MESSAGE;
        let mut messages = Vec::with_capacity(vec_size);

        for i in 0..vec_length {
            messages.push(IpcMessage::new(i as u8, [0; 255]));
        }
        messages
    }
}
