

pub struct KVMap {
    limit: usize,
    datas: HashMap<u32, StackItem>,
}

impl KVMap {

    pub fn new() -> KVMap {
        KVMap {
            limit: 32,
            datas: HashMap::new(),
        }
    }




    

}


