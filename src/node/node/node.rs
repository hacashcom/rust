


pub struct HacashNode {
    cnf: NodeConf,
    engine: Arc<BlockEngine>,
    txpool: Arc<MemTxPool>,
    p2p: Arc<P2PManage>,
    
    tokiort: Option<TokioRuntime>,
    blktxch: Option<Receiver<BlockTxArrive>>,

    knows: Knowledge,
}


impl HacashNode {

    pub fn open(ini: &IniObj, engine: Arc<BlockEngine>) -> HacashNode {
        let mut cnf = NodeConf::new(ini);

        let (tx, rx): (Sender<BlockTxArrive>, Receiver<BlockTxArrive>) = mpsc::channel(4000);

        let txpool = Arc::new(MemTxPool::new(vec![5000, 100]));
        let msghdl = Arc::new(MsgHandler::new(tx.clone(), engine.clone(), txpool.clone()));
        let p2p = Arc::new(P2PManage::new(&cnf, msghdl.clone()));
        msghdl.set_peer_mng(Box::new(PeerMngInst::new(p2p.clone())));

        let rt = new_current_thread_tokio_rt();

        HacashNode{
            cnf: cnf,
            engine: engine,
            txpool: txpool.clone(),
            p2p: p2p,
            tokiort: rt.into(),
            blktxch: rx.into(),
            knows: Knowledge::new(100),
        }
    }

}