

pub fn contract_store_key(addr: &ContractAddress) -> Vec<u8> {
    // hvm-contract-code-object-storage-key
    vec![addr.to_vec(), b"hvmccosk".to_vec()].concat()
}


impl Machine {

    fn load_contract(&mut self, addr: &ContractAddress) -> VmrtRes<&ContractStorage> {
        // check cache
        let mut cache = &mut self.contract_cache;
        if cache.contains_key(addr) {
            return Ok(&cache[addr])
        }
        // check cap limit
        let maxcn = self.space_cap.load_contract;
        if cache.len() >= maxcn {
            return itr_err_fmt!(OutOfLoadContract, "max contract number be loaded in one tx is {}", maxcn)
        }
        let ctky = contract_store_key(addr);
        match self.out_storage.get(&ctky).map_err(
            |e|ItrErr::new(OutStorageError, &format!("{}", &e))
        )? {
            Some(con) => {
                let obj = ContractStorage::build(&con).map_err(|e|
                    ItrErr::new(ContractError, &format!("contract {} parse error {}", 
                    contract_to_address(addr).readable(), &e)))?;
                // set cache
                cache.insert(*addr, obj);
                Ok(&cache[addr])
            },
            _ => itr_err_fmt!(ContractError, "contract {} not find", contract_to_address(addr).readable()),
        }
    }

    fn load_codes_by_syscall(&mut self, taradr: &ContractAddress, syscall: SystemCallType) 
        -> VmrtRes<Vec<u8>>
    {
        let target_contract = self.load_contract(taradr)?;
        target_contract.load_syscall(syscall)
    }


    fn load_codes_by_funcptr(&mut self, ctxadr: &ContractAddress, ivkadr: &ContractAddress, fnptr: &Funcptr) 
        -> VmrtRes<(ContractAddress, Vec<u8>)> 
    {

        use CallMode::*;
        use CallTarget::*;


        let mut target_addr = ivkadr.clone();
        let fnsign = fnptr.fnsign;
        let fmode = fnptr.mode;

        macro_rules! gettar {
            ($T: ident) => {
                match fnptr.target {
                    $T(x) => x.clone(),
                    _ => return itr_err_code!(ContractError),
                }
            }
        }

        if let External = fmode {
            target_addr = gettar!(Addr);
        }else if let Library | Static | Code = fmode {
            let ctobj = self.load_contract(ivkadr)?;
            target_addr = ctobj.libaddr( gettar!(Libidx) )?;
        }else if let InheritLoc = fmode {
            let ctxcont = self.load_contract(ctxadr)?;
            let mut adrlist = ctxcont.inherits();
            adrlist.insert(0, ctxadr.clone());
            // do search in inherits
            for adr in adrlist {
                target_addr = adr;
                let ctobj = self.load_contract(&target_addr)?;
                if ctobj.check_usrfunc(&fnsign) {
                    break // okn  search exist
                }
            }
        }else {
            // not support Main | System
            return itr_err_code!(ContractError)
        }
        // read func
        let target_contract = self.load_contract(&target_addr)?;
        let resfn = target_contract.load_usrfunc(&fnsign)?;
        Ok((target_addr, resfn))
    }

}



