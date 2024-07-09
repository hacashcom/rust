
/**
 * deploy contract
 */
 ActionDefine!{
    ContractDeploy : 35, (
        from : Address
    ),
    ACTLV_TOP_ONLY, // level
    11, // gas = 32
    (self, ctx, state, store, gas), // params
    true, // burn 90
    [], // req sign
    { 
        Ok(vec![])
    }
}



/**
 * upgrade contract
 */
 ActionDefine!{
    ContractUpgrade : 36, (
        addr : Address
    ),
    ACTLV_TOP_ONLY, // level
    11, // gas = 32
    (self, ctx, state, store, gas), // params
    true, // burn 90
    [], // req sign
    {
        vm::code_loader().lock().unwrap().clear_upgraded(
            &vm::rt::address_to_contract(&self.addr)
        );
        Ok(vec![])
    }
}


