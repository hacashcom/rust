

macro_rules! fnSatoshiOperateCommon{
    ($func_name: ident, $addr:ident, $sat:ident, $oldsat:ident,  $newsatblock:block) => (

pub fn $func_name(state: &mut CoreState, $addr: &Address, $sat: &Satoshi) -> Ret<Satoshi> {
    if $sat.uint() == 0 {
		return errf!("satoshi value cannot zore")
    }    
    let mut userbls = state.balance( $addr ).unwrap_or_default();
    let $oldsat = &userbls.satoshi.to_satoshi();
    /* -------- */
    let newsat = $newsatblock;// operate
    /* -------- */
    // save
    userbls.satoshi = SatoshiAuto::from_satoshi( &newsat );
    state.set_balance($addr, &userbls);
    Ok(newsat)
}

    )
}


/**************************** */

fnSatoshiOperateCommon!(sat_add, addr, sat, oldsat, {
    // do add
    *oldsat + *sat 
});

fnSatoshiOperateCommon!(sat_sub, addr, sat, oldsat, {  
    // check
    if *oldsat < *sat {
		return errf!("do sat_sub error: address {} balance {} not enough, need {}", 
            addr.readable(), oldsat, sat)
    }
    // do sub
    *oldsat - *sat
});



/**************************** */


pub fn sat_transfer(state: &mut CoreState, addr_from: &Address, addr_to: &Address, sat: &Satoshi) -> Ret<Vec<u8>> {
    if addr_from == addr_to {
		return errf!("cannot trs to self")
    }
    sat_sub(state, addr_from, sat)?;
    sat_add(state, addr_to, sat)?;
    // ok
    Ok(vec![])
}


pub fn sat_check(state: &mut CoreState, addr: &Address, sat: &Satoshi) -> Ret<Satoshi> {
    if 0 == sat.uint() {
        return errf!("check satoshi is cannot empty")
    }
    if let Some(bls) = state.balance( addr ) {
        let usrsat = bls.satoshi.to_satoshi();
        if usrsat >= *sat {
            return Ok(usrsat)
        }
    }
    errf!("address {} satoshi not enough", addr.readable())
}






