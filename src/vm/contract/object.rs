


// impl
impl_contract_function!{ContractSystemCall}
impl_contract_function!{ContractClientFunc}



impl ContractStorage {

    pub fn libaddr(&self, idx: u8) -> VmrtRes<ContractAddress> {
        let idx: usize = idx.into();
        let ary = self.contlhead.librarys.list();
        if idx >= ary.len() {
            return itr_err_fmt!(CallInvalid, "cannot find lib idx {}", idx)
        }
        Ok(address_to_contract(&ary[idx]))
    }

    pub fn librarys(&self) -> Vec<ContractAddress> {
        let ary = self.contlhead.librarys.list();
        let mut res = Vec::with_capacity(ary.len());
        for a in ary {
            res.push(address_to_contract(a))
        }
        res
    }

    pub fn inherits(&self) -> Vec<ContractAddress> {
        let ary = self.contlhead.inherits.list();
        let mut res = Vec::with_capacity(ary.len());
        for a in ary {
            res.push(address_to_contract(a))
        }
        res
    }

    // return bytecodes
    pub fn check_usrfunc(&self, fnsign: &FnSign) -> bool {
        let ary = self.userfuncs.list();
        for a in ary {
            if fnsign == a.sign.as_ref() {
                return true
            }
        }
        // not find
        false
    }

    // return bytecodes
    pub fn load_usrfunc(&self, fnsign: &FnSign) -> VmrtRes<Vec<u8>> {
        let ary = self.userfuncs.list();
        for a in ary {
            if fnsign == a.sign.as_ref() {
                return compile_to_bytecodes(a)
            }
        }
        // not find
        itr_err_fmt!(CallInvalid, "cannot find function by sign {}", &hex::encode(fnsign))
    }

    // return bytecodes
    pub fn load_syscall(&self, fnty: SystemCallType) -> VmrtRes<Vec<u8>> {
        let fnty: u8 = fnty as u8;
        let ary = self.sytmcalls.list();
        for a in ary {
            if fnty == a.sign[0] {
                return compile_to_bytecodes(a)
            }
        }
        // not find
        itr_err_fmt!(CallInvalid, "cannot find syscall by type {:?}", fnty)
    }


}
