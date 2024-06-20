

impl Machine {


    pub fn call_main(&mut self) -> VmrtRes<StackItem> {

        use CallMode::*;

        let mut retval = StackItem::nil();
        let mut current_frame = self.call_stacks.pop().unwrap();

        loop {
        
            // do execute
            let mut frame_exec = current_frame.exec(
                &mut self.gas_limit,
                &self.gas_table,
                &self.gas_extra,
            );
            let result = frame_exec.call()?;            
     
            // abort
            if let Abort = result {
                let err = current_frame.stack.pop()?.print_string();
                return itr_err_fmt!(ThrowAbort, "Abort: {}", err)
            }

            // finish
            if let Tailend | Finish | Return = result {
                let mut prev_frame = self.call_stacks.pop();
                if let Return = result {
                    retval = current_frame.stack.pop()?;
                }
                match prev_frame {
                    None => { 
                        break // all call finish
                    },
                    Some(frame) => {
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

            // test code
            let mut load_codes = hex::decode("4b4458be002301f5f5f5f5").unwrap();
            if self.call_stacks.len() >= 5 {
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
                self.call_stacks.push(current_frame)?;
                // create frame
                let next_depth = self.call_stacks.len();
                if next_depth >= self.space_limit.call_depth {
                    return itr_err_code!(OutOfCallDepth)
                }
                let next_frame = Frame::new(funcptr.mode, next_depth, load_codes, fnargv);
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
