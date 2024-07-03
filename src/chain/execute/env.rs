
pub struct ExecEnvObj<'a> {
    fastsync: bool,
    pdhei: u64,
    pdhash: Hash,
    mainaddr: Address,
    tx: &'a dyn TransactionRead,
    vmobj: Option<Box<dyn VMIvk>>,
}


impl ExecEnvObj<'_> {
    pub fn new<'a>(pdhei: u64, tx: &'a dyn TransactionRead) -> ExecEnvObj {
        ExecEnvObj{
            fastsync: false,
            pdhei: pdhei,
            pdhash: Hash::default(),
            mainaddr: tx.address().clone(),
            tx: tx,
            vmobj: None,
        }
    }
}


impl ExecEnv for ExecEnvObj<'_> {

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
    fn check_signature(&self, adr: &Address) -> RetErr {
        transaction::verify_target_signature(adr, self.tx)
    }
    fn call_depth(&self) -> u32 {
        0
    }
    fn fast_sync(&self) -> bool {
        self.fastsync
    }
    fn vm_main_call(&mut self, entry: &Address, irs: &[u8]) -> Ret<Vec<u8>> {
        if let None = self.vmobj {
            let gas = 1000000i64;
            let extcaller = vm::interpreter::TestExtActCaller::new();
            let outstorer = vm::interpreter::TestOutStorager::new();
            self.vmobj = Some(Box::new(vm::machine::Machine::new(
                gas, Arc::new(extcaller), Arc::new(outstorer)
            )));
        }
        // call
        self.vmobj.as_mut().unwrap().main_call(entry, irs)
    }
}
