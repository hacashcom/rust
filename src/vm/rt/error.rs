

// error define
#[repr(u8)]
#[derive(Debug)]
pub enum ItrErrCode {
    CodeTooLong = 1, // code length
    CodeOverRun = 2, // pc out of limit
    
    InstInvalid    = 5, // 
    InstNeverTouch = 6, // 
    
    OutOfGas       = 11,
    OutOfStack     = 12,
    OutOfLocal     = 13,
    OutOfHeap      = 14,
    OutOfMemory    = 15,
    OutOfGlobal    = 16,
    OutOfCallDepth = 17,
    
    GasError    = 21,
    StackError  = 22,
    LocalError  = 23,
    HeapError   = 24,
    MemoryError = 25,
    GlobalError = 26,
    
    CallNotExist = 31,
    CallInvalid  = 32,
    CallExitInvalid  = 33,
    
    CastFail = 36,
    
    Arithmetic = 41,

    ThrowAbort = 101, // user code call
}

#[derive(Debug)]
pub struct ItrErr(pub ItrErrCode, pub String);

impl ItrErr {
    pub fn new(n: ItrErrCode, tip: &str) -> ItrErr {
        ItrErr(n, tip.to_string())
    }
    pub fn code(n: ItrErrCode) -> ItrErr {
        ItrErr(n, "".to_string())
    }
}

// VM Runtime Error
pub type VmrtRes<T> = Result<T, ItrErr>;
pub type VmrtErr = Result<(), ItrErr>;


#[macro_export] 
macro_rules! itr_err {
    ($code: expr, $tip: expr) => {
        Err(ItrErr($code, $tip.to_string()))
    }
}

#[macro_export] 
macro_rules! itr_err_code {
    ($code: expr) => {
        Err(ItrErr($code, "".to_string()))
    }
}

#[macro_export] 
macro_rules! itr_err_fmt {
    ($code: expr, $( $v: expr),+ ) => {
        Err(ItrErr::new($code, &format!($( $v ),+)))
    }
}
