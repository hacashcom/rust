

fn convert_to_bytecodes(extact: &dyn ExtActCaller, func: &impl ContractFunction) -> VmrtRes<Vec<u8>> {
    match func.code_type() {
        CodeType::Bytecode => Ok(func.code_data().to_vec()),
        CodeType::AST => compile_irs_to_bytecodes(extact, func.code_data()),
        _ => itr_err_code!(CodeTypeError),
    }
}