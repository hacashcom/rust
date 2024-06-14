


#[derive(Debug)]
pub struct Stack {
    datas: Vec<StackItem>,
    limit: usize, // max len
}


impl Stack {

    pub fn new(lmt: usize) -> Stack {
        Stack {
            datas: vec![],
            limit: lmt,
        }
    }

    pub fn len(&self) -> usize {
        self.datas.len()
    }
        
}
