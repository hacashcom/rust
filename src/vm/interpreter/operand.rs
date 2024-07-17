

/**
*   such as: v = x + y
*/
fn binop_arithmetic<F>(operand_stack: &mut Stack, f: F) -> VmrtErr
where
    F: FnOnce(&StackItem, &StackItem) -> VmrtRes<StackItem>
{
    let mut y = operand_stack.pop()?;
    let mut x = operand_stack.peek()?;
    cast_arithmetic(&mut x, &mut y)?;
    let v = f(&x, &y)?;
    *x = v;
    Ok(())
}


/**
*   binop_between
*   such as: v = x && y
*/
fn binop_btw<F>(operand_stack: &mut Stack, f: F) -> VmrtErr
where
    F: FnOnce(&StackItem, &StackItem) -> VmrtRes<StackItem>
{
    let mut y = operand_stack.pop()?;
    let mut x = operand_stack.peek()?;
    let v = f(&x, &y)?;
    *x = v;
    Ok(())
}








