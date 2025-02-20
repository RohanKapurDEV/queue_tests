/// This is a simple struct that we will use for ipc
#[derive(Debug, Clone, Copy)]
pub struct IpcMessage {
    pub header: u8,
    pub data: [u8; 255],
}

impl IpcMessage {
    const SIZE_PER_MESSAGE: usize = size_of::<IpcMessage>();

    pub fn new(header: u8, data: [u8; 255]) -> Self {
        IpcMessage { header, data }
    }

    pub fn create_dummy_messages(vec_length: usize) -> Vec<IpcMessage> {
        let vec_size = vec_length * IpcMessage::SIZE_PER_MESSAGE;
        let mut messages = Vec::with_capacity(vec_size);

        for i in 0..vec_length {
            messages.push(IpcMessage::new(i as u8, [0; 255]));
        }

        println!("Current capacity: {}", messages.capacity());
        println!("Current length: {}", messages.len());

        assert_eq!(messages.capacity(), vec_size);
        println!("Vector capacity matches the expected size");

        messages
    }
}
