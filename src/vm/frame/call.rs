

use super::rt::CallExit::*;
use super::rt::ItrErrCode::*;


impl FrameExec<'_, '_> {

    pub fn call(&mut self) -> VmrtRes<CallExit> {
        // println!("FrameExec call pc = {}", self.pc);
        execute_code(
            self.codes,
            self.pc,
            self.mode,
            self.gas_limit,
            self.gas_table,
            self.gas_extra,
            self.local,
            self.stack,
        )
    }

    pub fn call_code(&mut self, codes: &[u8], pc: &mut usize) -> VmrtRes<CallExit> {
        execute_code(
            codes,
            pc,
            self.mode,
            self.gas_limit,
            self.gas_table,
            self.gas_extra,
            self.local,
            self.stack,
        )
    }



}


