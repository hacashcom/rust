

impl Machine<'_> {

    fn check_contract_count(&mut self, addr: &ContractAddress) -> VmrtErr {
        self.r.contract_count.insert(*addr);
        let maxcn = self.r.space_cap.load_contract;
        if self.r.contract_count.len() > maxcn {
            return itr_err_fmt!(OutOfLoadContract, "max contract number be loaded in one tx is {}", maxcn)
        }
        Ok(())
    }
      

}



