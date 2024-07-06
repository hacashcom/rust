
/**
 */
pub fn exec_tx_actions(is_fast_sync: bool, pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead) -> RetErr {
    // create exec env
    // let extcaller = &mut vm::interpreter::TestExtActCaller::new();
    // let outstorer = &mut vm::interpreter::TestOutStorager::new();

    let mut ctx = ExecEnvObj::new(pending_height, tx);
    let ctxptr: *mut ExecEnvObj = &mut ctx;
    // create env
    let mut extcaller = ExecCaller::new(ctxptr, bst, sto);
    let callptr1: *mut ExecCaller = &mut extcaller;
    let callptr2 = callptr1;
    // ptr
    ctx.pdhash = pending_hash;
    ctx.fastsync = is_fast_sync;

    ctx.extcaller = Some(callptr1);
    ctx.outstorer = Some(callptr2);

    // ignore coinbase tx
    let exlist = tx.actions();
    // exec
    for act in exlist {
        extcaller.exec(act.as_ref())?;
        // ignore return value
    }
    // ok finish successfully
    Ok(())
}

