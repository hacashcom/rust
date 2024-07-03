use lazy_static::lazy_static;


pub type ExtendActionsTryCreateFunc= fn(u16, &[u8]) -> Ret<Option<(Box<dyn Action>, usize)>>;

pub static mut EXTEND_ACTIONS_TRY_CREATE_FUNC: Option<ExtendActionsTryCreateFunc> = None;


pub fn create(buf: &[u8]) -> Ret<(Box<dyn Action>, usize)> {

    let kid = cut_kind(buf)?;
    let mut hasact = try_create(kid, buf)?;
    if let None = hasact {
        unsafe{
        if let Some(func) = EXTEND_ACTIONS_TRY_CREATE_FUNC {
            hasact = func(kid, buf)?;
        }
        }
    }
    match hasact {
        Some(res) => Ok(res),
        None => Err(format!("Action Kind <{}> not find.", kid))
    }

}



/******* pubFnRegActions ********/


macro_rules! pubFnRegActionCreateCommonEx {
    ( $trycreatefn:ident, $createfn:ident, $retty:ident, $($ty:ident)+ ) => {

        pub fn $trycreatefn(kind: u16, buf: &[u8]) -> Ret<Option<(Box<dyn $retty>, usize)>> {
            $(   
            if kind == <$ty>::kid() {
                let (act, sk) = <$ty>::create(buf)?;
                return Ok(Some((Box::new(act), sk)))
            }
            )+
            Ok(None)
        }

    }
}


/******* Create Func Define ********/


macro_rules! pubFnRegActionCreates {
    ( $($ty:ident)+ ) => {

        pub fn cut_kind(buf: &[u8]) -> Ret<u16> {
            let mut kind = Uint2::default();
            kind.parse(buf, 0)?;
            let kid = kind.to_u16();
            Ok(kid)
        }

        pubFnRegActionCreateCommonEx!{
            try_create, create, Action, $($ty)+
        }
    }
}


#[macro_export]
macro_rules! pubFnRegExtendActionCreates {
    ( $($ty:ident)+ ) => {
        pubFnRegActionCreates!{ $($ty)+ }


    }
}


/******* ActionDefine ********/

#[macro_export]
macro_rules! ActionDefineWithStruct {
    {   $actname:ident : $actid:expr, 
        $lv:expr, $gas:expr,
        ($p_self:ident, $p_env:ident, $p_state:ident, $p_store:ident ), 
        $burn90:expr, $reqsign:expr, $exec:expr 
    } => {

// ACTION_KIND DEFINE
concat_idents!(ACTION_KIND_ID = ACTION_KIND_, $actid {
pub const ACTION_KIND_ID: u16 = $actid;
});


impl Action for $actname {
    fn kind(&self) -> u16 {
        $actid
    }
    fn gas(&self) -> i64 {
        $gas
    }
    fn level(&self) -> u8 {
        $lv
    }
    fn burn_90(&$p_self) -> bool { 
        $burn90
    }
    fn req_sign(&$p_self) -> HashSet<Address> {
        HashSet::from($reqsign)
    }
}

impl ActExec for $actname {
    fn execute(&$p_self, $p_env: &mut dyn ExecEnv, $p_state: &mut dyn State, $p_store: &dyn Store) -> Box<dyn ExecResult> {
        $exec
    }
}


impl $actname {

    pub fn kid() -> u16 {
        $actid
    }

    pub fn new() -> $actname {
        let mut obj = <$actname as Field>::new();
        obj.kind = Uint2::from($actid);
        obj
    }
} 



/*
pub struct ACTION_NAME {
    pub kind: Uint2,
}
 */





    }
}


#[macro_export]
macro_rules! ActionDefine {
    {   $actname:ident : $actid:expr, 
        ($( $item:ident : $type:ty )*), 
        $lv:expr, $gas:expr,
        ($p_self:ident, $p_env:ident, $p_state:ident, $p_store:ident ), 
        $burn90:expr, $reqsign:expr, $exec:expr 
    } => {

        StructFieldStruct!{ $actname ,
            kind: Uint2
            $( 
                $item : $type 
            )*
        }

        ActionDefineWithStruct!{
            $actname : $actid, 
            $lv, $gas,
            ($p_self, $p_env, $p_state, $p_store ), 
            $burn90, $reqsign, $exec 
        }

    }
}