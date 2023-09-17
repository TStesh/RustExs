// BitField 2KB
pub struct BitField([u8; 2048]);

impl BitField {
    pub fn new(init_value: u8) -> Self { Self([init_value; 2048]) }
    //  прочитать бит в заданной позиции
    pub fn has(&self, index: u32) -> bool {
        let (byte_index, offset) = (index >> 3, index % 8);
        self.0[byte_index as usize] & (1 << (7 - offset)) > 0
    }
    // записать бит в заданную позицию
    pub fn set(&mut self, index: u32) {
        let (byte_index, offset) = (index >> 3, index % 8);
        self.0[byte_index as usize] |= (1 << (7 - offset));
    }
}
