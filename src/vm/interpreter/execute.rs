




macro_rules! itrparamu8 {
    ($codes: expr, $pc: expr, $tail: expr) => {
        { 
            $pc += 1;
            if $pc >= $tail {
                return itr_err_code!(CodeOverRun)
            }
            $codes[$pc]
        }
    }
}



macro_rules! itrparamu16 {
    ($codes: expr, $pc: expr, $tail: expr) => {
        { 
            let oldpc = $pc;
            $pc += 2;
            if $pc >= $tail {
                return itr_err_code!(CodeOverRun)
            }
            u16::from_be_bytes($codes[oldpc..$pc].try_into().unwrap())
        }
    }
}





pub fn execute_code(

    codes: &[u8], // max len = 65536
    gas_table: &[u8], // len = 256

    gas_usable: &mut i64, // max gas can be use

    operand_stack: &mut Stack,
    locals: &mut Stack,

) -> VmrtRes<ItrExitCode> {

    use super::rt::ItrExitCode::*;
    use super::rt::ItrErrCode::*;
    use super::bytecode::Bytecode::*;

    // check code length
    let codelen = codes.len();
    if codelen > u16::MAX as usize {
        return itr_err_code!(CodeTooLong)
    }
    let tail = codelen;

    // start run
    let mut pc: usize = 0; // pc

    loop {
        // check code seek
        if pc == tail {
            return Ok(Overend) // end of code
        }else if pc > tail {
            return itr_err_code!(CodeOverRun)
        }
        let instbyte = codes[pc as usize]; // u8
        let instruction: Bytecode = unsafe { std::mem::transmute(instbyte) }; // u8

        // do execute
        let gas_extra = 0i64;
        match instruction {
            NOP => {}, // do nothing
            NT  => return itr_err_code!(InstNeverTouch), // never touch
            ABT => return Ok(Abort), // end with error
            END => return Ok(Finish), // finish
            RET => return Ok(Return), // function return
            // constant
            PUSH0 => operand_stack.push(StackItem::U8(0))?,
            PUSH1 => operand_stack.push(StackItem::U8(1))?,
            PUSHU8 => operand_stack.push(StackItem::U8( itrparamu8!(codes, pc, tail) ))?,
            PUSHU16 => operand_stack.push(StackItem::U16( itrparamu16!(codes, pc, tail) ))?,
            // arithmetic
            ADD => binop_arithmetic(operand_stack, add_checked)?, // checked add
            _ => return itr_err_code!(InstInvalid)
        }


        // reduce gas for use
        *gas_usable -= gas_table[instruction as usize] as i64 + gas_extra;
        // check gas
        if *gas_usable < 0 {
            return itr_err_code!(OutOfGas) // out of gas
        }
        // next
        pc += 1;
    }

    // ok call finish
    Ok(Overend)
}