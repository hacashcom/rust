


pub struct ExecEnvObj<'a> {
    fastsync: bool,
    pdhei: u64,
    pdhash: Hash,
    mainaddr: Address,
    tx: &'a dyn TransactionRead,
    extcaller: Option<*mut ExecCaller<'a>>,
    outstorer: Option<*mut ExecCaller<'a>>,
    // vm
    vmobj: Option<Box<dyn VMIvk>>,
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
            extcaller: None,
            outstorer: None,
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
    fn vm<'a>(&'a mut self) -> &'a mut dyn VMIvk{
        if let None = self.vmobj {

            let fee_zhu = self.tx_fee().to_zhu_unsafe() as i64;
            let txsz = self.tx.size() as i64;
            let gas_price = fee_zhu / txsz;
            let gas = 1000000i64;
            let extcaller = vm::interpreter::TestExtActCaller::new();
            let outstorer = vm::interpreter::TestOutStorager::new();
            let t1 = Box::new(extcaller);
            let t2 = Box::new(outstorer);
            // let t1 = Box::new(ExtActCallerOutStorager::new(self.extcaller.take().unwrap()));
            // let t2 = Box::new(ExtActCallerOutStorager::new(self.extcaller.take().unwrap()));
            let mut vm = vm::machine::Machine::new( gas, t1, t2);
            self.vmobj = Some(Box::new(vm));
        }
        self.vmobj.as_mut().unwrap().as_mut()
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


impl OutStorager for ExecCaller<'_> {
    fn get(&self, key: &[u8]) -> Ret<Option<Vec<u8>>> {
        Ok(Some(vec![1,0,0,1]))
    }
    fn del(&mut self, key: &[u8]) -> RetErr {
        Ok(())
    }
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> RetErr {
        Ok(())
    }
}


// 


pub struct ExtActCallerOutStorager<'a> {
    pub wrap: *mut ExecCaller<'a>,
}

impl ExtActCallerOutStorager<'_> {
    pub fn new(w: *mut ExecCaller) -> ExtActCallerOutStorager {
        ExtActCallerOutStorager {
            wrap: w,
        }
    }
}

impl ExtActCaller for ExtActCallerOutStorager<'_> {

    fn call(&mut self, kind_and_body: Vec<u8>) -> Ret<(i64, Vec<u8>)> {
        unsafe{ (*self.wrap).call(kind_and_body) }
    }
}


impl OutStorager for ExtActCallerOutStorager<'_> {

    fn get(&self, key: &[u8]) -> Ret<Option<Vec<u8>>> {
        Ok(Some(vec![1,0,0,1]))
    }
    fn del(&mut self, key: &[u8]) -> RetErr {
        Ok(())
    }
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> RetErr {
        Ok(())
    }

}
