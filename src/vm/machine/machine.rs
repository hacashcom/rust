
pub struct Machine {
    gas_limit: i64,
    gas_table: [u8; 256],
    gas_extra: GasExtra,
    space_limit: SpaceLimit,
    // entry_codes: &[u8],
    global_vals: KVMap,
    memory_secs: HashMap<Address, KVMap>,
    // 
    out_storage: u8,
    code_loader: u8,
}








