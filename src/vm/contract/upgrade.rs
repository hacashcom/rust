


StructFieldStruct!{ ContractUpgrade,
    librarys: ContractAddrsssList
    sytmcalls: ContractSystemCallList
	userfuncs: ContractClientFuncList
}




impl ContractStorage {


    /*
    * return [(is_change_or_append, fnsign(1|4))]
    */
    pub fn upgrade(&mut self, cu: &ContractUpgrade) -> Ret<Vec<Vec<u8>>> {
        let l1 = &self.contlhead.librarys;
        let l2 = &cu.librarys;
        let n1 = l1.count().value() as usize;
        let n2 = l2.count().value() as usize;
        if n1 + n2 > 255 {
            return errf!("contract librarys list overflow")
        }
        // append librarys
        self.contlhead.librarys.append(l2.list().clone());
        // check sytmcalls
        let mut upparas = vec![];
        for s in cu.sytmcalls.list() {
            let ischange = match self.find_syscall(&s.sign) {
                Some(_) => 1, // change
                _ => 0, // append
            };
            let f = s.sign[0];
            let para = vec![ischange,f];
            upparas.push(para);
        }
        // check userfuncs
        for s in cu.userfuncs.list() {
            let ischange = match self.find_usrfunc(&s.sign) {
                Some(_) => 1, // change
                _ => 0, // append
            };
            let f = s.sign;
            let para = vec![ischange, f[0],f[1],f[2],f[3]];
            upparas.push(para);
        }

        Ok(upparas)
    }


}
