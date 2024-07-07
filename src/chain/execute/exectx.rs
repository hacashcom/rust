
/**
 */
pub fn exec_tx_actions(is_fast_sync: bool, pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead) -> RetErr {
    // create exec env

    let fee_zhu = tx.fee().to_zhu_unsafe() as i64;
    let txsz = tx.size() as i64;
    let gas_price = fee_zhu / txsz;
    let gas = 1000000i64;
    // let extcaller = vm::interpreter::TestExtActCaller::new();
    // let outstorer = vm::interpreter::TestOutStorager::new();
    // let t1 = Box::new(extcaller);
    // let t2 = Box::new(outstorer);
    let mut ctx = ExecEnvObj::new(pending_height, tx);
    let ctxptr: *mut ExecEnvObj = &mut ctx;
    // create env
    let mut extcaller = ExecCaller::new(ctxptr, bst, sto);
    let callptr1: *mut ExecCaller = &mut extcaller;
    let callptr2 = callptr1;
    let callptr3 = callptr1;

    // let t1 = Box::new(ExtActCallerOutStorager::new(callptr1));
    // let t2 = Box::new(ExtActCallerOutStorager::new(callptr2));
    let t1 = unsafe{ &mut *callptr1 };
    let t2 = unsafe{ &mut *callptr2 };
    let t3 = unsafe{ &mut *callptr3 };
    let mut vm = vm::boot_machine( gas, t1, t2, t3);

    // ptr
    ctx.pdhash = pending_hash;
    ctx.fastsync = is_fast_sync;

    ctx.vmobj = Some(&mut vm);
    // ctx.outstorer = Some(callptr2);

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

