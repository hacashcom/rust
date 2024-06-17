

pub const FN_SIGN_WIDTH: usize = 4;
pub const CONTRACT_ADDRESS_WIDTH: usize = 21;

pub type ContractAddress = [u8; CONTRACT_ADDRESS_WIDTH];


#[derive(Debug, Clone)]
pub enum CallTarget {
    Inherit,
    Libidx(u8),
    Addr(ContractAddress),
}

#[derive(Debug, Clone, Copy)]
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


