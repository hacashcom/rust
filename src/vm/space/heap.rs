
const HEAP_SEG_SIZE: usize = 256;

#[derive(Debug, Default)]
pub struct Heap {
    limit: usize,
    memdt: Vec<u8>,
}


impl Heap {
    pub fn new(lmt: usize) -> Heap {
        Heap{
            limit: lmt,
            memdt: Vec::new(),
        }
    }
    
    pub fn set_limit(&mut self, lmt: usize) {
        self.limit = lmt;
    }

    pub fn clear(&mut self) {
        self.memdt.clear();
    }
}



/**
* operand
*/
impl Heap {


    /**
    * use gas
    */
    pub fn grow(&mut self, n: u8) -> VmrtRes<i64> {
        if n == 0 {
            return itr_err_fmt!(OutOfHeap, "grow num cannot be zero")
        }
        let num = n as usize;
        let has = self.memdt.len() / HEAP_SEG_SIZE;
        // check
        let ttm = has + num;
        if ttm > self.limit {
            return itr_err_code!(OutOfHeap)
        }
        self.memdt.resize(ttm * HEAP_SEG_SIZE, 0);
        // calc gas
        let mut gsd = 0i64;
        for i in has .. ttm {
            gsd += 2u32.pow(i as u32) as i64 * 8 ;
        }
        Ok(gsd)
    }


    pub fn read(&self, ost: &StackItem, len: &StackItem) -> VmrtRes<StackItem> {
        let o = ost.to_u16()?;
        let l = len.to_u16()?;
        let v = self.read_ex(o, l)?;
        Ok(StackItem::buf(v))
    }


    pub fn readu(&self, x: u8) -> VmrtRes<StackItem> {
        let l = match x & 0b11000000 {
            0b00000000 => 1,
            0b01000000 => 2,
            0b10000000 => 4,
            0b11000000_ => 8, 
            _ => return itr_err_fmt!(HeapError, "read param invalid"),
        };
        let o = x & 0b00111111;
        let v = self.read_ex(o as u16, l)?;
        buf_to_uint(&v)
    }


    pub fn readul(&self, x: u16) -> VmrtRes<StackItem> {
        let mut xs = x.to_be_bytes();
        let l = match xs[0] & 0b11000000 {
            0b00000000 => 1,
            0b00100000 => 2,
            0b01000000 => 4,
            0b01100000 => 8,
            0b10000000 => 16,
            0b10100000 |
            0b11000000 |
            0b11100000 |
            _ => return itr_err_fmt!(HeapError, "read param invalid"),
        };
        xs[0] &= 0b00011111;
        let v = self.read_ex(u16::from_be_bytes(xs), l)?;
        buf_to_uint(&v)
    }


    pub fn writex(&mut self, ost: u8, val: &StackItem) -> VmrtErr {
        let o = ost as u16;
        let v = val.to_buf()?;
        self.write_ex(o, v)

    }

    pub fn writexl(&mut self, ost: u16, val: &StackItem) -> VmrtErr {
        let o = ost as u16;
        let v = val.to_buf()?;
        self.write_ex(o, v)
    }


    pub fn write(&mut self, ost: &StackItem, val: &StackItem) -> VmrtErr {
        let o = ost.to_u16()?;
        let v = val.to_buf()?;
        self.write_ex(o, v)
    }



    /////////////////////////////////////////////////////


    // 
    pub fn read_ex(&self, ost: u16, len: u16) -> VmrtRes<Vec<u8>> {
        let start = ost as usize;
        let end = start + len as usize;
        if end > self.memdt.len() {
            return itr_err_code!(OutOfHeap)
        }
        // read
        Ok(self.memdt[start .. end].to_vec())
    }

    // 
    pub fn write_ex(&mut self, ost: u16, val: Vec<u8>) -> VmrtErr {
        let start = ost as usize;
        let end = start + val.len();
        if end > self.memdt.len() {
            return itr_err_code!(OutOfHeap)
        }
        // write
        self.memdt[start .. end].copy_from_slice(&val);
        Ok(())
    }





}




