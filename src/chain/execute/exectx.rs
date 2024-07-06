
/**
 */
pub fn exec_tx_actions(is_fast_sync: bool, pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead) -> RetErr {
    // create exec env
    // let extcaller = &mut vm::interpreter::TestExtActCaller::new();
    // let outstorer = &mut vm::interpreter::TestOutStorager::new();

    let mut ctx = ExecEnvObj::new(pending_height, tx);
    ctx.pdhash = pending_hash;
    ctx.fastsync = is_fast_sync;

    let mut extcaller = ExecCaller::new(&mut ctx, bst, sto);
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

