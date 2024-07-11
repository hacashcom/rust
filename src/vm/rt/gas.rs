

pub struct GasTable([u8; 256]);


impl GasTable {
    pub fn new() -> GasTable {
        use Bytecode::*;
        // baseline gas = 2
        let mut gtb: [u8; 256] = vec![2].repeat(256).try_into().unwrap();
        // 1
        for i in [
            CASTU8, CASTU16, CASTU32, CASTU64, CASTU128, CASTBUF,
        ] {
            gtb[i as usize] = 1;
        }
        // ok
        GasTable(gtb)
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


