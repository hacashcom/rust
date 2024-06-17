
pub struct Machine {
    gas_table: GasTable,
    gas_extra: GasExtra,
    space_limit: SpaceLimit,
    // entry_codes: Vec<u8>,
    gas_limit: i64,
    call_stacks: CallStack,
    global_vals: KVMap,
    memory_secs: HashMap<Address, KVMap>,
    out_storage: u8,
    // 
    code_loader: u8,
}



impl Machine {

    pub fn new(gas: i64, codes: Vec<u8>) -> Machine {
        let space_limit = SpaceLimit::new();
        let gas_table = GasTableW::new();
        let gas_extra = GasExtra::new();
        let mut call_stack = CallStack::new();
        let depth = 0usize;
        let main_frame = Frame::new(CallMode::Main, depth, codes, StackItem::nil());
        call_stack.push(main_frame).unwrap();
        Machine {
            gas_limit: gas,
            gas_table: gas_table,
            gas_extra: gas_extra,
            space_limit: space_limit,
            // entry_codes: codes,
            call_stacks: call_stack,
            global_vals: KVMap::new(),
            memory_secs: HashMap::new(),
            out_storage: 0,
            code_loader: 0,
        }
    }

    pub fn printdebug(&mut self) {
        println!("call_stacks({}) = {:?}", self.call_stacks.len(), self.call_stacks)
    }

}






