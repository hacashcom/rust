
/////////////////// DIAL ///////////////////


pub async fn tcp_dial_handshake(addr: SocketAddr, outsec: u64) -> Ret<TcpStream> {
    let mut stream = errunbox!( TcpStream::connect(addr).await )?;
    let conn = &mut stream;
    tcp_check_handshake(conn, 4).await?;
    Ok(stream)
}

pub async fn tcp_dial_handshake_send_msg_and_read_all(addr: SocketAddr, msgty: u8, outsec: u64) -> Ret<Vec<u8>> {
    let mut stream = tcp_dial_handshake(addr, outsec).await?;
    let conn = &mut stream;
    tcp_send_msg(conn, msgty, vec![]).await?;
    // read all and no timeout
    tcp_read(conn, 0, outsec).await
}

pub async fn tcp_dial_to_check_is_public_id(addr: SocketAddr, pid: &PeerKey, outsec: u64) -> Ret<bool> {
    let mut conn = errunbox!( TcpStream::connect(addr).await )?;
    let conn = &mut conn;
    tcp_check_handshake(conn, 4).await?;
    tcp_send_msg(conn, MSG_REQUEST_NODE_KEY_FOR_PUBLIC_CHECK, vec![]).await?;
    let retmsg = tcp_read(conn, PEER_KEY_SIZE, 3).await?;
    if *pid != *retmsg {
        return errf!("peer id not match") // false
    }
    Ok(true)
}



/////////////////// READ WRITE ///////////////////


pub async fn tcp_check_handshake(conn: &mut (impl AsyncRead + AsyncWrite + Unpin), outsec: u64) -> RetErr {
    // send handshake
    let handshake = P2P_HAND_SHAKE_MAGIC_NUMBER.to_be_bytes();
    errunbox!( AsyncWriteExt::write_all(conn, &handshake).await )?;
    // check magic
    let magicn = tcp_read(conn, 4, outsec).await?;
    if magicn != handshake {
        return errf!("tcp handshake magic number error")
    }
    Ok(())
}

pub async fn tcp_handshake_read_one_msg(conn: &mut (impl AsyncRead + AsyncWrite + Unpin), outsec: u64) -> Ret<(u8, Vec<u8>)> {
    // check handshake
    tcp_check_handshake(conn, outsec+2).await?;
    // read msg
    tcp_read_msg(conn, outsec).await
}


/////////////////// WRITE ///////////////////


pub async fn tcp_send_msg(conn: &mut (impl AsyncWrite + Unpin), msgty: u8, mut msgbody: Vec<u8>) -> RetErr {

    let msgsize: u32 = 1 + msgbody.len() as u32;
    let mut bufcon = Vec::with_capacity(4 + msgsize as usize);
    bufcon.append(&mut msgsize.to_be_bytes().to_vec());
    bufcon.push(msgty);
    bufcon.append(&mut msgbody);
    tcp_send(conn, &bufcon).await
}

pub async fn tcp_send(conn: &mut (impl AsyncWrite + Unpin), body: &Vec<u8>) -> RetErr {
    errunbox!( AsyncWriteExt::write_all(conn, body).await )
}


/////////////////// READ ///////////////////


// return: ty body
pub async fn tcp_read_msg(conn: &mut (impl AsyncRead + std::marker::Unpin), outsec: u64) -> Ret<(u8, Vec<u8>)> {
    let size = tcp_read(conn, 4, outsec).await?;
    let size = u32::from_be_bytes( bufcut!(size, 0, 4) );
    if size < 1 || size > P2P_MSG_DATA_MAX_SIZE {
        return errf!("tcp msg size error")
    }
    let tybody = tcp_read(conn, size as usize, outsec).await?;
    // ok
    Ok((tybody[0], tybody[1..].to_vec()))
}


pub async fn tcp_read(conn: &mut (impl AsyncRead + std::marker::Unpin), readlen: usize, outsec: u64) -> Ret<Vec<u8>> {
    let is_read_all = readlen == 0;
    let is_time_out = outsec > 0 && outsec < u64::MAX;
    let mut buf = vec![];
    if is_read_all {

        // read all
        let ft = AsyncReadExt::read_to_end(conn, &mut buf);
        if is_time_out {
            // timeout
            if let Err(_) = timeout(secs(outsec), ft).await {
                return errf!("tcp read timeout")
            }
        }else{
            errunbox!( ft.await )?;
        }

    }else{
        // read length
        buf = vec![0u8; readlen];
        let ft = AsyncReadExt::read_exact(conn, &mut buf);
        if is_time_out {
            // timeout
            if let Err(_) = timeout(secs(outsec), ft).await {
                return errf!("tcp read timeout")
            }
        }else{
            errunbox!( ft.await )?;
        }

    }
    // ok finish
    Ok(buf)
}