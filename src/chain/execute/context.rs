


pub struct ExecEnvObj<'a> {
    fastsync: bool,
    pdhei: u64,
    pdhash: Hash,
    mainaddr: Address,
    tx: &'a dyn TransactionRead,
    // extcaller: &mut dyn ExtActCaller,
    // outstorer: &mut dyn OutStorager,
    // vm
    vmobj: Option<Box<dyn VMIvk>>,
}


impl ExecEnvObj<'_> {

    pub fn new<'a>(
        pdhei: u64, 
        tx: &'a dyn TransactionRead,
        // bst: &'a mut dyn State, 
        // sto: &'a dyn Store, 
        // extcaller: &mut dyn ExtActCaller,
        // outstorer: &mut dyn OutStorager,
    ) -> ExecEnvObj<'a> {

        ExecEnvObj {
            fastsync: false,
            pdhei: pdhei,
            pdhash: Hash::default(),
            mainaddr: tx.address().unwrap(),
            tx,
            // bst,
            // sto,
            // extcaller,
            // outstorer,
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
    fn vm(&mut self) -> &mut dyn VMIvk{
        if let None = self.vmobj {
            let fee_zhu = self.tx_fee().to_zhu_unsafe() as i64;
            let txsz = self.tx.size() as i64;
            let gas_price = fee_zhu / txsz;
            let gas = 1000000i64;

            let extcaller = vm::interpreter::TestExtActCaller::new();
            let outstorer = vm::interpreter::TestOutStorager::new();
            
            let vm = vm::machine::Machine::new( gas, Box::new(extcaller), Box::new(outstorer) );
            self.vmobj = Some(Box::new(vm));
        }
        self.vmobj.as_mut().unwrap().as_mut()
    }
}


/****************************************************/


pub struct ExecCaller<'a> {
    ctx: &'a mut ExecEnvObj<'a>,
    bst: &'a mut dyn State, 
    sto: &'a dyn Store, 
}

impl ExecCaller<'_> {

    pub fn new<'a>(
        ctx: &'a mut ExecEnvObj<'a>,
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
        act.execute(self.ctx, self.bst, self.sto)
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
