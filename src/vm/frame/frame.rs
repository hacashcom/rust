

/**
*
*/
pub struct CallFrame<'a> {
    pub storage: &'a mut u8,
    pub global: &'a mut u8,
    pub memory: &'a mut u8,
    pub heap: u8,
    pub local: u8,
    pub stack: u8,
} 


pub struct CallFrameExec<'a> {
    pub storage: &'a mut u8,
    pub global: &'a mut u8,
    pub memory: &'a mut u8,
    pub heap: &'a mut u8,
    pub local: &'a mut u8,
    pub stack: &'a mut u8,
} 

