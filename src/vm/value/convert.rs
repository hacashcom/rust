


fn buf_to_uint(buf: &[u8]) -> VmrtRes<StackItem> {
    let rlbts = buf_drop_left_zero(buf);
    let sizen = rlbts.len();
    match sizen {
        1 => Ok(StackItem::U8(rlbts[0])),
        2 => {
            let v = u16::from_be_bytes(rlbts.try_into().unwrap());
            Ok(StackItem::U16(v))
        },
        3..=4 => {
            let bts = buf_fill_left_zero(buf, 4);
            let v = u32::from_be_bytes(bts.try_into().unwrap());
            Ok(StackItem::U32(v))
        },
        5..=8 => {
            let bts = buf_fill_left_zero(buf, 8);
            let v = u64::from_be_bytes(bts.try_into().unwrap());
            Ok(StackItem::U64(v))
        },
        9..=16 => {
            let bts = buf_fill_left_zero(buf, 16);
            let v = u128::from_be_bytes(bts.try_into().unwrap());
            Ok(StackItem::U128(v))
        },
        _ => itr_err_fmt!(CastFail, "cannot cast 0x{} to uint", 
            hex::encode(buf)),
    }
}



