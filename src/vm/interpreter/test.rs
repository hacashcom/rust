

pub struct TestExtActCaller {

}

impl TestExtActCaller {
    pub fn new() -> TestExtActCaller {
        TestExtActCaller{}
    }
}


impl ExtActCaller for TestExtActCaller {
    fn call(&self, kind: u16,_: &[u8]) -> Result<(i64, Vec<u8>), Error> {
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


impl OutStorager for TestOutStorager {
    fn read(&self, key: Vec<u8>) -> Option<Vec<u8>>
    {
        Some(vec![1,0,0,1])
    }
}