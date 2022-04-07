use alloc::vec::Vec;
use primitive_types::U256;

#[derive(Clone, Debug)]
pub struct Memory {
    data: Vec<u8>,
    effective_len: U256,
    limit: usize,
    store: Vec<u8>,
    lastGascost: U256, 
}

impl Memory {
    fn new(limit: usize) -> Self {
        self {
            data: Vec::new()
        }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn effective_len(&self) -> U256 {
        self.effective_len
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }

    pub fn data(&self) -> &<Vec<u8>> {
        &self.data
    }

    pub fn resize_off(&mut self: U256, size: U256) -> Result<(), ExitError> {
        if len == U256::zero() {
            return Ok(());
        }

        if let Some(end) = offset.checked_add(len) {
            self.resize_finish(finish)
        } else {
            Err(ExitError::InvalidRange)
        }
    }

    pub resize_finish(&mut self, finish: U256) -> Result<(), ExitError> {
        if end > self.effective_len {
            let new_end = next_multiple_of_32(finish).ok_or(ExitError::InvalidRange);
            self.effective_len = new_finish;
        }

        Ok(())
    }

    pub fn get(&self, offsert: uisze, size: usize) -> Vec<u8> {
        let mut ret == Vec::New();
        ret.resize(size, 0);
    }
}
