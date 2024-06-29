
pub struct Machine<'a> {
    gas_table: GasTable,
    gas_extra: GasExtra,
    space_cap: SpaceCap,
    gas_limit: i64,
    global_vals: KVMap,
    memory_vals: HashMap<ContractAddress, KVMap>,
    // call_stacks: CallStack,
    extcaller: &'a mut dyn ExtActCaller,
    out_storage: u8,
    // 
    code_loader: u8,
    // 
    // entry_codes: Vec<u8>,
}



impl Machine<'_> {

    pub fn new<'a>(gas_limit: i64, extcaller: &'a mut dyn ExtActCaller) -> Machine<'a> {
        let space_cap = SpaceCap::new();
        let gas_table = GasTable::new();
        let gas_extra = GasExtra::new();
        // let call_stacks = CallStack::new();
        Machine {
            gas_limit,
            gas_table,
            gas_extra,
            space_cap,
            // call_stacks,
            global_vals: KVMap::new(),
            memory_vals: HashMap::new(),
            extcaller,
            out_storage: 0,
            code_loader: 0,
        }
    }



    pub fn printdebug(&mut self) {
        println!("gas_limit = {}", self.gas_limit)
    }

}






