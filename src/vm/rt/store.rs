
#[inline(always)]
pub fn contract_code_content_store_key(addr: &ContractAddress) -> Vec<u8> {
    // hvm-contract-code-object-storage-key
    // + key frefix
    vec![vec![3,1], addr.to_vec()].concat()
}

#[inline(always)]
pub fn contract_state_storage_key(addr: &ContractAddress, key: Vec<u8>) -> Vec<u8> {
    // hvm-contract-code-object-storage-key    
    let mut k = vec![vec![3,2], addr.to_vec(), key].concat();
    let kl = k.len();
    if kl > 32 {
        k = sha2(&k).to_vec();
    }
    k
}


