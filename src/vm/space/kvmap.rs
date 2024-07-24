
#[derive(Default)]
pub struct KVMap {
    outoferr: ItrErrCode,
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
        self.datas.entry(addr).or_insert(
            KVMap::new_of_err(self.limit, ItrErrCode::OutOfMemory)
        )
    }

    pub fn clear(&mut self) {
        self.datas.clear();
    }

}


impl KVMap {

    pub fn new_of_err(lmt: usize, oter: ItrErrCode) -> KVMap {
        KVMap {
            outoferr: oter,
            limit: lmt, // 20 or 16
            datas: HashMap::new(),
        }
    }

    pub fn new(lmt: usize) -> KVMap {
        Self::new_of_err(lmt, ItrErrCode::OutOfGlobal)
    }

    pub fn clear(&mut self) {
        self.datas.clear();
    }

}


/***************** operand *****************/


macro_rules! kvmap_check_key {
    ($k: expr, $do: expr) => {
        {
            let kbuf = $k.to_buf()?;
            let kl = kbuf.len();
            if kl <= 0 {
                return itr_err_fmt!(KVStoreError, "cannot {} with empty key", $do)
            }
            if kl > 64 {
                return itr_err_fmt!(KVStoreError, "cannot {} with key of length over 64 bytes", $do)
            }
            kbuf
        }
    }
}


impl KVMap {

    pub fn put(&mut self, k: &StackItem, v: StackItem) -> VmrtErr {
        let kbuf = kvmap_check_key!(k, "put");
        if v.is_nil() {
            self.datas.remove(&kbuf);
            return Ok(()) // delete
        }
        if self.datas.len() >= self.limit {
            return itr_err_code!(self.outoferr)
        }
        self.datas.insert(kbuf, v);
        Ok(())
    }

    pub fn get(&self, k: &StackItem) -> VmrtRes<StackItem> {
        let kbuf = kvmap_check_key!(k, "get");
        let v = match self.datas.get(&kbuf) {
            Some(v) => v.clone(),
            _ => StackItem::nil(), // not find
        };
        Ok(v)
    }

}

