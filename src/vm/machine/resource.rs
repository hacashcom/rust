


pub struct Resoure {
    pub code_load: Arc<Mutex<ContractLoader>>,
    pub gas_table: GasTable,
    pub gas_extra: GasExtra,
    pub space_cap: SpaceCap,
    pub memory_vals: AddrKVMap,
    pub global_vals: KVMap,
    pub contract_count: HashSet<ContractAddress>,
    // space cache
    pub frame_pools: Vec<Frame>,
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

    pub fn reclaim_frame(&mut self, f: Frame) {
        self.r.frame_pools.push(f);
    }

    pub fn reclaim_frames(&mut self, fs: Vec<Frame>) {
        for f in fs {
            self.reclaim_frame(f);
        }
    }

    pub fn alloc_frame(&mut self) -> Frame {
        match self.r.frame_pools.pop() {
            Some(f) => f,
            _ => Frame::default(),
        }
    }


}


