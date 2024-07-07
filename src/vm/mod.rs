
use std::sync::{ Mutex, Arc };

use crate::protocol::transaction::DynListAction;
use crate::sys::*;
use crate::interface::protocol::*;
use crate::interface::chain::*;
use crate::interface::vm::*;



#[macro_use]
pub mod rt;
pub mod ir;
pub mod value;
pub mod stack;
pub mod frame;
pub mod interpreter;
pub mod native;
pub mod contract;
pub mod machine;

use rt::*;
use contract::*;
use machine::*;


lazy_static! {
    static ref CONTRACT_LOADER: Arc<Mutex<ContractLoader>> = Arc::new(Mutex::new(
        ContractLoader::new(&SpaceCap::new())
    ));
}


pub fn code_loader() -> Arc<Mutex<ContractLoader>> {
    CONTRACT_LOADER.clone()
}


pub fn boot_machine<'a>(gas: i64,
    extn_caller: &'a mut dyn ExtActCaller,
    out_storage: &'a mut dyn OutStorager,
    out_storage_read: &'a mut dyn OutStoragerRead,
) -> Machine<'a> {

    machine::Machine::new(code_loader(), gas, 
        extn_caller, out_storage, out_storage_read)
}


















































/*

pub struct HacashVM {
    store: Arc<dyn Store>,
}




impl VM for HacashVM {

    fn new(ini: &IniObj, sto: Arc<dyn Store>) -> HacashVM {
        HacashVM{
            store: sto,
        }
    }

    fn exec(&self, ctx: &dyn ExecContext, bst: &mut dyn State, con: &Vec<Box<dyn Action>>) -> RetErr {
        do_exec(ctx, bst, self.store.as_ref(), con)
    }

}



fn do_exec(ctx: &dyn ExecContext, bst: &mut dyn State, sto: &dyn Store, actlist: &Vec<Box<dyn Action>>) -> RetErr {
    
    for act in actlist {

        // ext action
        if act.kind() > 0 {
            // exec
            let res = act.execute(ctx, bst, sto);
            if let Some(abort_err) = res.abort() {
                return Err(abort_err.clone()) // abort error
            }
        }else{
            let kd = act.kind();
            return errf!("cannot exec action bykind {}", kd)
        }

    }
    
    // ok finish
    Ok(())
}


*/