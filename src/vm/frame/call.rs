

use super::rt::CallExit::*;
use super::rt::ItrErrCode::*;
use super::interpreter::*;


macro_rules! do_execute_code {
    ($self: ident, $codes: expr, $pc: expr, ) => {
        execute_code(
            $codes,
            $pc,
            $self.mode,
            $self.gas_limit,
            $self.gas_table,
            $self.gas_extra,
            $self.extcaller,
            $self.local,
            $self.stack,
            $self.is_sys_call,
            $self.depth,
        )

    }
}

impl FrameExec<'_, '_> {

    #[inline(always)]
    pub fn call(&mut self) -> VmrtRes<CallExit> {
        // println!("FrameExec call pc = {}", self.pc);
        do_execute_code!{
            self,
            self.codes,
            self.pc,
        }
    }

    #[inline(always)]
    pub fn call_code(&mut self, codes: &[u8], pc: &mut usize) -> VmrtRes<CallExit> {
        do_execute_code!{
            self,
            codes,
            pc,
        }
    }



}


