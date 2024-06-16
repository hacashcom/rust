


#[derive(Debug, Clone)]
pub enum CallTarget {
    Local,
    Libidx(u8),
    Addr(Address),
}

#[derive(Debug, Clone)]
pub enum CallMode {
    External,
    Inherit,
    Library,
    Static,
    Code,
}

pub type FnSign = [u8; 4];

#[derive(Debug, Clone)]
pub struct Funcptr {
    pub mode: CallMode,
    pub target: CallTarget,
    pub fnsign: FnSign,
}


