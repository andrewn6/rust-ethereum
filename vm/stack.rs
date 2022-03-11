// uint256
use primitive_types::H256
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Stack {
    // set data to uint256
    data: Vec<H256>,
    limit: usize,
}
