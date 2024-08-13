use hashbrown::HashMap;

pub trait WriteDB {
    fn write(&self, key: Vec<u8>, value: Vec<u8>);
}

pub struct BatchedWriter {
    changes: Option<HashMap<Vec<u8>, Vec<u8>>>,
}

pub struct Writer {}

impl WriteDB for Writer {
    fn write(&self, key: Vec<u8>, value: Vec<u8>) {}
}

impl WriteDB for BatchedWriter {
    fn write(&self, key: Vec<u8>, value: Vec<u8>) {}
}

impl BatchedWriter {
    fn set_unroll_on_error(&mut self) {
        match self.changes {
            Some(_) => {}
            None => self.changes = Some(HashMap::new()),
        }
    }
}
