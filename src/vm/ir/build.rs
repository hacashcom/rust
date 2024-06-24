



pub fn compile_irs_to_bytecodes(bytes: &[u8]) -> VmrtRes<Vec<u8>> {
    let irs = parse_ir_list(bytes, &mut 0)?;
    Ok(irs.codegen())
}
