
#[derive(Debug, Default)]
pub struct Heap {
    limit: usize,
    memdt: Vec<u8>,
}


impl Heap {
    pub fn new(lmt: usize) -> Heap {
        Heap{
            limit: lmt,
            memdt: Vec::new(),
        }
    }
    
    pub fn set_limit(&mut self, lmt: usize) {
        self.limit = lmt;
    }

    pub fn clear(&mut self) {
        self.memdt.clear();
    }
}