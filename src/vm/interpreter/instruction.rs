use crate::vm::interpreter::ItrErrCode::*;
use crate::vm::value::StackItem::*;

fn check_failed_tip(op: &str, x: &StackItem, y: &StackItem) -> String {
    format!("arithmetic {} check failed with {:?} and {:?}", op, x, y)
}

fn add_checked(x: &StackItem, y: &StackItem) -> VmrtRes<StackItem> {
    match (x, y) {
        (U8(l), U8(r)) => u8::checked_add(*l, *r).map(StackItem::U8),
        (U16(l), U16(r)) => u16::checked_add(*l, *r).map(StackItem::U16),
        (U32(l), U32(r)) => u32::checked_add(*l, *r).map(StackItem::U32),
        (U64(l), U64(r)) => u64::checked_add(*l, *r).map(StackItem::U64),
        (U128(l), U128(r)) => u128::checked_add(*l, *r).map(StackItem::U128),
        (l, r) => return Err(ItrErr::new(Arithmetic, 
            &format!("cannot do arithmetic between {:?} and {:?}", x, y))),
    }
    .ok_or_else(||ItrErr::new(Arithmetic, &check_failed_tip("add", x, y)))
}



