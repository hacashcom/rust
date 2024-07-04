
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


