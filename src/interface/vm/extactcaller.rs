
pub trait ExtActCaller {
    // fn cutout(&self, _: &[u8]) -> Result<Vec<u8>, Error>;
    fn call(&mut self, kind: u16,_: &[u8]) -> Result<(i64, Vec<u8>), Error>;
}



pub trait OutStorager {
    // forever store
    fn read(key: Vec<u8>) -> Option<Vec<u8>>;
    // return: gas use
    fn set(key: Vec<u8>, value: Vec<u8>) -> Result<i64, Error>;
    fn get(key: Vec<u8>) -> Option<Vec<u8>>;
    fn exp(key: Vec<u8>, tcc: usize) -> Result<i64, Error>;
    fn del(key: Vec<u8>) -> Result<i64, Error>; // refund gas
}

