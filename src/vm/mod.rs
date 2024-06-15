
use std::sync::{ Arc };

use crate::protocol::transaction::DynListVMAction;
use crate::sys::*;
use crate::interface::protocol::*;
use crate::interface::chain::*;
use crate::interface::vm::*;

#[macro_use]
pub mod rt;

mod ast;
mod space;

pub mod bytecode;
pub mod value;
pub mod stack;
pub mod frame;
pub mod interpreter;
pub mod action;



pub struct HacashVM {
    store: Arc<dyn Store>,
}




impl VM for HacashVM {

    fn new(ini: &IniObj, sto: Arc<dyn Store>) -> HacashVM {
        HacashVM{
            store: sto,
        }
    }

    fn exec(&self, env: &dyn ExecEnv, bst: &mut dyn State, con: &Vec<Box<dyn VMAction>>) -> RetErr {
        do_exec(env, bst, self.store.as_ref(), con)
    }

}



fn do_exec(env: &dyn ExecEnv, bst: &mut dyn State, sto: &dyn Store, actlist: &Vec<Box<dyn VMAction>>) -> RetErr {
    
    for act in actlist {

        // ext action
        if act.kind() > 0 {
            let extact = act.as_ext();
            // exec
            let res = extact.execute(env, bst, sto);
            if let Some(abort_err) = res.abort() {
                return Err(abort_err.clone()) // abort error
            }
        }else{
            let (cd, kd) = (act.code(), act.kind());
            return errf!("cannot exec action by code {} or kind {}", cd, kd)
        }

    }
    
    // ok finish
    Ok(())
}