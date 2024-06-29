

pub const FN_SIGN_WIDTH: usize = 4;
pub const CONTRACT_ADDRESS_WIDTH: usize = 21;

pub type ContractAddress = [u8; CONTRACT_ADDRESS_WIDTH];

pub fn contract_to_address(ca: &ContractAddress) -> Address {
    Address::cons(ca.clone())
}

pub fn address_to_contract(adr: &Address) -> ContractAddress {
    **adr
}


#[derive(Debug, Clone)]
pub enum CallTarget {
    Inherit,
    Libidx(u8),
    Addr(ContractAddress),
}

impl CallTarget {
    pub fn idx(&self) -> u8 {
        match self {
            CallTarget::Libidx(i) => *i,
            _ => 0,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CallMode {
    Main,
    System,
    External,
    Inherit,
    Library,
    Static,
    Code,
}


pub type FnSign = [u8; FN_SIGN_WIDTH];

#[derive(Debug, Clone)]
pub struct Funcptr {
    pub mode: CallMode,
    pub target: CallTarget,
    pub fnsign: FnSign,
}


