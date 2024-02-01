pub struct Message {
    data: Vec<u8>,
}

impl Message {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
        }
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}