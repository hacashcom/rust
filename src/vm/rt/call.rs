


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
    InheritLoc,
    Library,
    Static,
    Code,
}


#[derive(Debug, Clone)]
pub struct Funcptr {
    pub mode: CallMode,
    pub target: CallTarget,
    pub fnsign: FnSign,
}


