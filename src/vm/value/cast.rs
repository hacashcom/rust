



macro_rules! cast_up_to_low {
    ($self: expr, $t1: ty, $t11: ident, $t2: ty, $t22: ident) => {
        if let $t22(n) = $self {
            if *n <= <$t1>::MAX as $t2 {
                *$self = $t11(*n as $t1);
                return Ok(())
            }
        }
    }
}


macro_rules! cast_low_to_up {
    ($self: expr, $t11: ident, $t2: ty, $t22: ident) => {
        if let $t11(n) = $self { 
            *$self = $t22(*n as $t2); 
            return Ok(()) 
        }
    }
}


fn cannot_cast_err(v: &StackItem, ty: &str) -> VmrtErr {
    itr_err_fmt!(CastFail, "cannot cast {:?} to {}", v, ty)
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
    
    pub fn is_zero(&self) -> bool {
        ! self.is_not_zero()
    }

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

    pub fn cast_u8(&mut self) -> VmrtErr {
        if let U8(_) = self { return Ok(()) }
        cast_up_to_low!{self, u8, U8, u16, U16}
        cast_up_to_low!{self, u8, U8, u32, U32}
        cast_up_to_low!{self, u8, U8, u64, U64}
        cast_up_to_low!{self, u8, U8, u128, U128}
        cannot_cast_err(self, "U8") // error
    }

    pub fn cast_u16(&mut self) -> VmrtErr {
        cast_low_to_up!{self, U8, u16, U16}
        if let U16(_) = self { return Ok(()) }
        cast_up_to_low!{self, u16, U16, u32, U32}
        cast_up_to_low!{self, u16, U16, u64, U64}
        cast_up_to_low!{self, u16, U16, u128, U128}
        cannot_cast_err(self, "U16") // error
    }

    pub fn cast_u32(&mut self) -> VmrtErr {
        cast_low_to_up!{self, U8, u32, U32}
        cast_low_to_up!{self, U16, u32, U32}
        if let U32(_) = self { return Ok(()) }
        cast_up_to_low!{self, u32, U32, u64, U64}
        cast_up_to_low!{self, u32, U32, u128, U128}
        cannot_cast_err(self, "U32") // error
    }

    pub fn cast_u64(&mut self) -> VmrtErr {
        cast_low_to_up!{self, U8, u64, U64}
        cast_low_to_up!{self, U16, u64, U64}
        cast_low_to_up!{self, U32, u64, U64}
        if let U64(_) = self { return Ok(()) }
        cast_up_to_low!{self, u64, U64, u128, U128}
        cannot_cast_err(self, "U64") // error
    }

    pub fn cast_u128(&mut self) -> VmrtErr {
        cast_low_to_up!{self, U8, u128, U128}
        cast_low_to_up!{self, U16, u128, U128}
        cast_low_to_up!{self, U32, u128, U128}
        cast_low_to_up!{self, U64, u128, U128}
        if let U128(_) = self { return Ok(()) }
        cannot_cast_err(self, "U128") // ERROR
    }

    pub fn cast_buf(&mut self) -> VmrtErr {
        match &self {
            Nil => *self = Buffer(vec![]),
            U8(n) =>   *self = Buffer(n.to_be_bytes().into()),
            U16(n) =>  *self = Buffer(n.to_be_bytes().into()),
            U32(n) =>  *self = Buffer(n.to_be_bytes().into()),
            U64(n) =>  *self = Buffer(n.to_be_bytes().into()),
            U128(n) => *self = Buffer(n.to_be_bytes().into()),
            Buffer(buf) => {},
        };
        Ok(())
    }






}






