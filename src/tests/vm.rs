

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



pub fn main_vm_frame_Call_2834756283974() {

    let mut codes = hex::decode("bf034b42be01bd0159").unwrap();

    let mut frame = vm::frame::CallFrame::make_new(
        &codes, StackItem::empty_buf());
    // do call
    let res = frame.exec().call();

    println!("vm frame call res = {:?}", res);
}



pub fn main_vm_execute_89234765982374() {

    // let mut codes = hex::decode("4e03010101594A58414B804809805959f000").unwrap();
    let mut codes = hex::decode("bf034b42be01bd0159f002").unwrap();
    let gas_table = vec![1].repeat(256);

    let gas_limit = 1576i64;
    let mut gas_usable = gas_limit;
    let mut operand_stack = stack::Stack::new(256);
    let mut locals = stack::Stack::new(256);

    let now = Instant::now();
    
    let mut lpnm: isize = 1;
    let mut res;
    loop {
        let mut pc: usize = 0;
        res = vm::interpreter::execute_code(&codes, &gas_table, 
            &mut gas_usable, &mut pc, &mut locals, &mut operand_stack);
        lpnm -= 1;
        if lpnm <= 0 { break }
    }

    println!("benchmark run time = {:?}", Instant::now().duration_since(now));

    println!("vm execute_code res = {:?}", res);

    println!("operand_stack: {:?}, locals: {:?} gas use: {}", operand_stack, locals, gas_limit - gas_usable);





}


