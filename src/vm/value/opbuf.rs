



impl StackItem {

    
    pub fn opbuf_cut(&mut self, ost: StackItem, len: StackItem) -> VmrtErr {
        let ost = ost.to_uint16()? as usize;
        let len = len.to_uint16()? as usize;
        let end = ost + len;
        let buf = self.to_buf();
        let bfl = buf.len();
        if end > u16::MAX as usize {
            return itr_err_fmt!(BufferOpFail, "buffer cut param({}, {}) overflow", ost, len)
        }
        if end > bfl {
            return itr_err_fmt!(BufferOpFail, "buffer length {} too short", bfl)
        }
        *self = Buffer(buf[ost..end].to_vec());
        Ok(())
    }

    pub fn opbuf_cat(&mut self, src: StackItem) -> VmrtErr {
        let mut dst = self.to_buf();
        let mut src = src.to_buf();
        dst.append(&mut src);
        if dst.len() > u16::MAX as usize {
            return itr_err_fmt!(BufferOpFail, "buffer length {} too long", dst.len())
        }
        *self = Buffer(dst);
        Ok(())
    }

    pub fn opbuf_byte(&mut self, idx: StackItem) -> VmrtErr {
        let idx = idx.to_uint16()? as usize;
        let buf = self.to_buf();
        let bfl = buf.len();
        if idx >= bfl {
            return itr_err_fmt!(BufferOpFail, "buffer length {} too short", bfl)
        } 
        *self = U8(buf[idx]);
        Ok(())
    }


}