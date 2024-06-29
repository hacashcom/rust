
const NORMAL_HASH_SIZE: usize = 32;


// sha3
fn sha3(v: &StackItem) -> VmrtRes<StackItem> {
    let stuff = v.cast_to_buf();
    if stuff.len() == 0 {
        return itr_err_fmt!(NativeCall, "cannot do sha3 with empty bytes")
    }
    let mut hasher = Sha3_256::new();
    hasher.update(stuff);
    let result = hasher.finalize();
    let result: [u8; NORMAL_HASH_SIZE] = result[..].try_into().unwrap();
    Ok(StackItem::buf(result.to_vec()))
}


// sha2
fn sha2(v: &StackItem) -> VmrtRes<StackItem> {
    let stuff = v.cast_to_buf();
    if stuff.len() == 0 {
        return itr_err_fmt!(NativeCall, "cannot do sha2 with empty bytes")
    }
    let mut hasher = Sha256::new();
    hasher.update(stuff);
    let result = hasher.finalize();
    let result: [u8; NORMAL_HASH_SIZE] = result[..].try_into().unwrap();
    Ok(StackItem::buf(result.to_vec()))
}


// ripemd160
fn ripemd160(v: &StackItem) -> VmrtRes<StackItem> {
    let stuff = v.cast_to_buf();
    if stuff.len() == 0 {
        return itr_err_fmt!(NativeCall, "cannot do ripemd160 with empty bytes")
    }
    let mut hasher = Ripemd160::new();
    hasher.update(stuff);
    // to [u8; 20]
    let result = hasher.finalize();
    let result: [u8; 20] = result[..].try_into().unwrap();
    Ok(StackItem::buf(result.to_vec()))
}
