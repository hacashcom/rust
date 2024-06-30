

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
    pub ivk_addr: ContractAddress, // call func
    pub ctx_addr: ContractAddress, // storage and local ctx
} 


pub struct FrameExec<'a, 'b> {
    is_sys_call: bool,
    depth: usize,
    //
    codes: &'a [u8],
    pub pc: &'a mut usize,
    mode: &'a CallMode,
    local: &'a mut Stack,
    stack: &'a mut Stack,
    ivk_addr: &'a ContractAddress,
    ctx_addr: &'a ContractAddress,
    // machine
    pub gas_limit: &'b mut i64,
    gas_table: &'b GasTable,
    gas_extra: &'b GasExtra,
    extn_caller: &'b dyn ExtActCaller,
} 


impl Frame {

    pub fn new(ivk: ContractAddress, sto: ContractAddress, mode: CallMode, deep: usize, codes:Vec<u8>, input: StackItem) -> Frame {
        let mut locals = Stack::new(256);
        locals.push(input).unwrap(); // function args
        Frame {
            mode: mode,
            depth: deep, // max 16
            pc:  0,
            codes: codes,
            local: locals,
            stack: Stack::new(256),
            ivk_addr: ivk,
            ctx_addr: sto,
        }
    }

    pub fn exec<'a, 'b>(&'a mut self, 
        gas_limit: &'b mut i64,
        gas_table: &'b GasTable,
        gas_extra: &'b GasExtra,
        extn_caller: &'b dyn ExtActCaller,
        is_sys_call: bool,
    ) -> FrameExec<'a, 'b> {
        FrameExec {
            is_sys_call,
            depth: self.depth,
            mode: &self.mode,
            pc: &mut self.pc,
            codes: &self.codes,
            local: &mut self.local,
            stack: &mut self.stack,
            ivk_addr: &self.ivk_addr,
            ctx_addr: &self.ctx_addr,
            gas_limit,
            gas_table,
            gas_extra,
            extn_caller,
        } 
    }


    
}


