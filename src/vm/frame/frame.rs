

/**
*
*/
#[derive(Debug, Default)]
pub struct Frame {
    pub depth: usize, // max 16
    pub codes: Vec<u8>,
    pub pc: usize,
    pub mode: CallMode,
    pub local: Stack,
    pub stack: Stack,
    pub heap: Heap,
    //
    pub ivk_addr: ContractAddress, // call func
    pub ctx_addr: ContractAddress, // storage and local ctx
} 


pub struct FrameExec<'a, 'b, 'c> {
    is_sys_call: bool,
    depth: usize,
    //
    codes: &'a [u8],
    pub pc: &'a mut usize,
    mode: &'a CallMode,
    stack: &'a mut Stack,
    local: &'a mut Stack,
    heap: &'a mut Heap,
    ivk_addr: &'a ContractAddress,
    ctx_addr: &'a ContractAddress,
    // machine
    pub gas_limit: &'b mut i64,
    gas_table: &'b GasTable,
    gas_extra: &'b GasExtra,
    extn_caller: &'c mut dyn ExtActCaller,
    out_storage: &'c mut dyn OutStorager,
    memory: &'c mut AddrKVMap,
    global: &'c mut KVMap,
}


impl Frame {

    /*
    pub fn new(cap: &SpaceCap, prev: Option<&Frame>, 
        ivk: ContractAddress, sto: ContractAddress, 
        mode: CallMode, deep: usize, codes: Vec<u8>, input: StackItem
    ) -> VmrtRes<Frame> {
        // stack and local cap
        let (st, lt) = Self::locstacap(cap, prev)?;
        // function argv
        let mut locals = Stack::new(st);
        locals.push(input).unwrap(); // function args
        Ok(Frame {
            mode: mode,
            depth: deep, // max 16
            pc:  0,
            codes: codes,
            stack: Stack::new(lt),
            heap: Heap::new(cap.max_heap_seg),
            local: locals,
            ivk_addr: ivk,
            ctx_addr: sto,
        })
    }
    */

    pub fn new(cap: &SpaceCap, prev: Option<&Frame>, 
        ivk: ContractAddress, sto: ContractAddress, 
        mode: CallMode, deep: usize, codes: Vec<u8>, input: StackItem
    ) -> VmrtRes<Frame> {
        let mut f = Frame::default();
        f.init(cap, prev, ivk, sto, mode, deep, codes, input)?;
        Ok(f)
    }
    

    pub fn init(&mut self, cap: &SpaceCap, prev: Option<&Frame>, 
        ivk: ContractAddress, sto: ContractAddress, 
        mode: CallMode, deep: usize, codes:Vec<u8>, input: StackItem
    ) -> VmrtErr {
        // clear
        self.local.clear();
        self.stack.clear();
        self.heap.clear();
        // stack and local cap
        let (st, lt) = Self::locstacap(cap, prev)?;
        self.stack.set_limit(st);
        self.local.set_limit(lt);
        self.heap.set_limit(cap.max_heap_seg);
        // reset
        self.mode = mode;
        self.depth = deep;
        self.pc =  0;
        self.codes = codes;
        self.ivk_addr = ivk;
        self.ctx_addr = sto;
        Ok(())
    }


    fn locstacap(cap: &SpaceCap, prev: Option<&Frame>) -> VmrtRes<(usize, usize)> {
        let sm = cap.total_stack;
        let lm = cap.total_local;
        let mut st = sm;
        let mut lt = lm;
        if let Some(prev) = prev {
            let sl = prev.stack.len();
            let ll = prev.local.len();
            if sl >= sm {
                return itr_err_code!(OutOfStack)
            }
            if ll >= lm {
                return itr_err_code!(OutOfLocal)
            }
            st = sm - sl;
            lt = lm - ll;
        }
        Ok((st, lt))
    }


    /*****************************************************/

    pub fn exec<'a, 'b, 'c>(&'a mut self, 
        gas_limit: &'b mut i64,
        gas_table: &'b GasTable,
        gas_extra: &'b GasExtra,
        extn_caller: &'c mut dyn ExtActCaller,
        out_storage: &'c mut dyn OutStorager,
        memory: &'c mut AddrKVMap,
        global: &'c mut KVMap,
        is_sys_call: bool,
    ) -> FrameExec<'a, 'b, 'c> {
        FrameExec {
            is_sys_call,
            depth: self.depth,
            mode: &self.mode,
            pc: &mut self.pc,
            codes: &self.codes,
            stack: &mut self.stack,
            local: &mut self.local,
            heap: &mut self.heap,
            ivk_addr: &self.ivk_addr,
            ctx_addr: &self.ctx_addr,
            gas_limit,
            gas_table,
            gas_extra,
            extn_caller,
            out_storage,
            memory,
            global,
        } 
    }


    
}


