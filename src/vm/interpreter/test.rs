

pub struct TestExtActCaller {

}

impl TestExtActCaller {
    pub fn new() -> TestExtActCaller {
        TestExtActCaller{}
    }
}


impl ExtActCaller for TestExtActCaller {

    fn call(&mut self, kb: Vec<u8>, depth: i8) -> Result<(i64, Vec<u8>), Error> {
        Ok((10, vec![1]))
    }

}


/////////////////////////////////////////


pub struct TestOutStorager {

}

impl TestOutStorager {
    pub fn new() -> TestOutStorager {
        TestOutStorager{}
    }
}


impl OutStoragerRead for TestOutStorager {
    fn get(&self, key: &[u8]) -> Ret<Option<Vec<u8>>> {
        Ok(Some(vec![1,0,0,1]))
    }
}

impl OutStorager for TestOutStorager {
    fn del(&mut self, key: &[u8]) -> RetErr {
        Ok(())
    }
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> RetErr {
        Ok(())
    }
}