
use crate::vm;
use crate::vm::*;
use crate::vm::value::*;


pub fn main_test98237456289375() {

    let mut v1 = StackItem::U8(8);
    let mut v2 = StackItem::U64(64);
    let mut v3 = StackItem::Buffer(vec![1].repeat(7));
    let mut v4 = StackItem::Buffer(vec![0,1,2]);
    
    println!("{:?}", cast_arithmetic(&mut v4, &mut v3) );

    println!( "{:?}  {:?}  {:?}  {:?}", v1, v2, v3, v4 );

}


pub fn main_vm_execute_89234765982374() {

    let codes = hex::decode("4A4B80480980").unwrap();
    let gas_table = vec![1].repeat(256);

    let mut gas_usable = 10000i64;
    let mut operand_stack = stack::Stack::new(256);
    let mut locals = stack::Stack::new(256);

    let res = vm::interpreter::execute_code(&codes, &gas_table, 
        &mut gas_usable, &mut operand_stack, &mut locals );

    println!("vm execute_code res = {:?}", res);

    println!("operand_stack: {:?}", operand_stack);





}


