use crate::vm::interpreter::ItrErrCode::*;
use crate::vm::value::StackItem::*;

fn check_failed_tip(op: &str, x: &StackItem, y: &StackItem) -> String {
    format!("arithmetic {} check failed with {:?} and {:?}", op, x, y)
}

/////////////////////// logic ///////////////////////


macro_rules! lgcv {
    ($v: expr) => {
        Ok(U8( match $v { true => 1, false => 0 } ))
    }
}

macro_rules! lgcdo {
    ($op: ident, $l: expr, $r: expr, $t: ty) => {
        lgcv!( (*$l as $t).$op(&(*$r as $t)) )
    }
}

macro_rules! lgcmatch {
    ($op: ident, $x: expr, $y: expr) => {
        match ($x, $y) {
            (U8(l), U8(r)) =>     lgcdo!($op, l, r, u8),
            (U8(l), U16(r)) =>    lgcdo!($op, l, r, u16),
            (U8(l), U32(r)) =>    lgcdo!($op, l, r, u32),
            (U8(l), U64(r)) =>    lgcdo!($op, l, r, u64),
            (U8(l), U128(r)) =>   lgcdo!($op, l, r, u128),

            (U16(l), U8(r)) =>    lgcdo!($op, l, r, u16),
            (U16(l), U16(r)) =>   lgcdo!($op, l, r, u16),
            (U16(l), U32(r)) =>   lgcdo!($op, l, r, u32),
            (U16(l), U64(r)) =>   lgcdo!($op, l, r, u64),
            (U16(l), U128(r)) =>  lgcdo!($op, l, r, u128),

            (U32(l), U8(r)) =>    lgcdo!($op, l, r, u32),
            (U32(l), U16(r)) =>   lgcdo!($op, l, r, u32),
            (U32(l), U32(r)) =>   lgcdo!($op, l, r, u32),
            (U32(l), U64(r)) =>   lgcdo!($op, l, r, u64),
            (U32(l), U128(r)) =>  lgcdo!($op, l, r, u128),

            (U64(l), U8(r)) =>    lgcdo!($op, l, r, u64),
            (U64(l), U16(r)) =>   lgcdo!($op, l, r, u64),
            (U64(l), U32(r)) =>   lgcdo!($op, l, r, u64),
            (U64(l), U64(r)) =>   lgcdo!($op, l, r, u64),
            (U64(l), U128(r)) =>  lgcdo!($op, l, r, u128),

            (U128(l), U8(r)) =>    lgcdo!($op, l, r, u128),
            (U128(l), U16(r)) =>   lgcdo!($op, l, r, u128),
            (U128(l), U32(r)) =>   lgcdo!($op, l, r, u128),
            (U128(l), U64(r)) =>   lgcdo!($op, l, r, u128),
            (U128(l), U128(r)) =>  lgcdo!($op, l, r, u128),

            (l, r) => return itr_err_fmt!(Arithmetic, 
                "cannot do logic operand between {:?} and {:?}", $x, $y),
        }
    }
}


fn lgc_equal(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    match (x, y) {
        (Buffer(l), Buffer(r)) => lgcv!(l.eq(r)) ,
        _ => lgcmatch!(eq, x, y)
    }
}

fn lgc_not_equal(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    match (x, y) {
        (Buffer(l), Buffer(r)) => lgcv!(l.ne(r)) ,
        _ => lgcmatch!(ne, x, y)
    }
}

fn lgc_lt(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    lgcmatch!(lt, x, y)
}

fn lgc_le(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    lgcmatch!(le, x, y)
}

fn lgc_gt(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    lgcmatch!(gt, x, y)
}

fn lgc_ge(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    lgcmatch!(ge, x, y)
}

fn lgc_and(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    let v = match (x.to_bool(), y.to_bool()) {
        (true, true) => U8(1),// true
        _ => U8(0), // false
    };
    Ok(v)
}

fn lgc_or(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    let v = match (x.to_bool(), y.to_bool()) {
        (false, false) => U8(0),// false
        _ => U8(1), // true
    };
    Ok(v)
}


/////////////////////// arithmetic ///////////////////////

macro_rules! ahmtdo {
    ( $x: expr, $y: expr, $op: ident ) => {
        match ($x, $y) {
            (U8(l), U8(r)) => <u8>::$op(*l, *r).map(StackItem::U8),
            (U16(l), U16(r)) => <u16>::$op(*l, *r).map(StackItem::U16),
            (U32(l), U32(r)) => <u32>::$op(*l, *r).map(StackItem::U32),
            (U64(l), U64(r)) => <u64>::$op(*l, *r).map(StackItem::U64),
            (U128(l), U128(r)) => <u128>::$op(*l, *r).map(StackItem::U128),
            (_, _) => return itr_err_fmt!(Arithmetic, 
                "cannot do arithmetic between {:?} and {:?}", $x, $y),
        }
    }
}

macro_rules! ahmtdocheck {
    ( $x: expr, $y: expr, $op: ident, $tip: expr ) => {
        ahmtdo!($x, $y, $op)
        .ok_or_else(||ItrErr::new(Arithmetic, &check_failed_tip($tip, $x, $y)))
    }
}


fn add_checked(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    ahmtdocheck!(x, y, checked_add, "add")
}

fn sub_checked(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    ahmtdocheck!(x, y, checked_sub, "sub")
}

fn mul_checked(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    ahmtdocheck!(x, y, checked_mul, "mul")
}

fn div_checked(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    ahmtdocheck!(x, y, checked_div, "div")
}

fn mod_checked(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    ahmtdocheck!(x, y, checked_rem, "mod") // rem = mod
}

// the value is must within u32
fn pow_checked(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    match (x, y) {
        (U8(l), U8(r)) => <u8>::checked_pow(*l, *r as u32).map(StackItem::U8),
        (U16(l), U16(r)) => <u16>::checked_pow(*l, *r as u32).map(StackItem::U16),
        (U32(l), U32(r)) => <u32>::checked_pow(*l, *r).map(StackItem::U32),
        (_, _) => return itr_err_fmt!(Arithmetic, 
            "cannot do pow arithmetic between {:?} and {:?}", x, y),
    }.ok_or_else(||ItrErr::new(Arithmetic, &check_failed_tip("pow", x, y)))
}



