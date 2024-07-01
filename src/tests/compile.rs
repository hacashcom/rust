




pub fn main_compile_2834756928374() {

    let extact = TestExtActCaller::new();

    // let irnodes = hex::decode("e1ed4aed4b4a").unwrap();
    let irnodes = hex::decode("e24be000024a4aed4b").unwrap();
    let bytecodes = ir::compile_irs_to_bytecodes(&extact, &irnodes).unwrap();

    let codestr = hex::encode(&bytecodes);
    println!("{}", codestr);

    main_vm_machine_call(&codestr);



}












