
pub struct Machine {
    gas_table: GasTable,
    gas_extra: GasExtra,
    space_cap: SpaceCap,
    gas_limit: i64,
    global_vals: KVMap,
    memory_vals: HashMap<ContractAddress, KVMap>,
    // call_stacks: CallStack,
    extn_caller: Box<dyn ExtActCaller>,
    out_storage: Box<dyn OutStorager>,
    // entry_codes: Vec<u8>,
    contract_cache: HashMap<ContractAddress, ContractStorage>,
}



impl Machine {

    pub fn new(
        gas_limit: i64, 
        extn_caller: Box<dyn ExtActCaller>,
        out_storage: Box<dyn OutStorager>,
    ) -> Machine {
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
            extn_caller,
            out_storage,
            contract_cache: HashMap::new(),
        }
    }



    pub fn printdebug(&mut self) {
        println!("gas_limit = {}", self.gas_limit)
    }

}






