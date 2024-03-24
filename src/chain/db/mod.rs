use std::sync::{Arc, Weak};
use std::collections::{ HashMap };
use std::cell::{ RefCell };
use std::path::Path;

use concat_idents::concat_idents;
// use rusty_leveldb::{DB as LevelDB, Options as LevelOptions, DBIterator, LdbIterator};


use crate::interface::field::*;
use crate::interface::chain::*;

use crate::sys::*;
use crate::base::field::*;
use crate::core::field::*;
use crate::core::component::*;

use super::util::*;

pub mod leveldb;

use leveldb::*;

// include!("macro.rs");
// include!("disk.rs");
include!("mem.rs");
include!("db.rs");
include!("store.rs");
// include!("state.rs");
// include!("def.rs");
