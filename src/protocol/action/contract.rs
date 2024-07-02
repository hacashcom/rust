
/**
 * deploy contract
 */
 ActionDefine!{
    ContractDeploy : 35, (
        from : Address
    ),
    ACTLV_TOP_ONLY, // level
    11, // gas = 32
    (self, env, state, store), // params
    true, // burn 90
    [], // req sign
    { 
        ActExecRes::wrap(Ok(()))
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
    (self, env, state, store), // params
    true, // burn 90
    [], // req sign
    { 
        ActExecRes::wrap(Ok(()))
    }
}


