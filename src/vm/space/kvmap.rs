
#[derive(Default)]
pub struct KVMap {
    limit: usize,
    datas: HashMap<Vec<u8>, StackItem>,
}

#[derive(Default)]
pub struct AddrKVMap{
    limit: usize,
    datas: HashMap<ContractAddress, KVMap>,
}


impl AddrKVMap {

    pub fn new(lmt: usize) -> AddrKVMap {
        AddrKVMap {
            limit: lmt,
            datas: HashMap::new(),
        }
    }

    pub fn entry(&mut self, addr: ContractAddress) -> &mut KVMap {
        self.datas.entry(addr).or_insert(KVMap::new(self.limit))
    }

    pub fn clear(&mut self) {
        self.datas.clear();
    }

}


impl KVMap {

    pub fn new(lmt: usize) -> KVMap {
        KVMap {
            limit: lmt, // 20 or 16
            datas: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.datas.clear();
    }

}




