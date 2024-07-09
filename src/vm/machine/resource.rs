




pub struct Resoure {
    pub code_load: Arc<Mutex<ContractLoader>>,
    pub gas_table: GasTable,
    pub gas_extra: GasExtra,
    pub space_cap: SpaceCap,
    pub global_vals: KVMap,
    pub memory_vals: HashMap<ContractAddress, KVMap>,
    pub contract_count: HashSet<ContractAddress>,
}


impl Resoure {

    pub fn reset(&mut self) {
        
    }

}



impl Machine<'_> {

    pub fn release_resource(mut self) -> Resoure {
        self.r.reset();
        self.r
    }



}


