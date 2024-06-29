


pub fn native_env(idx: u8) -> VmrtRes<StackItem> {
    match idx {
        _ => itr_err_fmt!(NativeCall, "notfind native env idx {}", idx),
    }
}



pub fn native_call(idx: u8, v: &StackItem) -> VmrtRes<StackItem> {
    
    match idx {
        1 => sha3(v),
        _ => itr_err_fmt!(NativeCall, "notfind native func idx {}", idx),
    }
}






