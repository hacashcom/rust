




/**
 * Channel Open
 */
 ActionDefine!{
    ChannelOpen : 2, (
        channel_id     : ChannelId
        left_bill      : AddrHac
        right_bill     : AddrHac
    ),
    ACTLV_TOP_ONLY, // level
    16 + (21+11)*2, // gas
    (self, env, state, store), // params
    false, // burn 90
    [
        self.left_bill.address,
        self.right_bill.address
    ], // req sign
    ActExecRes::wrap(channel_open(self, env, state, store))
}

fn channel_open(this: &ChannelOpen, env: &dyn ExecEnv, sta: &mut dyn State, sto: &dyn Store) -> RetErr {

    let (cid, left_addr, left_amt, right_addr, right_amt ) = (
        &this.channel_id,
        &this.left_bill.address,
        &this.left_bill.amount,
        &this.right_bill.address,
        &this.right_bill.amount
    );


    // sub balance
    let mut core_state = CoreState::wrap(sta);
    if left_amt.is_not_empty() {
        hac_sub(&mut core_state, left_addr,  left_amt)?;
    }
    if right_amt.is_not_empty() {
        hac_sub(&mut core_state, right_addr, right_amt)?;
    }
    drop(core_state);

    // 
    let mut state = MintState::wrap(sta);
    let store = MintStoreDisk::wrap(sto);

    // check id size
    check_vaild_store_item_key("channel", &cid, ChannelId::width())?;

    // check format
    if left_addr == right_addr {
        return errf!("left address cannot equal with right address")
    }
    if left_amt.size() > 6 || right_amt.size() > 6 {
        return errf!("left or right amount bytes too long")
    }
    if left_amt.is_negative() || right_amt.is_negative() ||
        (left_amt.is_empty() && right_amt.is_empty()) {
        return errf!("left or right amount is not positive or two both is empty")
    }

    // check exist
    let mut reuse_version = Uint4::from(1);
	// channel ID with the same left and right addresses and closed by consensus can be reused
    let havcha = state.channel(cid);
    if let Some(chan) = havcha {
        let chan_stat = chan.status;
        let samebothaddr = *left_addr==chan.left_bill.address && *right_addr == chan.right_bill.address;
        if !samebothaddr || CHANNEL_STATUS_AGREEMENT_CLOSED != chan_stat {
            // exist or cannot reuse
            return errf!("channel {} is openning or cannot reuse.", cid)
        }
        reuse_version = chan.reuse_version.clone();
        reuse_version += 1u64;
    }

    // save channel
    let pd_hei = env.pending_height();
    let channel = ChannelSto{
        status: CHANNEL_STATUS_OPENING,
        reuse_version: reuse_version,
        belong_height: Uint5::from(pd_hei),
        arbitration_lock_block: Uint2::from(5000), // lock period is about 17 days
        interest_attribution: CHANNEL_INTEREST_ATTRIBUTION_TYPE_DEFAULT,
        left_bill: AddrHacSat{
            address: left_addr.clone(),
            hacsat: HacSat{amount: left_amt.clone(), satoshi: SatoshiOptional::new()}},
        right_bill: AddrHacSat{
            address: right_addr.clone(),
            hacsat: HacSat{amount: right_amt.clone(), satoshi: SatoshiOptional::new()}},
        if_challenging: ChallengePeriodDataOptional::new(), // none
        if_distribution: ClosedDistributionDataOptional::new(), // none
    };
    state.set_channel(cid, &channel);

    // update total count
    let mut ttcount = state.total_count();
    ttcount.opening_channel += 1u64;
    ttcount.channel_deposit_zhu += left_amt.add(right_amt) ? .to_zhu_unsafe() as u64;
    state.set_total_count(&ttcount);

    // ok finish
    Ok(())
}





/**
 * Channel Close
 */
 ActionDefine!{
    ChannelClose : 3, (
        channel_id     : ChannelId
    ),
    ACTLV_TOP_ONLY, // level
    16, // gas
    (self, env, state, store), // params
    false, // burn 90
    [], // req sign
    ActExecRes::wrap(channel_close(self, env, state, store))
}

fn channel_close(this: &ChannelClose, env: &dyn ExecEnv, sta: &mut dyn State, sto: &dyn Store) -> RetErr {

    let mut state = MintState::wrap(sta);

    let cid = &this.channel_id;
    check_vaild_store_item_key("channel", cid, ChannelId::width())?;
    // query
    let chan = must_have!("channel", state.channel(cid));
	// verify two address sign
    env.check_signature( &chan.left_bill.address )?;
    env.check_signature( &chan.right_bill.address )?;
    drop(state);
    // do close
    close_channel_default( env.pending_height(), sta, cid, &chan)
}