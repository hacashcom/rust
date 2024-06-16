

/**
*
*/
pub struct CallFrame<'a> {
    pub deep: usize, // max 16
    pub pc: usize,
    pub codes: &'a [u8],
    pub local: Stack,
    pub stack: Stack,
} 


pub struct CallFrameExec<'a> {
    pub pc: &'a mut usize,
    pub codes: &'a [u8],
    pub local: &'a mut Stack,
    pub stack: &'a mut Stack,
} 


impl CallFrame<'_> {


    pub fn new<'a>(codes: &'a [u8], input: StackItem) -> CallFrame<'a> {
        let mut locals = Stack::new(256);
        locals.push(input).unwrap(); // function args
        CallFrame {
            deep: 0, // max 16
            pc:  0,
            codes: codes,
            local: locals,
            stack: Stack::new(256),
        }
    }

    pub fn exec<'a>(&'a mut self) -> CallFrameExec<'a> {
        CallFrameExec {
            pc: &mut self.pc,
            codes: &self.codes,
            local: &mut self.local,
            stack: &mut self.stack,
        } 
    }


    
}

