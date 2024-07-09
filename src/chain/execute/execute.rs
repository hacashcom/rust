
/**
 */
 pub fn exec_tx_actions(is_fast_sync: bool, pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead) -> RetErr {
    // exec by vm
    let gas_mult = (tx.gas_max() as u32).pow(3) as i64;
    let fee_zhu = tx.fee().to_shuo_unsafe() as i64;
    let fee_check = fee_zhu * gas_mult;
    let feeadr = tx.address()?;
    if fee_check > 0 {
        let amt = Amount::from_shuo(fee_check)?;
        let mut state = CoreState::wrap(bst);
        operate::hac_sub(&mut state, &feeadr, &amt)?;
        // gas prepayments
    }

    let txsz = tx.size() as i64;
    let gas_price = fee_zhu / txsz;
    if gas_price <= 0 {
        return Err(format!("tx fee {} too low to calculate gas price", tx.fee()))
    }
    let gas_limit = gas_mult* txsz;

    // let extcaller = vm::interpreter::TestExtActCaller::new();
    // let outstorer = vm::interpreter::TestOutStorager::new();
    // let t1 = Box::new(extcaller);
    // let t2 = Box::new(outstorer);
    let mut ctx = ExecEnvObj::new(pending_height, tx);
    // ptr
    ctx.pdhash = pending_hash;
    ctx.fastsync = is_fast_sync;

    // ptr
    let ctxptr: *mut ExecEnvObj = &mut ctx;
    // create env
    let mut extcaller = ExecCaller::new(ctxptr, bst, sto);
    let callptr: *mut ExecCaller = &mut extcaller;

    // let t1 = Box::new(ExtActCallerOutStorager::new(callptr1));
    // let t2 = Box::new(ExtActCallerOutStorager::new(callptr2));
    let t1 = unsafe{ &mut *callptr };
    let t2 = unsafe{ &mut *callptr };
    let t3 = unsafe{ &mut *callptr };
    let mut machine = vm::boot_vm( gas_limit, t1, t2, t3);
    // set ctx
    let vmptr: *mut vm::machine::Machine = &mut machine;
    ctx.vmobj = Some( unsafe{ &mut *vmptr });
    // ctx.outstorer = Some(callptr2);
    // execute action, ignore coinbase tx
    let exlist = tx.actions();
    // exec
    for act in exlist {
        extcaller.exec(act.as_ref())?;
        // ignore return value
    }
    // gas refund
    let gas_refund = machine.gas_refund();
    vm::shut_vm( machine ); // reuse resource
    if gas_refund > 0 {
        if gas_refund > gas_limit {
            return Err(format!("gas refund {} cannot more than gas limit {}", gas_refund, gas_limit))
        }
        let fee_shuo = gas_price * gas_refund;
        let amt = Amount::from_shuo(fee_shuo)?;
        let mut state = CoreState::wrap(bst);
        operate::hac_add(&mut state, &feeadr, &amt)?;
        // gas refund
    }
    // ok finish successfully
    Ok(())
}

