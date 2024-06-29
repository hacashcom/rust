



/**
* parse ir block
*/
pub fn parse_ir_list(stuff: &[u8], seek: &mut usize) -> VmrtRes<IRNodeBlock> {
    let codelen = stuff.len();
    if codelen > u16::MAX as usize {
        return itr_err_code!(CodeTooLong)
    }
    let mut block = IRNodeBlock::new();
    loop {
        let pres = parse_ir_node(stuff, seek)?;
        let Some(irnode) = pres else {
            break // end
        };
        block.push(irnode);
    }
    // finish
    Ok(block)
}



/**
* parse one node
*/
pub fn parse_ir_node(stuff: &[u8], seek: &mut usize) -> VmrtRes<Option<Box<dyn IRNode>>> {
    let codesz = stuff.len();
    if codesz == 0 || *seek >= codesz {
        return Ok(None) // finish end
    }
    Ok(Some(parse_ir_node_must(stuff, seek)?))
}

// must
pub fn parse_ir_node_must(stuff: &[u8], seek: &mut usize) -> VmrtRes<Box<dyn IRNode>> {

    let codesz = stuff.len();
    if codesz == 0 || *seek >= codesz {
        return itr_err_code!(CodeOverRun)
    }
    
    macro_rules! itrbuf {
        ($l: expr) => {
            {
                let _r = *seek + $l;
                if _r >= codesz {
                    return itr_err_code!(CodeOverRun)
                }
                let bts = stuff[*seek.._r].to_vec();
                *seek = _r;
                bts
            }
        }
    }
    // code
    let insbyte = stuff[*seek];// u8
    let inst: Bytecode = unsafe { std::mem::transmute(insbyte) };
    // parse
    let mut irnode: Box<dyn IRNode>;
    // mv sk
    *seek += 1;
    irnode = match inst {
        // block IF WHILE
        IR_BLOCK => {
            let mut block = IRNodeBlock::new();
            let p = itrbuf!(2);
            let n = u16::from_be_bytes(p.try_into().unwrap());
            for i in 0..n {
                block.push(parse_ir_node_must(stuff, seek)?);
            }
            Box::new(block)
        }
        IR_IF => {
            Box::new(IRNodeTriple{
                code: inst,
                subx: parse_ir_node_must(stuff, seek)?,
                suby: parse_ir_node_must(stuff, seek)?,
                subz: parse_ir_node_must(stuff, seek)?,
            })
        }
        IR_WHILE => {
            Box::new(IRNodeDouble{
                code: inst,
                subx: parse_ir_node_must(stuff, seek)?,
                suby: parse_ir_node_must(stuff, seek)?,
            })
        }
        // triple

        // double
        ADD
        | SUB => {
            Box::new(IRNodeDouble{
                code: inst,
                subx: parse_ir_node_must(stuff, seek)?,
                suby: parse_ir_node_must(stuff, seek)?,
            })
        }
        // single
        RET
        | ABT
        | CASTU8
        | CASTU16
        | CASTU32
        | CASTU64
        | CASTU128
        | CASTBUF => {
            Box::new(IRNodeSingle{
                code: inst,
                subx: parse_ir_node_must(stuff, seek)?,
            })
        }
        // leaf
        PUSH0
        | PUSH1 
        | PUSHNBUF => {
            Box::new(IRNodeLeaf{code: inst})
        }
        // inst invalid
        _ => return itr_err_code!(InstInvalid),
    };
    // ok
    Ok(irnode)
}