



/**
* parse ir block
*/
pub fn parse_ir(stuff: &[u8], seek: &mut usize) -> VmrtRes<IRNodeBlock> {
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
    // mv sk
    *seek += 1;
    let insbyte = stuff[*seek];// u8
    let inst: Bytecode = unsafe { std::mem::transmute(insbyte) };
    // parse
    let mut irnode: Box<dyn IRNode>;
    irnode = match inst {
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
        CASTU8
        | CASTU16 => {
            Box::new(IRNodeSingle{
                code: inst,
                subx: parse_ir_node_must(stuff, seek)?,
            })
        }
        // leaf
        NT
        | NOP
        | END
        | DUP
        | POP
        | PUSH0
        | PUSH1 => {
            Box::new(IRNodeLeaf{code: inst})
        }
        // inst invalid
        _ => return itr_err_code!(InstInvalid),
    };
    // ok
    Ok(irnode)
}