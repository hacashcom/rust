use sha3::{Digest, Sha3_256};
use sha2::{Sha256};
use ripemd::{Ripemd160};

use super::rt::*;
use super::rt::ItrErrCode::*;
use super::value::*;

include!("hash.rs");
include!("native.rs");

