
const STORAGE_EXPIRED_BLOCK_SECTION: u64 = 100;
const STORAGE_EXPIRED_BLOCK_MAX: u64 = u16::MAX as u64 * STORAGE_EXPIRED_BLOCK_SECTION;



pub struct Storage<'a> {
    hei: u64, // u64
    key: StackItem,
    adr: &'a ContractAddress,
    sto: &'a mut dyn OutStorager,
    cap: &'a SpaceCap,
}


impl Storage<'_> {

    pub fn wrap<'a>(hei: u64, key: StackItem, 
        adr: &'a ContractAddress, 
        sto: &'a mut dyn OutStorager, 
        cap: &'a SpaceCap,
    ) -> Storage<'a> {
        Storage {
            hei,
            key,
            adr,
            sto,
            cap,
        }
    }

}




macro_rules! storage_check_key {
    ($self: expr, $k: expr, $do: expr) => {
        {
            let kbuf = $k.to_buf()?;
            let kbl = kbuf.len();
            if kbl == 0 {
                return itr_err_fmt!(StorageError, "cannot {} storage with empty key", $do)
            }
            if kbl > 64 {
                return itr_err_fmt!(StorageError, "cannot {} storage with key of length over 64 bytes", $do)
            }
            // k + prefix = [3,2]
            contract_state_storage_key($self.adr, kbuf)
        }
    }
}


macro_rules! storage_map_err { ($e: expr) => { 
    $e.map_err(|e|ItrErr::new(StorageError, &format!("{}", &e)))?
} }



impl Storage<'_> {


    /*
    * return gas use
    */
    pub fn rent_time(&mut self, tist: u16) -> VmrtRes<i64> {
        let k = storage_check_key!(self, &self.key, "rent");
        // load old
        let Some((ov, mut ot)) = self.load_exp(&k)? else {
            return itr_err_fmt!(StorageError, "key not find in storage")
        };
        let gas = ov.len() as i64 * tist as i64;
        // new value
        const EB: u64 = STORAGE_EXPIRED_BLOCK_SECTION; // 100 height
        ot += tist as u64 * EB;
        set_in_range!(ot, EB, STORAGE_EXPIRED_BLOCK_MAX);
        let vnew = vec![Uint5::from(ot).serialize(), ov].concat();
        storage_map_err!(self.sto.set(&k, vnew)); // save
        // ok
        Ok(gas)
    }


    /*
    *
    */
    pub fn save(&mut self, val: &StackItem) -> VmrtRes<i64> {
        let k = storage_check_key!(self, &self.key, "load");
        if val.is_nil() {
            storage_map_err!(self.sto.del(&k));
            return Ok(0) // delete, zero gas
        }
        const EB: u64 = STORAGE_EXPIRED_BLOCK_SECTION; // 100 height
        let mut exphei = EB;
        let v = val.to_buf()?;
        let gas = v.len() as i64;
        // load old
        let Some((ov, ot)) = self.load_exp(&k)? else {
            let vnew = vec![Uint5::from(self.hei + exphei).serialize(), v].concat();
            storage_map_err!(self.sto.set(&k, vnew));
            return Ok(gas) // save new
        };
        // reuse expired height
        let mut exphei = ot * ov.len() as u64 / v.len() as u64;
        set_in_range!(exphei, EB, STORAGE_EXPIRED_BLOCK_MAX);
        let vnew = vec![Uint5::from(self.hei + exphei).serialize(), ov].concat();
        storage_map_err!(self.sto.set(&k, vnew)); // save
        Ok(gas)
    }

    /*
    * return (expired, value)
    */
    pub fn load(&mut self) -> VmrtRes<(StackItem, StackItem)> {
        let k = storage_check_key!(self, &self.key, "load");
        let nil = StackItem::nil();
        Ok(match self.load_exp(&k)? {
            None => (nil.clone(), nil),
            Some((v, t)) => (StackItem::U64(t), StackItem::Buffer(v)),
        })
    }


    /* */
    fn load_exp(&mut self, k: &Vec<u8>) -> VmrtRes<Option<(Vec<u8>, u64)>> {
        let v = storage_map_err!(self.sto.get(k));
        let nil = StackItem::nil();
        let Some(bts) = v else {
            return Ok(None) // no find
        };
        if bts.len() < 5 { // 5 = expired block height
            panic!("StorageError, expired block height of item not find");
            // return itr_err_fmt!(StorageError, "expired block height of item not find")
            return Ok(None)
        }
        let exphei = Uint5::must(&bts);
        if exphei < self.hei {
            storage_map_err!(self.sto.del(&k)); // delete
            return Ok(None) // expired delete
        }
        // ok find
        return Ok(Some((bts[5..].to_vec(), *exphei)))
    }






}

