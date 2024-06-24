

/**
*
*/
#[derive(Debug)]
pub struct Frame {
    pub depth: usize, // max 16
    pub codes: Vec<u8>,
    pub pc: usize,
    pub mode: CallMode,
    pub local: Stack,
    pub stack: Stack,
    //
    // pub ivk_addr: ContractAddress, // call func
    // pub sto_addr: ContractAddress, // storage ctx
} 


pub struct FrameExec<'a, 'b> {
    depth: &'a usize,
    codes: &'a [u8],
    pub pc: &'a mut usize,
    mode: &'a CallMode,
    local: &'a mut Stack,
    stack: &'a mut Stack,
    // machine
    pub gas_limit: &'b mut i64,
    gas_table: &'b GasTable,
    gas_extra: &'b GasExtra,
} 


impl Frame {

    pub fn new(mode: CallMode, deep: usize, codes:Vec<u8>, input: StackItem) -> Frame {
        let mut locals = Stack::new(256);
        locals.push(input).unwrap(); // function args
        Frame {
            mode: mode,
            depth: deep, // max 16
            pc:  0,
            codes: codes,
            local: locals,
            stack: Stack::new(256),
        }
    }

    pub fn exec<'a, 'b>(&'a mut self, 
        gas_limit: &'b mut i64,
        gas_table: &'b GasTable,
        gas_extra: &'b GasExtra,
    ) -> FrameExec<'a, 'b> {
        FrameExec {
            mode: &self.mode,
            pc: &mut self.pc,
            codes: &self.codes,
            depth: &self.depth,
            local: &mut self.local,
            stack: &mut self.stack,
            gas_limit,
            gas_table,
            gas_extra,
        } 
    }


    
}


