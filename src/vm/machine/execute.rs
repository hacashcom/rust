

impl Machine<'_> {


    pub fn main_call(&mut self, caller: ContractAddress, codes: Vec<u8>) -> VmrtRes<StackItem> {
        self.do_call(caller, codes, StackItem::nil(), false)
    }

    pub fn sys_call(&mut self, caller: ContractAddress, codes: Vec<u8>, input: Vec<u8>) -> VmrtRes<StackItem> {
        self.do_call(caller, codes, StackItem::buf(input), true)
    }


    /*
    * do call
    */
    fn do_call(&mut self, caller: ContractAddress, codes: Vec<u8>, input: StackItem, is_sys_call: bool) -> VmrtRes<StackItem> {

        use CallMode::*;

        let mut max_call_depth = self.space_cap.call_depth;
        let mut call_mode = CallMode::Main;
        if is_sys_call {
            max_call_depth = 1; // system call can just 1 depth
            call_mode = CallMode::System;
        }

        let mut retval = StackItem::nil(); 
        let mut current_frame = Frame::new(caller.clone(), caller, call_mode, 0usize, codes, input);
        let mut call_stacks = CallStack::new();

        loop {

            let cur_ivk_addr = current_frame.ivk_addr;
            let mut frame_exec = current_frame.exec(
                &mut self.gas_limit,
                &self.gas_table,
                &self.gas_extra,
                self.extcaller,
                is_sys_call,
            );
            let result = frame_exec.call()?;

            // abort
            if let Abort = result {
                let err = current_frame.stack.pop()?.print_string();
                return itr_err_fmt!(ThrowAbort, "Abort: {}", err)
            }

            // finish
            if let Tailend | Finish | Return = result {
                if let Return = result {
                    retval = current_frame.stack.pop()?;
                } else {
                    retval = StackItem::nil();
                }
                let mut prev_frame = call_stacks.pop();
                match prev_frame {
                    None => { 
                        break // all call finish
                    },
                    Some(frame) => {
                        // return prev func to continue
                        current_frame = frame;
                        current_frame.stack.push(retval.clone())?; // put func return value
                        continue // continue to run prev frame codes
                    },
                };
            }

            // call func
            let Call(ref funcptr) = result else {
                return itr_err_code!(CallExitInvalid)
            };

            // check
            if is_sys_call && External == funcptr.mode {
                // system func cannot do External call
                return itr_err_fmt!(CallInvalid, "Sys call mode invalid call: {:?}", funcptr)
            }

            // load code
            let mut next_ivk_addr = cur_ivk_addr;
            let mut next_sto_addr = cur_ivk_addr;
            let mut load_codes = hex::decode("4b4458be002301f5f5f5f5").unwrap();
            if call_stacks.len() >= 5 {
                load_codes = hex::decode("4aee").unwrap(); // abort
            }

            // mode: code
            if let Code = funcptr.mode {
                let mut pc = 0usize;
                let res = frame_exec.call_code(&load_codes, &mut pc)?;
                let (Tailend | Finish) = res else {
                    return itr_err_fmt!(CallExitInvalid, 
                        "Call code mode cannot finish by {:?}", res)
                };
                // continue to run prev frame codes
                continue
            }

            // normal call
            if let External | Inherit | Library | Static = funcptr.mode {
                // save prev frame
                let fnargv = current_frame.stack.pop()?; // func argv
                call_stacks.push(current_frame)?;
                // create frame
                let next_depth = call_stacks.len();
                if next_depth >= max_call_depth {
                    return itr_err_code!(OutOfCallDepth)
                }
                if External == funcptr.mode {
                    if let CallTarget::Addr(adr) = funcptr.target {
                        next_ivk_addr = adr.clone();
                        next_sto_addr = adr.clone();
                    }
                }
                let mut next_frame = Frame::new(next_ivk_addr, next_sto_addr, 
                    funcptr.mode, next_depth, load_codes, fnargv);
                current_frame = next_frame;
                continue
            }

            // some error
            return itr_err_fmt!(CallInvalid, "Invalid call: {:?}", funcptr)
        }

        // all call finished, function return
        return Ok(retval)

    }




}
