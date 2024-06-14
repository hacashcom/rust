


#[repr(u8)]
pub enum Bytecode {
    NT    = 0xff, // panic: never touch

    END   = 0xef, // end with finish
    ABT   = 0xee, // end with error (abort)
    RET   = 0xed, // ret with data (function return)
    NOP   = 0xec, // do nothing

    PUSH0 = 0x4a,
    PUSH1 = 0x4b,
    PUSHU8 = 0x48,
    PUSHU16 = 0x49,

    ADD   = 0x80, // +
}










/*
macro_rules! define_bytecode_ptrs {
    ($( $name:ident : $bytv:expr)+) => {  
        $(
// concat_idents!(op_name = OP_, $name { pub const op_name: u8 = $bytv; });

pub const $name: u8 = $bytv;

        )+
    }
}

// define
// pub const OP_NOP: u8 = 0xfd;
define_bytecode_ptrs!{

    NT    : 0xff // panic: never touch

    END   : 0xef // end with finish
    ABT   : 0xee // end with error (abort)
    RET   : 0xed // ret with data (function return)
    NOP   : 0xec // do nothing

    ADD   : 0x80 // +

}
*/