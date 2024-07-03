
pub const FN_SIGN_WIDTH: usize = 4;
pub const CONTRACT_ADDRESS_WIDTH: usize = 21;

pub type ContractAddress = [u8; CONTRACT_ADDRESS_WIDTH];

pub fn contract_to_address(ca: &ContractAddress) -> Address {
    Address::cons(ca.clone())
}

pub fn address_to_contract(adr: &Address) -> ContractAddress {
    **adr
}


pub type FnSign = [u8; FN_SIGN_WIDTH];


macro_rules! unsafe_std_mem_transmute  {
    ($v: expr) => { 
        unsafe { std::mem::transmute($v) }
    }
}


//////////////////////////////////////////

/*

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Contract {
    AST = 0u8,
    Bytecode = 1,
    ______M1 = 2,
    ______M2 = 3,
}
W*/



//////////////////////////////////////////


#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum CodeType {
    AST = 0u8,
    Bytecode = 1,
    ______M1 = 2,
    ______M2 = 3,
}


//////////////////////////////////////////


#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SystemCallType {
    Upgrade      = 0u8,
    Append       = 1,

    PermitHAC    = 5,
    PermitHACD   = 6,
    PermitSAT    = 7,
    PermitAsset  = 8,
    _________a5  = 9,

    PayableHAC   = 15,
    PayableHACD  = 16,
    PayableSAT   = 17,
    PayableAsset = 18,
    ________a10  = 19,

}



