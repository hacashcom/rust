


pub struct ExecEnvObj<'a> {
    fastsync: bool,
    pdhei: u64,
    pdhash: Hash,
    mainaddr: Address,
    tx: &'a dyn TransactionRead,
    // extcaller: Option<*mut ExecCaller<'a>>,
    // outstorer: Option<*mut ExecCaller<'a>>,
    // vm
    vmobj: Option<&'a mut dyn VMIvk>,
}


impl ExecEnvObj<'_> {

    pub fn new<'a>(
        pdhei: u64, 
        tx: &'a dyn TransactionRead,
    ) -> ExecEnvObj<'a> {

        ExecEnvObj {
            fastsync: false,
            pdhei: pdhei,
            pdhash: Hash::default(),
            mainaddr: tx.address().unwrap(),
            tx,
            // extcaller: None,
            // outstorer: None,
            vmobj: None,
        }
    }


}


impl ExecContext for ExecEnvObj<'_> {

    fn pending_height(&self) -> u64 {
        self.pdhei
    }
    fn pending_hash(&self) -> &Hash {
        &self.pdhash
    }
    fn tx_fee(&self) -> &Amount {
        self.tx.fee()
    }
    fn main_address(&self) -> &Address {
        &self.mainaddr
    }
    fn addr_list(&self) -> &AddrOrList {
        &self.tx.addrlist()
    }
    fn check_signature(&self, adr: &Address) -> RetErr {
        transaction::verify_target_signature(adr, self.tx)
    }
    fn call_depth(&self) -> u32 {
        0
    }
    fn fast_sync(&self) -> bool {
        self.fastsync
    }
    fn vm(&mut self) -> &mut dyn VMIvk {
        *self.vmobj.as_mut().unwrap()
    }
}


/****************************************************/


pub struct ExecCaller<'a> {
    ctx: *mut ExecEnvObj<'a>,
    bst: &'a mut dyn State, 
    sto: &'a dyn Store, 
}

impl ExecCaller<'_> {

    pub fn new<'a>(
        ctx: *mut ExecEnvObj<'a>,
        bst: &'a mut dyn State, 
        sto: &'a dyn Store, 
    ) -> ExecCaller<'a> {

        ExecCaller {
            ctx,
            bst, 
            sto, 
        }
    }

    fn exec(&mut self, act: &dyn Action) -> Ret<(i64, Vec<u8>)> {
        unsafe { act.execute(&mut *self.ctx, self.bst, self.sto) }
    }

}


impl ExtActCaller for ExecCaller<'_> {

    fn call(&mut self, kind_and_body: Vec<u8>) -> Ret<(i64, Vec<u8>)> {
        let (act, sk) = action::create(&kind_and_body)?;
        if sk != kind_and_body.len() {
            return Err("action data length error".to_owned())
        }
        self.exec(act.as_ref())
    }
}


impl OutStoragerRead for ExecCaller<'_> {
    fn get(&self, key: &[u8]) -> Ret<Option<Vec<u8>>> {
        Ok( self.bst.get_at(key).map(|d|d.as_ref().to_vec()) )
    }
}


impl OutStorager for ExecCaller<'_> {
    fn del(&mut self, key: &[u8]) -> RetErr {
        self.bst.del_at(key);
        Ok(())
    }
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> RetErr {
        self.bst.set_at(key, value);
        Ok(())
    }
}


impl OutContext for ExecCaller<'_> { }


