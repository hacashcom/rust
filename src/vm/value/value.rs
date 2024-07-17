
/*
#[derive(Debug, Clone)]
pub enum ValueWrap {
    None,
    // ctrl flow
    Continue,
    Break,
    Return(StackItem),
    // val
    Value(StackItem),
}


*/




macro_rules! to_uint_up_to_low {
    ($self: expr, $t1: ident, $t2: ident, $t3: ident) => {
        if let $t1(n) = $self {
            if *n <= <$t3>::MAX as $t2 {
                return Ok(*n as $t3)
            }
        }
    }
}


macro_rules! to_uint_low_to_up {
    ($self: expr, $t1: ident, $t2: ident) => {
        if let $t1(n) = $self { 
            return Ok(*n as $t2)
        }
    }
}


macro_rules! to_uint_for_buf {
    ($self: expr, $sz: expr, $t1: ident ) => {
        if let Buffer(buf) = $self {
            let rlbts = buf_drop_left_zero(buf);
            if rlbts.len() <= $sz {
                let bts = buf_fill_left_zero(&rlbts, $sz);
                let v = <$t1>::from_be_bytes(bts.try_into().unwrap());
                return Ok(v)
            }
        }
    }
}






impl StackItem {

    pub fn is_not_zero(&self) -> bool {
        match self {
            Nil => false,
            U8(n)   => *n != 0,
            U16(n)  => *n != 0,
            U32(n)  => *n != 0,
            U64(n)  => *n != 0,
            U128(n) => *n != 0,
            Buffer(b)  => buf_is_not_zero(b),
        }
    }
    
    pub fn to_bool(&self) -> bool {
        self.is_not_zero()
    }
    
    pub fn is_zero(&self) -> bool {
        ! self.is_not_zero()
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

    pub fn to_buf(&self) -> Vec<u8> {
        match &self {
            Nil => vec![],
            U8(n) =>   n.to_be_bytes().into(),
            U16(n) =>  n.to_be_bytes().into(),
            U32(n) =>  n.to_be_bytes().into(),
            U64(n) =>  n.to_be_bytes().into(),
            U128(n) => n.to_be_bytes().into(),
            Buffer(buf) => buf.clone(),
        }
    }

    pub fn to_uint8(&self) -> VmrtRes<u8> {
        if let U8(n) = self { return Ok(*n) }
        to_uint_up_to_low!{self, U16,  u16,  u8}
        to_uint_up_to_low!{self, U32,  u32,  u8}
        to_uint_up_to_low!{self, U64,  u64,  u8}
        to_uint_up_to_low!{self, U128, u128, u8}
        to_uint_for_buf!{self, 1, u8}
        cannot_cast_err!(self, "u8") // error
    }

    pub fn to_uint16(&self) -> VmrtRes<u16> {
        to_uint_low_to_up!{self, U8, u16}
        if let U16(n) = self { return Ok(*n) }
        to_uint_up_to_low!{self, U32,  u32,  u16}
        to_uint_up_to_low!{self, U64,  u64,  u16}
        to_uint_up_to_low!{self, U128, u128, u16}
        to_uint_for_buf!{self, 2, u16}
        cannot_cast_err!(self, "u16") // error
    }

    pub fn to_uint32(&self) -> VmrtRes<u32> {
        to_uint_low_to_up!{self, U8, u32}
        to_uint_low_to_up!{self, U16, u32}
        if let U32(n) = self { return Ok(*n) }
        to_uint_up_to_low!{self, U64,  u64,  u32}
        to_uint_up_to_low!{self, U128, u128, u32}
        to_uint_for_buf!{self, 4, u32}
        cannot_cast_err!(self, "u32") // error
    }

    pub fn to_uint64(&self) -> VmrtRes<u64> {
        to_uint_low_to_up!{self, U8,  u64}
        to_uint_low_to_up!{self, U16, u64}
        to_uint_low_to_up!{self, U32, u64}
        if let U64(n) = self { return Ok(*n) }
        to_uint_up_to_low!{self, U128, u128, u64}
        to_uint_for_buf!{self, 8, u64}
        cannot_cast_err!(self, "u64") // error
    }

    pub fn to_uint128(&self) -> VmrtRes<u128> {
        to_uint_low_to_up!{self, U8,  u128}
        to_uint_low_to_up!{self, U16, u128}
        to_uint_low_to_up!{self, U32, u128}
        to_uint_low_to_up!{self, U64, u128}
        if let U128(n) = self { return Ok(*n) }
        to_uint_for_buf!{self, 16, u128}
        cannot_cast_err!(self, "u128") // error
    }

    
}