

pub struct BlockChainKernel {

    cnf: KernelConf,

    store: Arc<BlockStore>,
    state: Weak<ChainState>,

    sroot: Arc<ChunkRoller>, // tree root block
    scusp: Weak<ChunkRoller>, // current latest block

    // insert lock
    isrlck: Mutex<bool>,
    // updlck: RwLock<bool>,
}

impl BlockChainKernel {


    pub fn init(&mut self, ini: &IniObj) -> Option<Error> {
        let cnf = NewKernelConf(ini);
        self.cnf = cnf;
        None
    }

    pub fn get_latest_state(&self) -> Arc<dyn State> {
        if let Some(st) = self.state.upgrade() {
            return st
        }
        if let Some(sc) = self.scusp.upgrade() {
            return sc.state.clone()
        }
        // base
        self.sroot.state.clone()
    }
}





