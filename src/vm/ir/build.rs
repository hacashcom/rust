



pub fn compile_irs_to_bytecodes(extact: &dyn ExtActCaller, bytes: &[u8]) -> VmrtRes<Vec<u8>> {
    let irs = parse_ir_block(extact, bytes, &mut 0)?;
    Ok(irs.codegen())
}
