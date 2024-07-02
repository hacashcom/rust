
/**
 * do insert block crate new state
 * return new state
 */
pub fn exec_tx_actions(is_fast_sync: bool, pending_height: u64, pending_hash: Hash, vm: &dyn VM, bst: &mut dyn State, tx: &dyn TransactionRead) -> RetErr {
    // create exec env
    let mut env = ExecEnvObj::new(pending_height, tx);
    env.pdhash = pending_hash;
    env.fastsync = is_fast_sync;
    // ignore coinbase tx
    let exlist = tx.actions();
    // exec
    let exres = vm.exec(&env, bst, exlist)?;
    // ok
    Ok(())
}

