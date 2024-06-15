
/**
* parse bytecode params
*/




macro_rules! checkcodetail {
    ($pc: expr, $tail: expr) => {
        if $pc == $tail {
            return Ok(Overend) // end of code
        }else if $pc > $tail {
            return itr_err_code!(CodeOverRun)
        }
    }
}

macro_rules! itrparam {
    ($codes: expr, $pc: expr, $tail: expr, $l: expr, $t: ty) => {
        { 
            let r = $pc + $l; 
            if r > $tail {
                return itr_err_code!(CodeOverRun)
            }
            let v = <$t>::from_be_bytes($codes[$pc..r].try_into().unwrap());
            $pc = r;
            v
        }
    }
}

macro_rules! itrparamu8 {
    ($codes: expr, $pc: expr, $tail: expr) => {
        itrparam!{$codes, $pc, $tail, 1, u8}
    }
}

macro_rules! itrparamu16 {
    ($codes: expr, $pc: expr, $tail: expr) => {
        itrparam!{$codes, $pc, $tail, 2, u16}
    }
}

macro_rules! itrparambufex {
    ($codes: expr, $pc: expr, $tail: expr, $l: expr, $t: ty) => {
        {
            let s = itrparam!{$codes, $pc, $tail, $l, $t};
            let l = $pc;
            let r = l + s as usize;
            if r > $tail {
                return itr_err_code!(CodeOverRun)
            }
            $pc = r;
            StackItem::Buffer( $codes[l..r].into() )
        }
    }
}

macro_rules! itrparambuf {
    ($codes: expr, $pc: expr, $tail: expr) => {
        itrparambufex!($codes, $pc, $tail, 1, u8)
    }
}

macro_rules! itrparambufl {
    ($codes: expr, $pc: expr, $tail: expr) => {
        itrparambufex!($codes, $pc, $tail, 2, u16)
    }
}

macro_rules! dojump {
    ($codes: expr, $pc: expr, $tail: expr, $l: expr) => {
        {
            let tpc = match $l {
                1 =>  itrparamu8!($codes, $pc, $tail) as usize,
                2 => itrparamu16!($codes, $pc, $tail) as usize,
                _ => return itr_err_code!(CodeOverRun),
            };
            checkcodetail!(tpc, $tail);
            $pc = tpc; // jump to
        }
    }
}

macro_rules! dobr {
    ( $operand_stack: expr, $codes: expr, $pc: expr, $tail: expr, $l: expr) => {
        if $operand_stack.pop()?.is_not_zero() {
            dojump!($codes, $pc, $tail, $l);
        }else{
            $pc += $l;
        }
    }
}


/**
* execute code
*/

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
        checkcodetail!(pc, tail);
        // read inst
        let instbyte = codes[pc as usize]; // u8
        let instruction: Bytecode = unsafe { std::mem::transmute(instbyte) }; // u8
        pc += 1; // next

        // println!("{} {:?}", pc, instruction);

        // do execute
        let mut gas_extra = 0i64;
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
            PUSHBUF => operand_stack.push(itrparambuf!(codes, pc, tail))?,
            PUSHBUFL => operand_stack.push(itrparambufl!(codes, pc, tail))?, // buf long
            DUP =>  operand_stack.push(operand_stack.last()?)?,
            POP => { operand_stack.pop()?; }, // drop
            // locals
            ALLOC => {
                let num = itrparamu8!(codes, pc, tail);
                gas_extra += num as i64 * 6; // resource fee
                locals.alloc(num)?;
            },
            PUT => locals.save(operand_stack.pop()?, itrparamu8!(codes, pc, tail) as u16)?,
            GET => operand_stack.push(locals.load(itrparamu8!(codes, pc, tail) as u16)?)?,
            // cast
            CASTU8 => operand_stack.peek()?.cast_u8()?,
            CASTU16 => operand_stack.peek()?.cast_u16()?,
            CASTU32 => operand_stack.peek()?.cast_u32()?,
            CASTU64 => operand_stack.peek()?.cast_u64()?,
            CASTU128 => operand_stack.peek()?.cast_u128()?,
            /*CASTU256 => operand_stack.peek()?.cast_u256()?,*/
            CASTBUF => operand_stack.peek()?.cast_buf()?,
            // logic
            NOT => operand_stack.peek()?.cast_bool_not()?,
            EQ => binop_btw(operand_stack, lgc_equal)?,
            NEQ => binop_btw(operand_stack, lgc_not_equal)?,
            LT => binop_btw(operand_stack, lgc_lt)?,
            GT => binop_btw(operand_stack, lgc_gt)?,
            LE => binop_btw(operand_stack, lgc_le)?,
            GE => binop_btw(operand_stack, lgc_ge)?,
            // arithmetic
            ADD => binop_arithmetic(operand_stack, add_checked)?,
            SUB => binop_arithmetic(operand_stack, sub_checked)?,
            MUL => binop_arithmetic(operand_stack, mul_checked)?,
            DIV => binop_arithmetic(operand_stack, div_checked)?,
            MOD => binop_arithmetic(operand_stack, mod_checked)?,
            POW => binop_arithmetic(operand_stack, pow_checked)?,
            // workflow control
            JMP =>  dojump!(codes, pc, tail, 1),
            JMPL => dojump!(codes, pc, tail, 2),
            BR =>  dobr!(operand_stack, codes, pc, tail, 1),
            BRL => dobr!(operand_stack, codes, pc, tail, 2),
            // inst invalid
            _ => return itr_err_code!(InstInvalid),
        }

        // reduce gas for use
        *gas_usable -= gas_table[instruction as usize] as i64 + gas_extra;
        // check gas
        if *gas_usable < 0 {
            return itr_err_code!(OutOfGas) // out of gas
        }
        // next
        continue
    }

    // ok call finish
    Ok(Overend)
}