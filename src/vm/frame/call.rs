

use super::rt::CallExit::*;
use super::rt::ItrErrCode::*;
use super::interpreter::*;


macro_rules! do_execute_code {
    ($self: ident, $codes: expr, $pc: expr, ) => {
        execute_code_of_call(
            $codes,
            $pc,
            $self.mode,
            $self.gas_limit,
            $self.gas_table,
            $self.gas_extra,
            $self.space_cap,
            $self.extn_caller,
            $self.out_storage,
            $self.stack,
            $self.local,
            $self.heap,
            $self.memory,
            $self.global,
            $self.ctx_addr,
            $self.is_sys_call,
            $self.depth,
            $self.pending_height,
        )

    }
}

impl FrameExec<'_, '_, '_> {

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


