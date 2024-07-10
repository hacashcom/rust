
/**
* parse bytecode params
*/




macro_rules! checkcodetail {
    ($pc: expr, $tail: expr) => {
        if $pc == $tail {
            return Ok(Tailend) // end of code
        }else if $pc > $tail {
            return itr_err_code!(CodeOverRun)
        }
    }
}

macro_rules! itrbuf {
    ($codes: expr, $pc: expr, $tail: expr, $l: expr) => {
        { 
            let r = $pc + $l;
            if r > $tail {
                return itr_err_code!(CodeOverRun)
            }
            let v: [u8; $l] = $codes[$pc..r].try_into().unwrap();
            $pc = r;
            v
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
            let s = itrparam!{$codes, $pc, $tail, $l, $t} + 1;
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

macro_rules! jump {
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

macro_rules! ostjump {
    ($codes: expr, $pc: expr, $tail: expr, $l: expr) => {
        {
            let tpc = match $l {
                1 => itrparam!{$codes, $pc, $tail, 1, i8} as isize,
                2 => itrparam!{$codes, $pc, $tail, 2, i16} as isize,
                _ => return itr_err_code!(CodeOverRun),
            };
            let tpc = ($pc as isize + tpc);
            if tpc < 0 {
                return itr_err_code!(CodeOverRun)
            }
            checkcodetail!(tpc as usize, $tail);
            $pc = tpc as usize; // jump to
        }
    }
}

macro_rules! branch {
    ( $ops: expr, $codes: expr, $pc: expr, $tail: expr, $l: expr) => {
        if $ops.pop()?.is_not_zero() {
            jump!($codes, $pc, $tail, $l);
        }else{
            $pc += $l;
        }
    }
}

macro_rules! ostbranchex {
    ( $ops: expr, $codes: expr, $pc: expr, $tail: expr, $l: expr, $cond: ident) => {
        if $ops.pop()?.$cond() {
            ostjump!($codes, $pc, $tail, $l);
        }else{
            $pc += $l;
        }
    }
}
// is_not_zero
macro_rules! ostbranch {
    ( $ops: expr, $codes: expr, $pc: expr, $tail: expr, $l: expr) => {
        ostbranchex!($ops, $codes, $pc, $tail, $l, is_not_zero)
    }
}

macro_rules! funcptr {
    ($codes: expr, $pc: expr, $tail: expr, $mode: expr) => {
        {
            let idx = itrparamu8!($codes, $pc, $tail);
            let sig = itrbuf!($codes, $pc, $tail, FN_SIGN_WIDTH);
            Call(Funcptr{
                mode: $mode,
                target: CallTarget::Libidx(idx),
                fnsign: sig,
            })
        }
    }
}


/**
* execute code
*/
pub fn execute_code(

    codes: &[u8], // max len = 65536
    pc: &mut usize, // pc
    mode: &CallMode,

    gas_usable: &mut i64, // max gas can be use

    gas_table: &GasTable, // len = 256
    gas_extra: &GasExtra,

    extactcaller: &mut dyn ExtActCaller,
    outstorager: &mut dyn OutStorager,

    locals: &mut Stack,
    operand_stack: &mut Stack,

    is_sys_call: bool,
    call_depth: usize,

) -> VmrtRes<CallExit> {

    use super::rt::CallExit::*;
    use super::rt::ItrErrCode::*;
    use super::rt::Bytecode::*;

    let ops = operand_stack;

    // check code length
    let codelen = codes.len();
    if codelen > u16::MAX as usize {
        return itr_err_code!(CodeTooLong)
    }
    let tail = codelen;

    macro_rules! pu8 { () => { itrparamu8!(codes, *pc, tail) } }
    macro_rules! pu16 { () => { itrparamu16!(codes, *pc, tail) } }
    macro_rules! pbuf { () => { itrparambuf!(codes, *pc, tail) } }
    macro_rules! pbufl { () => { itrparambufl!(codes, *pc, tail) } }
    macro_rules! pcutbuf { ($w: expr) => { itrbuf!(codes, *pc, tail, $w) } }

    // start run
    loop {

        // check code seek
        checkcodetail!(*pc, tail);
        // read inst
        let instbyte = codes[*pc as usize]; // u8
        let instu8 = instbyte.clone(); // u8
        let instruction: Bytecode = unsafe_std_mem_transmute!(instu8); // u8
        *pc += 1; // next

        // do execute
        let mut gas_added = 0i64;

        macro_rules! extcall { ($ifv: expr) => { 
            let mut actbody = vec![instbyte, pu8!()];
            if $ifv {
                let mut bdv = ops.peek()?.cast_to_buf();
                actbody.append(&mut bdv);
            }
            let (gasu, cres) = extactcaller.call(actbody, call_depth as i8).map_err(|e|
                ItrErr::new(ExtActCallError, &format!("{}", &e)))?;
            gas_added += gasu;
            let resv = StackItem::buf(cres);
            if $ifv {
                *ops.peek()? = resv;
            } else {
                ops.push( resv );
            }
        }}

        match instruction {
            // ext action
            EXTACTION  => {
                if is_sys_call || call_depth>0 {
                    return itr_err_code!(InstDisabled)
                }
                extcall!(true);
            },
            EXTFUNC    => { extcall!(true); },
            EXTENV     => { extcall!(false); },
            // native call
            NATIVECALL => { *ops.peek()? = (native_call(pu8!(), ops.peek()?)?) }
            NATIVEENV => ops.push( native_env(pu8!())? )?,
            // constant
            PUSH0    => ops.push(StackItem::U8(0))?,
            PUSH1    => ops.push(StackItem::U8(1))?,
            PUSHU8   => ops.push(StackItem::U8(pu8!()))?,
            PUSHU16  => ops.push(StackItem::U16(pu16!()))?,
            PUSHNBUF => ops.push(StackItem::empty_buf())?,
            PUSHBUF  => ops.push(pbuf!())?,
            PUSHBUFL => ops.push(pbufl!())?, // buf long
            DUP  =>  ops.push(ops.last()?)?,
            POP  => { ops.pop()?; }, // drop
            SWAP => ops.swap()?,
            // locals
            ALLOC => {
                let num = pu8!();
                gas_added += num as i64 * gas_extra.resource_local_item; // resource fee
                locals.alloc(num)?;
            },
            PUT => locals.save(ops.pop()?, pu8!() as u16)?,
            GET => ops.push(locals.load(pu8!() as u16)?)?,
            // cast
            CASTU8   => ops.peek()?.cast_u8()?,
            CASTU16  => ops.peek()?.cast_u16()?,
            CASTU32  => ops.peek()?.cast_u32()?,
            CASTU64  => ops.peek()?.cast_u64()?,
            CASTU128 => ops.peek()?.cast_u128()?,
            /*CASTU256 => ops.peek()?.cast_u256()?,*/
            CASTBUF  => ops.peek()?.cast_buf()?,
            // logic
            NOT => ops.peek()?.cast_bool_not()?,
            EQ  => binop_btw(ops, lgc_equal)?,
            NEQ => binop_btw(ops, lgc_not_equal)?,
            LT  => binop_btw(ops, lgc_lt)?,
            GT  => binop_btw(ops, lgc_gt)?,
            LE  => binop_btw(ops, lgc_le)?,
            GE  => binop_btw(ops, lgc_ge)?,
            // arithmetic
            ADD => binop_arithmetic(ops, add_checked)?,
            SUB => binop_arithmetic(ops, sub_checked)?,
            MUL => binop_arithmetic(ops, mul_checked)?,
            DIV => binop_arithmetic(ops, div_checked)?,
            MOD => binop_arithmetic(ops, mod_checked)?,
            POW => binop_arithmetic(ops, pow_checked)?,
            // workflow control
            JMPL  =>        jump!(codes, *pc, tail, 2),
            JMPS  =>     ostjump!(codes, *pc, tail, 1),
            JMPSL =>     ostjump!(codes, *pc, tail, 2),
            BRL   =>      branch!(ops, codes, *pc, tail, 2),
            BRS   =>   ostbranch!(ops, codes, *pc, tail, 1),
            BRSL  =>   ostbranch!(ops, codes, *pc, tail, 2),   
            BRNSL => ostbranchex!(ops, codes, *pc, tail, 2, is_zero),   
            // other
            NT  => return itr_err_code!(InstNeverTouch), // never touch
            NOP => {}, // do nothing
            BURN => gas_added += pu16!() as i64,         
            // exit
            RET => return Ok(Return), // function return
            ABT => return Ok(Abort), // end with error
            END => return Ok(Finish), // finish
            CALLCODE =>   return Ok(funcptr!(codes, *pc, tail, CallMode::Code)),
            CALLSTATIC => return Ok(funcptr!(codes, *pc, tail, CallMode::Static)),
            CALLLIB =>    return Ok(funcptr!(codes, *pc, tail, CallMode::Library)),
            CALLLOC =>    return Ok(Call(Funcptr{
                mode: CallMode::InheritLoc,
                target: CallTarget::Inherit,
                fnsign: pcutbuf!(FN_SIGN_WIDTH),
            })),
            CALL => return Ok(Call(Funcptr{ // External
                mode: CallMode::External,
                target: CallTarget::Addr(pcutbuf!(CONTRACT_ADDRESS_WIDTH)),
                fnsign: pcutbuf!(FN_SIGN_WIDTH),
            })),
            // inst invalid
            _ => return itr_err_code!(InstInvalid),
        }

        // reduce gas for use
        *gas_usable -= gas_table.gas(instbyte) + gas_added;
        // check gas
        if *gas_usable < 0 {
            return itr_err_code!(OutOfGas) // out of gas
        }
        // next
        continue
    }

    // ok call finish
    Ok(Tailend)
}