

use crate::vm;
use crate::vm::*;
use crate::vm::rt::*;
use crate::vm::value::*;
use crate::vm::space::*;
use crate::vm::interpreter::*;
use crate::interface::vm::*;


pub fn main_test98237456289375() {

    let mut v1 = StackItem::U8(8);
    let mut v2 = StackItem::U64(64);
    let mut v3 = StackItem::Buffer(vec![1].repeat(7));
    let mut v4 = StackItem::Buffer(vec![0,1,2]);
    
    println!("{:?}", cast_arithmetic(&mut v4, &mut v3) );

    println!( "{:?}  {:?}  {:?}  {:?}", v1, v2, v3, v4 );

}


pub fn main_vm_machine_call(codes: &str) {

    let mut codes = hex::decode(codes).unwrap();

    let mut gas = 1000000i64;
    let mut extcaller = TestExtActCaller::new();
    let mut outstorer = TestOutStorager::new();
    let mut outstorerread = TestOutStorager::new();
    let mut machine = vm::machine::Machine::new( gas,&mut extcaller, &mut outstorer, &mut outstorerread, vm::code_loader());

    let adr1 = Address::min();
    println!("Address::min() = {}", &adr1.readable());

    let res = machine.do_main_call(&adr1, &codes);

    println!("vm machine call res = {:?}", res);
    machine.printdebug();
    
}


pub fn main_vm_machine_call_3746582364523() {

    main_vm_machine_call("4b21fbfbfbfb");

}



pub fn main_vm_frame_call_2834756283974() {

    // let mut codes = hex::decode("bf034b42be01bd0159").unwrap();
    let mut codes = hex::decode("4e03010101594A58414B804809805959f000").unwrap();

    let mut gas = 1000000i64;

    let adr1 = [1u8; 21];
    let adr2 = [1u8; 21];
    let iptv = StackItem::empty_buf();
    let mut frame = vm::frame::Frame::new(&SpaceCap::new(), None, adr1, adr2,
        CallMode::External, 0, codes, iptv);
    // do call
    let now = Instant::now();
    let mut extcaller = vm::interpreter::TestExtActCaller::new();
    let mut outstorer = vm::interpreter::TestOutStorager::new();
    let res = frame.unwrap().exec(
        &mut gas, &GasTable::new(), &GasExtra::new(), 
        &mut extcaller, &mut outstorer, 
        &mut AddrKVMap::new(20), &mut KVMap::new(20), false,
    ).call();
    println!("benchmark run time = {:?}", Instant::now().duration_since(now));

    println!("vm frame call res = {:?}", res);
}



pub fn main_vm_execute_89234765982374() {

    // let mut codes = hex::decode("4e03010101594A58414B804809805959f000").unwrap();
    let mut codes = hex::decode("bf034b42be01bd0159f002").unwrap();
    let gas_table = GasTable::new();
    let gas_extra = GasExtra::new();

    let gas_limit = 1576i64;
    let mut gas_usable = gas_limit;
    let mut operand_stack = space::Stack::new(256);
    let mut locals = space::Stack::new(256);

    let now = Instant::now();
    
    let mut lpnm: isize = 1;
    let mut res;
    loop {
        let ctxadr = new_contract_address();
        let mode = CallMode::External;
        let mut pc: usize = 0;
        let mut extcaller = vm::interpreter::TestExtActCaller::new();
        let mut outstorer = vm::interpreter::TestOutStorager::new();
        res = vm::interpreter::execute_code(&codes, &mut pc, &mode, 
            &mut gas_usable, &gas_table, &gas_extra, &mut extcaller, &mut outstorer,
            &mut operand_stack, &mut locals, &mut Heap::new(64),
            &mut AddrKVMap::new(20), &mut KVMap::new(20), &ctxadr, false, 0);
        lpnm -= 1;
        if lpnm <= 0 { break }
    }

    println!("benchmark run time = {:?}", Instant::now().duration_since(now));

    println!("vm execute_code res = {:?}", res);

    println!("operand_stack: {:?}, locals: {:?} gas use: {}", operand_stack, locals, gas_limit - gas_usable);





}


