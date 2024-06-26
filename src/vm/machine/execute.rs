

impl Machine {


    pub fn main_call(&mut self, entry: &Address, irs: &[u8]) -> VmrtRes<StackItem> {
        // parse
        let count = u16::from_be_bytes(irs[0..2].try_into().unwrap());
        let codes = parse_ir_list(
            self.extn_caller.clone().as_ref(), 
            count as usize,
            &irs[2..],
        )?.codegen();
        let entry = address_to_contract(entry);
        self.do_call(entry, codes, StackItem::nil(), false)
    }

    pub fn sys_call(&mut self, entry: &Address, fnty: SystemCallType, input: Vec<u8>) -> VmrtRes<StackItem> {
        let entry = address_to_contract(entry);
        let codes = self.load_codes_by_syscall(&entry, fnty)?;
        self.do_call(entry, codes, StackItem::buf(input), true)
    }


    /*
    * do call
    */
    fn do_call(&mut self, entry: ContractAddress, codes: Vec<u8>, input: StackItem, is_sys_call: bool) -> VmrtRes<StackItem> {

        use CallMode::*;

        let mut max_call_depth = self.space_cap.call_depth;
        let mut call_mode = CallMode::Main;
        if is_sys_call {
            max_call_depth = 1; // system call can just 1 depth
            call_mode = CallMode::System;
        }

        let mut retval = StackItem::nil(); 
        let mut current_frame = Frame::new(entry.clone(), entry, call_mode, 0usize, codes, input);
        let mut call_stacks = CallStack::new();


        macro_rules! create_frame_exec {
            () => {
                current_frame.exec(
                    &mut self.gas_limit,
                    &self.gas_table,
                    &self.gas_extra,
                    self.extn_caller.as_ref(),
                    is_sys_call,
                )
            }
        }

        loop {

            let cur_ctx_addr = current_frame.ctx_addr.clone();
            let cur_ivk_addr = current_frame.ivk_addr.clone();
            let mut next_ctx_addr = cur_ctx_addr.clone();

            // do call
            let mut frame_exec = create_frame_exec!();
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
            let (contract_addr, load_codes) = self.load_codes_by_funcptr(
                &cur_ctx_addr,
                &cur_ivk_addr,
                &funcptr,
            )?;
            let mut next_ivk_addr = contract_addr;

            // mode: code
            if let Code = funcptr.mode {
                let mut pc = 0usize;
                let mut frame_exec = create_frame_exec!();
                let res = frame_exec.call_code(&load_codes, &mut pc)?;
                let (Tailend | Finish) = res else {
                    return itr_err_fmt!(CallExitInvalid, 
                        "Call code mode cannot finish by {:?}", res)
                };
                // continue to run prev frame codes
                continue
            }

            // normal call
            if let External | InheritLoc | Library | Static = funcptr.mode {
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
                        next_ctx_addr = adr.clone();
                    }
                }
                let mut next_frame = Frame::new(next_ivk_addr, next_ctx_addr, 
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
