
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
pub fn execute_code_of_call(

    codes: &[u8], // max len = 65536
    pc: &mut usize, // pc
    mode: &CallMode,

    gas_usable: &mut i64, // max gas can be use

    gas_table: &GasTable, // len = 256
    gas_extra: &GasExtra,
    space_cap: &SpaceCap,

    extactcaller: &mut dyn ExtActCaller,
    outstorager: &mut dyn OutStorager,

    operand_stack: &mut Stack,
    locals: &mut Stack,
    heap:  &mut Heap,
    memory: &mut AddrKVMap,
    global: &mut KVMap,

    ctx_addr: &ContractAddress,

    is_sys_call: bool,
    call_depth: usize,
    height: u64, // pedding block height

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
    macro_rules! pk { () => { ops.peek()? } }
    macro_rules! pv { () => { ops.pop()? } }
    macro_rules! pvr { () => { &pv!() } }
    macro_rules! pcutbuf { ($w: expr) => { itrbuf!(codes, *pc, tail, $w) } }
    macro_rules! peekset { ($f: ident) => { pk!().$f()? } }
    macro_rules! peeksetp1 { ($f: ident) => { { let p1=pv!(); pk!().$f(p1)? } } }
    macro_rules! peeksetp2 { ($f: ident) => { 
        { let p1=pv!(); let p2=pv!(); pk!().$f(p2, p1)? } 
    } }
    macro_rules! stowrap { () => { 
        Storage::wrap(height, pv!(), ctx_addr, outstorager, space_cap)
    } }

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
        let mut gsd = 0i64; // gas added

        macro_rules! extcall { ($ifv: expr) => { 
            let mut actbody = vec![instbyte, pu8!()];
            if $ifv {
                let mut bdv = pk!().to_buf()?;
                actbody.append(&mut bdv);
            }
            let (gasu, cres) = extactcaller.call(actbody, call_depth as i8).map_err(|e|
                ItrErr::new(ExtActCallError, &format!("{}", &e)))?;
            gsd += gasu;
            let resv = StackItem::buf(cres);
            if $ifv {
                *pk!() = resv;
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
            NATIVECALL => { let pk=pk!(); *pk = (native_call(pu8!(), pk)?) }
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
            POP  => { pv!(); }, // drop
            CAT  => peeksetp1!(opbuf_cat),
            SWAP => ops.swap()?,
            // buf
            CUT  => peeksetp2!(opbuf_cut),
            BYTE => peeksetp1!(opbuf_byte),
            TYPE => peekset!(cast_type_id),
            SIZE => peekset!(cast_size_num),
            // locals
            ALLOC => {
                let num = pu8!();   
                gsd += num as i64 * gas_extra.resource_local_item; // resource fee
                locals.alloc(num)?;
            },
            PUT => locals.save(pv!(), pu8!() as u16)?,
            GET => ops.push(locals.load(pu8!() as u16)?)?,
            // heap & memory & global & storage
            HGROW    => gsd += heap.grow( pu8!() )?,
            HREAD    => *pk!() = heap.read( pvr!(), pk!() )?,
            HREADU   => ops.push( heap.readu(  pu8!() )? )?,
            HREADUL  => ops.push( heap.readul( pu16!() )? )?,
            HWRITE   => heap.write(   pvr!(), pvr!() )?,
            HWRITEX  => heap.writex(  pu8!(),      pvr!() )?,
            HWRITEXL => heap.writexl( pu16!(),     pvr!() )?,
            GPUT => global.put(pvr!(), pv!())?,
            GGET => *pk!() = global.get(pk!())?,
            MPUT => memory.entry(*ctx_addr).put(pvr!(), pv!())?,
            MGET => *pk!() = memory.entry(*ctx_addr).get(pk!())?,
            SRENT => gsd += stowrap!().rent_time( pu16!() )?,
            SSAVE => gsd += stowrap!().save( pvr!() )?,
            SLOAD => { let sv = stowrap!().load()?; 
                ops.push(sv.0)?/*expired*/; 
                ops.push(sv.1)?/*value*/ },
            // cast
            CASTU8   => peekset!(cast_u8),
            CASTU16  => peekset!(cast_u16),
            CASTU32  => peekset!(cast_u32),
            CASTU64  => peekset!(cast_u64),
            CASTU128 => peekset!(cast_u128),
            /*CASTU256 => peekset!(cast_u256),*/
            CASTBUF  => peekset!(cast_buf),
            // logic
            AND => binop_btw(ops, lgc_and)?,
            OR =>  binop_btw(ops, lgc_or)?,
            NOT => peekset!(cast_bool_not),
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
            INC => { ops.push(StackItem::U8(1)); binop_arithmetic(ops, add_checked)? },
            DEC => { ops.push(StackItem::U8(1)); binop_arithmetic(ops, sub_checked)? },
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
            BURN => gsd += pu16!() as i64,         
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
        *gas_usable -= gas_table.gas(instbyte) + gsd;
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