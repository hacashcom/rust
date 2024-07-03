
/**
 * do insert block crate new state
 * return new state
 */
pub fn exec_tx_actions(is_fast_sync: bool, pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead) -> RetErr {
    // create exec env
    let mut env = ExecEnvObj::new(pending_height, tx);
    env.pdhash = pending_hash;
    env.fastsync = is_fast_sync;
    // ignore coinbase tx
    let exlist = tx.actions();
    // exec
    for act in exlist {
        let res = act.execute(&mut env, bst, sto);
        if let Some(abort_err) = res.abort() {
            return Err(abort_err.clone()) // abort error
        }
    }
    // ok finish
    Ok(())
}

