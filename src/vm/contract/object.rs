


// impl
impl_contract_function!{ContractSystemCall}
impl_contract_function!{ContractClientFunc}



impl ContractStorage {

    pub fn check_field_validity(&self, sc: &SpaceCap) -> RetErr {
        // head
        let h = &self.contlhead;
        if *h.vrsn != [0] 
        || *h.marks != [0,0,0,0,0] 
        || *h.inherits.count() > sc.inherit_contract
        || *h.mexts != [0,0] 
        || *self.morextend != [0,0] {
            return errf!("contract storage data format error")
        }
        // sytmcalls
        let sycls = self.sytmcalls.list();
        for a in sycls {
            if *a.mark != [0] 
            || *a.vrsn != [0]
            || *a.sign == [0]
            || a.code.length() == 0 {
                return errf!("contract sytmcalls data format error")
            }
            SystemCallType::check(a.sign[0])?;
        }
        // userfuncs
        let urfns = self.userfuncs.list();
        for a in urfns {
            if *a.mark != [0,0,0] 
            || *a.vrsn != [0]
            || *a.sign == [0,0,0,0] 
            || a.code.length() == 0 {
                return errf!("contract userfuncs data format error")
            }
            fn_sign_check(&a.sign)?;
        }
        // ok success
        Ok(())
    }


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

    fn find_syscall(&self, sg: &Fixed1) -> Option<usize> {
        let max = self.sytmcalls.list().len();
        for i in 0..max {
            if *sg == self.sytmcalls[i].sign {
                return Some(i)
            }
        }
        // not find
        None
    }

    fn find_usrfunc(&self, sg: &Fixed4) -> Option<usize> {
        let max = self.userfuncs.list().len();
        for i in 0..max {
            if *sg == self.userfuncs[i].sign {
                return Some(i)
            }
        }
        // not find
        None
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
