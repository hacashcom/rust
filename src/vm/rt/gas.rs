

pub struct GasTable([u8; 256]);


impl GasTable {
    pub fn new() -> GasTable {
        GasTable(vec![2].repeat(256).try_into().unwrap())
    }

    #[inline(always)]
    pub fn gas(&self, code: u8) -> i64 {
        self.0[code as usize] as i64
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


