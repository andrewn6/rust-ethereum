// uint256
use primitive_types::H256;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Stack {
    // set data to uint256
    data: Vec<H256>,
    // used for limiting length of stacks
    limit: usize
}

// Implementation of Stack
impl Stack {
    pub fn new_stack(limit: usize) -> Self {
        Self {
            data:: Vec::new(),
            limit,
        }
    }

    #[inline]
    pub fn limit(&self) -> usize {
        self.data.len()
    }
    
    pub fn length(&self) ->  usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> usize {
        self.data.is_empty()
    }

    pub fn data(&self) -> &Vec<H256> {
        // return &self.data
        &self.data
    }
}
