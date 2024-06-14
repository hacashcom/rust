
#[derive(Debug, Clone)]
pub enum StackItem {
    // Nil,          // type_id = 0
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

    pub fn empty_buf() -> StackItem {
        Buffer(vec![])
    }

    pub fn val_size(&self) -> usize {
        match self {
            // Nil => 0,
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

    pub fn cast_bool(&mut self) -> RetErr {
        let notz = match self {
            // Nil     => false,
            U8(n)   => *n != 0,
            U16(n)  => *n != 0,
            U32(n)  => *n != 0,
            U64(n)  => *n != 0,
            U128(n) => *n != 0,
            Buffer(b)  => buf_is_not_zero(b),
            s => return errf!("cannot cast {:?} to bool", s),
        };
        if notz {
            *self = U8(1); // true
        } else {
            *self = U8(0); // false
        }
        Ok(())
    }

}





/**
* ret: change left(-1) nothing(0) or right(1), err is cannot do cast
*/
pub fn castv(l: &mut StackItem, r: &mut StackItem) -> Ret<i8> {
    match (l, r) {
        (U8(_),     U8(_))     => Ok(0),
        (U16(_),    U16(_))    => Ok(0),
        (U32(_),    U32(_))    => Ok(0),
        (U64(_),    U64(_))    => Ok(0),
        (U128(_),   U128(_))   => Ok(0),
        (Buffer(_), Buffer(_)) => Ok(0),

        (l, r) => errf!("cannot do cast between with type {:?} and {:?}", l, r),
    }
}

