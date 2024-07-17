

// error define
#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ItrErrCode {
    ContractError = 1,
    CodeTypeError = 2,
    CodeTooLong = 3, // code length
    CodeOverRun = 4, // pc out of limit
    
    InstInvalid    = 5, // 
    InstDisabled   = 6, // 
    InstNeverTouch = 7, // 
    
    OutOfGas       = 11,
    OutOfStack     = 12,
    OutOfLocal     = 13,
    OutOfHeap      = 14,
    OutOfMemory    = 15,
    OutOfGlobal    = 16,
    OutOfCallDepth = 17,
    OutOfLoadContract = 18,
    
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
    BufferOpFail = 37,
    
    Arithmetic = 41,
    BufferHandle = 42,
    NativeCall = 43,

    ExtActCallError = 51,
    OutStorageError = 55,

    ThrowAbort = 101, // user code call
}

#[derive(Debug)]
pub struct ItrErr(pub ItrErrCode, pub String);

impl ToString for ItrErr {
    fn to_string(&self) -> String {
        format!("{:?}: {}", self.0, self.1)
    }
}


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

#[macro_export] 
macro_rules! cannot_cast_err {
    ($v: expr, $ty: expr) => {
        itr_err_fmt!(CastFail, "cannot cast {:?} to {}", $v, $ty)
    }
}