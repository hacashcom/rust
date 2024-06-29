

pub struct TestExtActCaller {

}

impl TestExtActCaller {
    pub fn new() -> TestExtActCaller {
        TestExtActCaller{}
    }
}


impl ExtActCaller for TestExtActCaller {
    fn call(&mut self, kind: u16,_: &[u8]) -> Result<(i64, Vec<u8>), Error> {
        Ok((10, vec![1]))
    }
}