

use super::rt::CallExit::*;
use super::rt::ItrErrCode::*;


impl CallFrameExec<'_> {

    pub fn call_code(&mut self, codes: &[u8]) -> VmrtRes<StackItem> {

        // test
        let gas_limit = 15000000i64;
        let mut gas_usable = gas_limit;
        let gas_table = vec![1].repeat(256);

        let result = execute_code(
            &codes,
            &gas_table,
            &mut gas_usable,
            self.pc,
            self.local,
            self.stack,
        )?;
        
        match result {
            Tailend | Finish => Ok(StackItem::Nil),
            Return => Ok( self.stack.pop()? ),
            Abort => itr_err_fmt!(ThrowAbort, "Abort: {}", self.stack.pop()?.print_string()),
            Call(funcptr) => Ok( StackItem::U8(1) ),
        }
    
    }

    pub fn call(&mut self) -> VmrtRes<StackItem> {
        self.call_code(&self.codes)
    }
}