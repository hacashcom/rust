

pub type GasTable = [u8; 256];
pub struct GasTableW {}


impl GasTableW {
    pub fn new() -> GasTable {
        vec![2].repeat(256).try_into().unwrap()
    }
}



pub struct GasExtra {
    pub resource_local_item: i64,
}

impl GasExtra {
    pub fn new() -> GasExtra {
        GasExtra {
            resource_local_item: 6, // 
        }
    }
}


