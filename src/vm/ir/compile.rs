
use Bytecode::*;

type IRNRef<'a> = &'a Box<dyn IRNode>;

// (u16::MAX/2 - jpl as u16).into();
const JMP_CODES_LEN: usize = 3; 
const BLOCK_CODES_MAX_LEN: usize = 65535 / 2 - JMP_CODES_LEN; 



fn compile_double(btcd: Bytecode, x: IRNRef, y: IRNRef) -> Option<Vec<u8>> {
    let res = match btcd {
        IR_WHILE => compile_while(x, y),
        _ => return None
    };
    Some(res)
}


fn compile_while(x: IRNRef, b: IRNRef) -> Vec<u8> {
    let jpl = JMP_CODES_LEN;
    //
    let mut cond = x.codegen();
    let mut body = b.codegen();
    let body_l = body.len() + 1 + jpl;
    let total_l = body_l + cond.len() + jpl;
    if total_l > BLOCK_CODES_MAX_LEN {
        panic!("compile codes too long")
    }
    // codes
    let mut codes = cond;
    codes.push(BRNSL as u8);
    codes.append( &mut (body_l as i16).to_be_bytes().to_vec());
    // body
    codes.append( &mut body );
    codes.push(POP as u8);
    codes.push(JMPSL as u8);
    codes.append( &mut (-(total_l as i16)).to_be_bytes().to_vec());
    // ok
    codes
}


/**************************************************/



fn compile_triple(btcd: Bytecode, x: IRNRef, y: IRNRef, z: IRNRef) -> Option<Vec<u8>> {
    let res = match btcd {
        IR_IF => compile_if(x, y, z),
        _ => return None
    };
    Some(res)
}


fn compile_if(x: IRNRef, a: IRNRef, b: IRNRef) -> Vec<u8> {
    let jpl = JMP_CODES_LEN;
    let maxl = BLOCK_CODES_MAX_LEN;
    let mut if_br = a.codegen();
    let mut else_br = b.codegen();
    let mut cond = x.codegen();
    let if_l = if_br.len();
    let else_l = else_br.len() + jpl;
    let cond_l = cond.len() + jpl;
    if if_l > maxl || else_l > maxl {
        panic!("compile codes too long")
    }
    // codes
    let mut codes = cond;
    codes.push(BRSL as u8);
    codes.append( &mut (else_l as i16).to_be_bytes().to_vec());
    // else br
    codes.append(&mut else_br); // else codes
    codes.push(JMPSL as u8);
    codes.append( &mut (if_l as i16).to_be_bytes().to_vec());
    // if br
    codes.append(&mut if_br);  // if codes
    // ok
    codes
}

