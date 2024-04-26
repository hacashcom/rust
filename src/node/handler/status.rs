

impl MsgHandler {

    async fn send_status(&self, peer: Arc<Peer>) {
        // println!("+++ send_status_to_peer peer={}", peer.nick());
        let my_status = create_status(self);
        let msgbuf = my_status.serialize();
        peer.send_msg(MSG_STATUS, msgbuf).await;
    }

    async fn receive_status(&self, peer: Arc<Peer>, buf: Vec<u8>) {
        // println!(">>> receive_status_from_peer peer={}", peer.nick());
        let status = HandshakeStatus::create(&buf);
        if status.is_err() {
            peer.disconnect().await;
            return
        }
        let (status, _) = status.unwrap();
        let my_status = create_status(self);
        // check
        if status.genesis_hash != my_status.genesis_hash {
            peer.disconnect().await; 
            return // is not a same network
        }
        // sync blocks first
        let start_hei = my_status.latest_height.uint() + 1;
        if my_status.latest_height == 0 && status.latest_height > 0 {

            try_sync_blocks(self, peer, start_hei).await;
            return
        }
        // check hash fork and sync new blocks
        if my_status.latest_height < status.latest_height {
            let mut ubh = self.engine.config().unstable_block;
            if ubh > 255 {
                ubh = 255
            }
            send_req_block_hash_msg(peer, ubh as u8, start_hei).await;
            return
        }
    }

}

async fn try_sync_blocks(hdl: &MsgHandler, peer: Arc<Peer>, starthei: u64) {
    send_req_block_msg(peer, starthei).await;
}

fn create_status(hdl: &MsgHandler) -> HandshakeStatus {
    let latest = hdl.engine.latest_block();
    let mintck = hdl.engine.mint_checker();
    let msgobj = HandshakeStatus {
        genesis_hash: *mintck.genesis().hash(),
        block_version: Uint1::from(1),
        transaction_type: Uint1::from(2),
        action_kind: Uint2::from(12),
        repair_serial: Uint2::from(1),
        __mark: Uint3::from(0),
        latest_height: *latest.objc().height(),
        latest_hash: *latest.hash(),
    };
    msgobj
}
