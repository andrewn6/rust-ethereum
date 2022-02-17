use alloc::vec::Vec;
use primitive_types::U256;

pub struct Memory {
    data: Vec<u8>,
    effective_len: U256,
    limit: usize,
    store: Vec<u8>,
    lastGascost: U256, 
}

impl Memory {
    fn new(limit: usize) -> Self {
        Self {
            data: Vec::new()
        }
    }
}
