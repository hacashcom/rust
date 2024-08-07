
pub struct Machine<'a> {
    /*
    code_load: Arc<Mutex<ContractLoader>>,
    gas_table: GasTable,
    gas_extra: GasExtra,
    space_cap: SpaceCap,
    gas_limit: i64,
    global_vals: KVMap,
    memory_vals: HashMap<ContractAddress, KVMap>,
    contract_count: HashSet<ContractAddress>,
    */
    r: Resoure,
    pending_height: u64,
    gas_limit: i64,
    // call_stacks: CallStack,
    extn_caller: &'a mut dyn ExtActCaller,
    out_storage: &'a mut dyn OutStorager,
    out_storead: &'a mut dyn OutStoragerRead,
    // entry_codes: Vec<u8>,
}



impl Machine<'_> {

    pub fn new_by_resouce<'a>(
        pending_height: u64,
        gas_limit: i64, 
        extn_caller: &'a mut dyn ExtActCaller,
        out_storage: &'a mut dyn OutStorager,
        out_storead: &'a mut dyn OutStoragerRead,
        r: Resoure,
    ) -> Machine<'a> {;
        Machine {
            r,
            pending_height,
            gas_limit,
            extn_caller,
            out_storage,
            out_storead,
        }
    }


    pub fn new<'a>(
        pending_height: u64,
        gas_limit: i64, 
        extn_caller: &'a mut dyn ExtActCaller,
        out_storage: &'a mut dyn OutStorager,
        out_storead: &'a mut dyn OutStoragerRead,
        code_load: Arc<Mutex<ContractLoader>>,
    ) -> Machine<'a> {
        let space_cap = SpaceCap::new();
        let gas_table = GasTable::new();
        let gas_extra = GasExtra::new();
        // let call_stacks = CallStack::new();
        let scmg = space_cap.max_global;
        let scmm = space_cap.max_memory;
        Machine {
            r: Resoure{
                code_load,
                gas_table,
                gas_extra,
                space_cap,
                // call_stacks,
                global_vals: KVMap::new(scmg),
                memory_vals: AddrKVMap::new(scmm),
                contract_count: HashSet::new(),
                frame_pools: Vec::new(),
            },
            pending_height,
            gas_limit,
            extn_caller,
            out_storage,
            out_storead,
        }
    }

    pub fn gas_refund(&self) -> i64 {
        self.gas_limit
    }



    pub fn printdebug(&mut self) {
        println!("gas_limit = {}", self.gas_limit)
    }

}











