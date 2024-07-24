
/**
*
*/
pub fn exec_tx_actions(is_fast_sync: bool, 
    pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead,
) -> RetErr {

    let (a,b,c,d,e,f) = (is_fast_sync, pending_height, pending_hash, bst, sto, tx);
    let no_need_vm = tx.ty() < transaction::TX_TYPE_3 || tx.gas_max() <= 0;

    match no_need_vm {
        true  => exec_tx_actions_normal(a,b,c,d,e,f),
        false => exec_tx_actions_withvm(a,b,c,d,e,f),
    }
}


////////////////////////////////////////////////////////////////////


fn exec_tx_actions_normal(is_fast_sync: bool, 
    pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead,
) -> RetErr {

    // context & env
    let mut ctx = ExecEnvObj::new(pending_height, tx);
    // ptr
    ctx.pdhash = pending_hash;
    ctx.fastsync = is_fast_sync;
    let ctxptr: *mut ExecEnvObj = &mut ctx;

    // create env
    let mut extcaller = ExecCaller::new(ctxptr, bst, sto);

    // exec not vm
    let exlist = tx.actions();
    let call_depth = -1i8;
    for act in exlist {
        extcaller.exec(act.as_ref(), call_depth)?;
        // ignore return value
    }
    
    Ok(())
}



fn exec_tx_actions_withvm(is_fast_sync: bool, 
    pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead,
) -> RetErr {

    // exec by vm
    let gas_mult = (tx.gas_max() as u32).pow(3) as i64;
    let fee_shuo = tx.fee().to_shuo_unsafe() as i64;
    let fee_check = fee_shuo * gas_mult;
    let feeadr = tx.address()?;
    if fee_check > 0 {
        let amt = Amount::from_shuo(fee_check)?;
        let mut state = CoreState::wrap(bst);
        operate::hac_sub(&mut state, &feeadr, &amt)?;
        // gas prepayments
    }

    let txsz = tx.size() as i64;
    let gas_price = fee_shuo / txsz;
    if gas_price <= 0 {
        return errf!("tx fee {} too low to calculate gas price", tx.fee())
    }
    let gas_limit = gas_mult* txsz;

    // context & env
    let mut ctx = ExecEnvObj::new(pending_height, tx);
    // ptr
    ctx.pdhash = pending_hash;
    ctx.fastsync = is_fast_sync;
    let ctxptr: *mut ExecEnvObj = &mut ctx;
    // create env
    let mut extcaller = ExecCaller::new(ctxptr, bst, sto);
    let callptr: *mut ExecCaller = &mut extcaller;
    let t1 = unsafe{ &mut *callptr };
    let t2 = unsafe{ &mut *callptr };
    let t3 = unsafe{ &mut *callptr };
    let mut machine = vm::boot_vm( pending_height, gas_limit, t1, t2, t3);
    // set ctx
    let vmptr: *mut vm::machine::Machine = &mut machine;
    ctx.vmobj = Some( unsafe{ &mut *vmptr });
    
    // execute action, ignore coinbase tx
    let exlist = tx.actions();
    let call_depth = -1i8;
    // exec
    for act in exlist {
        extcaller.exec(act.as_ref(), call_depth)?;
        // ignore return value
    }

    // gas refund
    let gas_refund = machine.gas_refund();
    vm::shut_vm( machine ); // reuse resource
    if gas_refund > 0 {
        if gas_refund > gas_limit {
            return errf!("gas refund {} cannot more than gas limit {}", gas_refund, gas_limit)
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

