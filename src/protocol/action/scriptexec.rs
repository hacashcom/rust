



/**
 * execute script
 */
 ActionDefine!{
    ScriptExecute : 37, (
        mark: Fixed1
        vern: Fixed1
        codes: BytesW2
    ),
    ACTLV_TOP, // level
    11, // gas = 32
    (self, env, state, store), // params
    true, // burn 90
    [], // req sign
    { 
        ActExecRes::wrap(Ok(()))
    }
}



