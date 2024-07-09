
const FN_KEY_WIDTH: usize = 21 + 1 + 4; // Address + t + (systy|fnsign)

type FnKey = [u8; FN_KEY_WIDTH];


struct InheritsAndLibs {
    inherits: Vec<ContractAddress>,
    librarys: Vec<ContractAddress>,
}


impl InheritsAndLibs {

    pub fn libaddr(&self, idx: u8) -> VmrtRes<ContractAddress> {
        let idx: usize = idx.into();
        let ary = self.librarys();
        if idx >= ary.len() {
            return itr_err_fmt!(CallInvalid, "cannot find lib idx {}", idx)
        }
        Ok(ary[idx].clone())
    }

    pub fn inherits(&self) -> &Vec<ContractAddress> {
        &self.inherits
    }

    pub fn librarys(&self) -> &Vec<ContractAddress> {
        &self.librarys
    }

}



/****************************************/

pub struct ContractLoader {
    max_func: usize, // 200 cache
    max_libx: usize, // 100 cache
    max_body: usize, // 50 cache
    // func
    cache_funcary: VecDeque<FnKey>,
    cache_funcs: HashMap<FnKey, Vec<u8>>, // func => codes
    // libx
    cache_libxary: VecDeque<ContractAddress>,
    cache_libxs: HashMap<ContractAddress, InheritsAndLibs>,
    // body
    cache_bodyary: VecDeque<ContractAddress>,
    cache_bodys: HashMap<ContractAddress, ContractStorage>,
}


impl ContractLoader {

    pub fn new(cap: &SpaceCap) -> ContractLoader {
        let mf = cap.max_ctl_func; // 200
        let ml = cap.max_ctl_libx; // 100
        let mb = cap.max_ctl_body; // 50
        
        ContractLoader{
            max_func: mf,
            max_libx: ml,
            max_body: mb,
            cache_funcary: VecDeque::with_capacity(mf),
            cache_funcs: HashMap::with_capacity(mf), // func => codes
            cache_libxary: VecDeque::with_capacity(ml),
            cache_libxs: HashMap::with_capacity(ml), // addr => libx
            cache_bodyary: VecDeque::with_capacity(mb),
            cache_bodys: HashMap::with_capacity(mb), // addr => bodys
        }

    }

    pub fn remove_upgraded(&mut self, addr: &ContractAddress) {
        self.cache_bodys.remove(addr);
        self.cache_libxs.remove(addr);
        self.cache_bodyary.retain(|x|x!=addr);
        self.cache_libxary.retain(|x|x!=addr);
        self.cache_funcary.retain(|x|&x[0..21]!=addr);
        self.cache_funcs.retain(|x,_|&x[0..21]!=addr);
    }

    
}


impl ContractLoader {

    /**
    * return (target_addr, codes)
    */
    pub fn load_by_funcptr(&mut self, 
        out_storage: &mut dyn OutStoragerRead, 
        ctxadr: &ContractAddress, 
        ivkadr: &ContractAddress, 
        fnptr: &Funcptr,
    ) -> VmrtRes<(ContractAddress, &[u8])> {
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
        if let InheritLoc = fmode {

            let mut adrlist = self.load_inhlibs(out_storage, ctxadr)?.inherits().clone();
            adrlist.insert(0, ctxadr.clone());
            // do search in inherits
            for adr in adrlist {
                target_addr = adr;
                if self.check_usrfunc_cache(&target_addr, &fnsign) {
                    break // ok find
                }
                let ctobj = self.load_contract(out_storage, &target_addr)?;
                if ctobj.check_usrfunc(&fnsign) {
                    break // ok search exist
                }
            }

        } else if let External = fmode {
            target_addr = gettar!(Addr);
        } else if let Library | Static | Code = fmode {
            target_addr = self.load_inhlibs(out_storage, ivkadr)?.libaddr( gettar!(Libidx) )?;
        } else {
            // not support Main | System
            return itr_err_code!(ContractError)
        }
        // read func
        let funcodes = self.load_usrfunc(out_storage, &target_addr, fnsign)?;
        Ok((target_addr, &funcodes))
    }



    /**
    * load_usrfunc
    */
    pub fn load_usrfunc(&mut self, 
        out_storage: &mut dyn OutStoragerRead, 
        addr: &ContractAddress, 
        fnsign: FnSign,
    ) -> VmrtRes<&[u8]> {
        self.load_func_codes(out_storage, addr, &FnKeyObj::Usr(fnsign))
    }

    /**
    * load_syscall
    */
    pub fn load_syscall(&mut self, 
        out_storage: &mut dyn OutStoragerRead, 
        addr: &ContractAddress, 
        syscall: SystemCallType,
    ) -> VmrtRes<&[u8]> {
        self.load_func_codes(out_storage, addr, &FnKeyObj::Sys(syscall))
    }

    /**
    * check_usrfunc_cache
    */
    fn check_usrfunc_cache(&mut self, 
        addr: &ContractAddress, 
        fnsign: &FnSign,
    ) -> bool {
        let mut fnkey = [0u8; FN_KEY_WIDTH];
        fnkey[0..21].copy_from_slice(addr);
        fnkey[21] = 1;
        fnkey[22..].copy_from_slice(fnsign);
        // search
        self.cache_funcs.contains_key(&fnkey)
    }

    fn load_func_codes(&mut self, 
        out_storage: &mut dyn OutStoragerRead, 
        addr: &ContractAddress, 
        fnk: &FnKeyObj,
    ) -> VmrtRes<&[u8]> {
        use FnKeyObj::*;
        let mut fnkey = [0u8; FN_KEY_WIDTH];
        fnkey[0..21].copy_from_slice(addr);
        fnkey[21] = match fnk {
            Sys(f) => {
                fnkey[22] = *f as u8;
                0 // sys
            },
            Usr(f) => {
                fnkey[22..].copy_from_slice(f);
                1 // usr
            },
        };
        // find or insert
        if self.cache_funcs.contains_key(&fnkey) {
            return Ok(&self.cache_funcs[&fnkey])
        }
        // clear cache
        if self.cache_funcary.len() > self.max_libx{
            if let Some(key) = self.cache_funcary.pop_back() {
                self.cache_funcs.remove(&key);
            };
        }
        // load
        let contract = self.load_contract(out_storage, addr)?;
        let value = match fnk {
            Sys(f) => contract.load_syscall(*f)?,
            Usr(f) => contract.load_usrfunc(f)?,
        };
        // ok
        self.cache_funcary.push_front(fnkey.clone());
        Ok(self.cache_funcs.entry(fnkey).or_insert(value))
    }
    

    fn load_inhlibs(&mut self, 
        out_storage: &mut dyn OutStoragerRead, 
        addr: &ContractAddress,
    ) -> VmrtRes<&InheritsAndLibs> {
        // find or insert
        if self.cache_libxs.contains_key(addr) {
            return Ok(&self.cache_libxs[addr])
        }
        // clear cache
        if self.cache_libxary.len() > self.max_libx{
            if let Some(key) = self.cache_libxary.pop_back() {
                self.cache_libxs.remove(&key);
            };
        }
        // load
        let contract = self.load_contract(out_storage, addr)?;
        let value = InheritsAndLibs {
            inherits: contract.inherits(),
            librarys: contract.librarys(),
        };
        // ok
        self.cache_libxary.push_front(*addr);
        Ok(self.cache_libxs.entry(*addr).or_insert(value))
    }


    fn load_contract(&mut self, 
        out_storage: &mut dyn OutStoragerRead, 
        addr: &ContractAddress,
    ) -> VmrtRes<&ContractStorage> {
        let cache = &mut self.cache_bodys;
        // clear cache
        if self.cache_bodyary.len() > self.max_body {
            if let Some(key) = self.cache_bodyary.pop_back() {
                cache.remove(&key);
            };
        }
        // find or insert
        let item = cache.entry(*addr);
        if let Occupied(hav) = item {
            return Ok(hav.into_mut())
        }
        // read or insert
        let ctky = contract_store_key(addr);
        let value = match out_storage.get(&ctky).map_err(
            |e|ItrErr::new(OutStorageError, &format!("{}", &e))
        )? {
            Some(con) => {
                let obj = ContractStorage::build(&con).map_err(|e|
                    ItrErr::new(ContractError, &format!("contract {} parse error {}", 
                    contract_to_address(addr).readable(), &e)))?;
                    obj
            },
            _ => return itr_err_fmt!(ContractError, "contract {} not find", contract_to_address(addr).readable()),
        };
        // ok
        self.cache_bodyary.push_front(*addr);
        Ok(item.or_insert(value))
    }

}











