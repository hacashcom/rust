

/*
    such as: v = x + y
*/
fn binop_arithmetic<F>(operand_stack: &mut Stack, f: F) -> VmrtErr
where
    F: FnOnce(&StackItem, &StackItem) -> VmrtRes<StackItem>
{
    let mut x = operand_stack.pop()?;
    let mut y = operand_stack.pop()?;
    cast_arithmetic(&mut x, &mut y)?;
    let v = f(&x, &y)?;
    operand_stack.push(v)
}









