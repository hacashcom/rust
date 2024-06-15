
#[derive(Debug, Clone)]
pub enum StackItem {
    Nil,             // type_id = 0
    U8(u8),          //           1
    U16(u16),        //           2
    U32(u32),        //           3
    U64(u64),        //           4
    U128(u128),      //           5
    // U256(u256),   //           6
    Buffer(Vec<u8>), //           7
}

use StackItem::*;

impl StackItem {

    pub fn nil() -> StackItem {
        Nil
    }
    pub fn empty_buf() -> StackItem {
        Buffer(vec![])
    }

    pub fn val_size(&self) -> usize {
        match self {
            Nil => 0,
            U8(_) => 1,
            U16(_) => 2,
            U32(_) => 4,
            U64(_) => 8,
            U128(_) => 16,
            // U256(_) => 32,
            Buffer(b) => b.len(),
            _ => 0, 
        }
    }


}



