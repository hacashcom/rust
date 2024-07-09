




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

    pub fn clear(&mut self) {
        self.global_vals.clear();
        self.memory_vals.clear();
        self.contract_count.clear();
    }

}



impl Machine<'_> {

    pub fn release_resource(mut self) -> Resoure {
        self.r.clear();
        self.r
    }



}


