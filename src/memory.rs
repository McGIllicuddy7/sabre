#[derive(Clone, Copy, Debug)]
pub struct Memory {
    pub memory: [u8; 1_000_000],
}
impl Memory {
    pub fn new() -> Self {
        Self { memory: [0; _] }
    }
    pub fn memory_read(&self, address: u32) -> u32 {
        let add = address as usize;
        let values = [
            self.memory[add],
            self.memory[add + 1],
            self.memory[add + 2],
            self.memory[add + 3],
        ];
        u32::from_le_bytes(values)
    }

    pub fn memory_write(&mut self, address: u32, value: u32) {
        let add = address as usize;
        let v = u32::to_le_bytes(value);
        self.memory[add] = v[0];
        self.memory[add + 1] = v[1];
        self.memory[add + 2] = v[2];
        self.memory[add + 3] = v[3];
    }

    pub fn memory_read_word(&self, address: u32) -> u16 {
        let add = address as usize;
        let values = [self.memory[add], self.memory[add + 1]];
        u16::from_le_bytes(values)
    }

    pub fn memory_write_word(&mut self, address: u32, value: u16) {
        let add = address as usize;
        let v = u16::to_le_bytes(value);
        self.memory[add] = v[0];
        self.memory[add + 1] = v[1];
    }

    pub fn memory_read_byte(&self, address: u32) -> u8 {
        let add = address as usize;
        self.memory[add]
    }

    pub fn memory_write_byte(&mut self, address: u32, value: u8) {
        let add = address as usize;
        self.memory[add] = value
    }
}
