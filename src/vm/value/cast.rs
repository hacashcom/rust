

macro_rules! do_cast_uint {
    ($self: expr, $ty: ident, $f: ident) => {{
        let n = $self.$f()?;
        *$self = $ty(n);
        Ok(())
    }}
}




impl StackItem {

    pub fn cast_bool(&mut self) -> VmrtErr {
        if self.is_not_zero() {
            *self = U8(1); // true
        } else {
            *self = U8(0); // false
        }
        Ok(())
    }

    pub fn cast_bool_not(&mut self) -> VmrtErr {
        if self.is_not_zero() {
            *self = U8(0); // false
        } else {
            *self = U8(1); // true
        }
        Ok(())
    }


    /////////////////////////////////

    pub fn cast_u8(&mut self) -> VmrtErr {
        do_cast_uint!(self, U8, to_uint8)
    }

    pub fn cast_u16(&mut self) -> VmrtErr {
        do_cast_uint!(self, U16, to_uint16)
    }

    pub fn cast_u32(&mut self) -> VmrtErr {
        do_cast_uint!(self, U32, to_uint32)
    }

    pub fn cast_u64(&mut self) -> VmrtErr {
        do_cast_uint!(self, U64, to_uint64)
    }

    pub fn cast_u128(&mut self) -> VmrtErr {
        do_cast_uint!(self, U128, to_uint128)
    }

    /*
    pub fn cast_u16(&mut self) -> VmrtErr {
        cast_low_to_up!{self, U8, u16, U16}
        if let U16(_) = self { return Ok(()) }
        cast_up_to_low!{self, u16, U16, u32, U32}
        cast_up_to_low!{self, u16, U16, u64, U64}
        cast_up_to_low!{self, u16, U16, u128, U128}
        cannot_cast_err!(self, "U16") // error
    }

    pub fn cast_u32(&mut self) -> VmrtErr {
        cast_low_to_up!{self, U8, u32, U32}
        cast_low_to_up!{self, U16, u32, U32}
        if let U32(_) = self { return Ok(()) }
        cast_up_to_low!{self, u32, U32, u64, U64}
        cast_up_to_low!{self, u32, U32, u128, U128}
        cannot_cast_err!(self, "U32") // error
    }

    pub fn cast_u64(&mut self) -> VmrtErr {
        cast_low_to_up!{self, U8, u64, U64}
        cast_low_to_up!{self, U16, u64, U64}
        cast_low_to_up!{self, U32, u64, U64}
        if let U64(_) = self { return Ok(()) }
        cast_up_to_low!{self, u64, U64, u128, U128}
        cannot_cast_err!(self, "U64") // error
    }

    pub fn cast_u128(&mut self) -> VmrtErr {
        cast_low_to_up!{self, U8, u128, U128}
        cast_low_to_up!{self, U16, u128, U128}
        cast_low_to_up!{self, U32, u128, U128}
        cast_low_to_up!{self, U64, u128, U128}
        if let U128(_) = self { return Ok(()) }
        cannot_cast_err!(self, "U128") // ERROR
    }
    */

    pub fn cast_buf(&mut self) -> VmrtErr {
        match &self {
            U8(n) =>   *self = Buffer(n.to_be_bytes().into()),
            U16(n) =>  *self = Buffer(n.to_be_bytes().into()),
            U32(n) =>  *self = Buffer(n.to_be_bytes().into()),
            U64(n) =>  *self = Buffer(n.to_be_bytes().into()),
            U128(n) => *self = Buffer(n.to_be_bytes().into()),
            Buffer(buf) => {},
            _ => return cannot_cast_err!(self, "Buffer") // ERROR
        };
        Ok(())
    }

}





impl StackItem {

    pub fn cast_type_id(&mut self) -> VmrtErr {
        let v = match &self {
            Nil =>     U8(0),
            U8(n) =>   U8(1),
            U16(n) =>  U8(2),
            U32(n) =>  U8(3),
            U64(n) =>  U8(4),
            U128(n) => U8(5),
            // U256(n) => U8(6),
            Buffer(buf) => U8(7),
        };
        *self = v;
        Ok(())
    }


    pub fn cast_size_num(&mut self) -> VmrtErr {
        let v = match &self {
            Nil =>     U16(0),
            U8(n) =>   U16(1),
            U16(n) =>  U16(2),
            U32(n) =>  U16(4),
            U64(n) =>  U16(8),
            U128(n) => U16(16),
            // U256(n) = U16(32),
            Buffer(buf) => {
                let sz = buf.len();
                if sz > u16::MAX as usize {
                    return itr_err_fmt!(BufferOpFail, "buffer size overflow")
                }
                U16( sz as u16 )
            },
        };
        *self = v;
        Ok(())
    }



}




