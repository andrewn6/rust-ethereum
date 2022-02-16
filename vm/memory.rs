use alloc::vec::Vec;
use primitive_types::U256

pub struct Memory {
    store: Vec<u8>,
    lastGascost: U256
}
