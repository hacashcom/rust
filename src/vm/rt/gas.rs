

pub struct GasTable([u8; 256]);


impl GasTable {
    pub fn new() -> GasTable {
        use Bytecode::*;
        // baseline gas = 2
        let mut gtb: [u8; 256] = vec![2].repeat(256).try_into().unwrap();
        // 1
        for i in [
            CASTU8, CASTU16, CASTU32, CASTU64, CASTU128, CASTBUF,
            PUSH0, PUSH1, PUSHU8, PUSHU16, POP, DUP, TYPE, SIZE, BYTE, 
            NOT, NOP, RET, ABT, END, NT, JMPL, JMPS, JMPSL,  
        ] { gtb[i as usize] = 1; }
        // 3
        for i in [
            MUL, DIV, MOD
        ] { gtb[i as usize] = 3; }
        // 4
        for i in [
            POW, PUT, HREAD, HREADU, HREADUL
        ] { gtb[i as usize] = 4; }
        // 8
        for i in [
            MGET, HWRITE, HWRITEX, HWRITEXL
        ] { gtb[i as usize] = 8; }
        // 16
        for i in [
            MPUT, GGET, 
        ] { gtb[i as usize] = 16; }
        // 32
        for i in [
            GPUT, 
        ] { gtb[i as usize] = 32; }
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


