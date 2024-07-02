use std::any::Any;


fn bbb(ext: &dyn Any) -> bool {

    if ext.is::<String>() {
        return true
    }

    false
}
