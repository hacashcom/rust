
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
            mainaddr: tx.address().unwrap(),
            tx: tx,
            vmobj: None,
        }
    }

    fn create_vm(&self) -> vm::machine::Machine {

        let fee_zhu = self.tx_fee().to_zhu_unsafe() as i64;
        let txsz = self.tx.size() as i64;
        let gas_price = fee_zhu / txsz;

        let gas = 1000000i64;
        let extcaller = vm::interpreter::TestExtActCaller::new();
        let outstorer = vm::interpreter::TestOutStorager::new();
        vm::machine::Machine::new(
            gas, Arc::new(extcaller), Arc::new(outstorer)
        )
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
        if let None = self.vmobj {
            let vm = self.create_vm();
            self.vmobj = Some(Box::new(vm));
        }
        self.vmobj.as_mut().unwrap().as_mut()
    }
}
