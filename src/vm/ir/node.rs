

/*************************************/


pub struct IRNodeLeaf {
    pub code: Bytecode,
}

impl IRNode for IRNodeLeaf {
    fn bytecode(&self) -> u8 {
        self.code as u8
    }
    // fn parsing(&mut self, buf: &[u8], seek: &mut usize) -> RetErr {
    //     self.code = buf[*seek] ;
    //     *seek += 1;
    //     Ok(())
    // }
}


/*************************************/


pub struct IRNodeParams {
    pub code: Bytecode,
    pub para: Vec<u8>,
}

impl IRNode for IRNodeParams {
    fn bytecode(&self) -> u8 {
        self.code as u8
    }
    fn codegen(&self) -> Vec<u8> {
        let mut codes = vec![self.bytecode()];
        codes.append( &mut self.para.clone() );
        return codes
    }
}


/*************************************/


pub struct IRNodeSingle {
    pub code: Bytecode,
    pub subx: Box<dyn IRNode>,
}

impl IRNode for IRNodeSingle {
    fn bytecode(&self) -> u8 {
        self.code as u8
    }
    fn codegen(&self) -> Vec<u8> {
        let mut codes = self.subx.codegen(); // x
        codes.push(self.bytecode()); // code
        return codes
    }
}

pub struct IRNodeDouble {
    pub code: Bytecode,
    pub subx: Box<dyn IRNode>,
    pub suby: Box<dyn IRNode>,
}

impl IRNode for IRNodeDouble {
    fn bytecode(&self) -> u8 {
        self.code as u8
    }
    fn codegen(&self) -> Vec<u8> {
        let mut codes = self.subx.codegen(); // x
        codes.append( &mut self.suby.codegen() ); // y
        codes.push(self.bytecode()); // code
        return codes
    }
}

pub struct IRNodeTriple {
    pub code: Bytecode,
    pub subx: Box<dyn IRNode>,
    pub suby: Box<dyn IRNode>,
    pub subz: Box<dyn IRNode>,
}

impl IRNode for IRNodeTriple {
    fn bytecode(&self) -> u8 {
        self.code as u8
    }
    fn codegen(&self) -> Vec<u8> {
        let mut codes = self.subx.codegen(); // x 
        codes.append( &mut self.suby.codegen() ); // y
        codes.append( &mut self.subz.codegen() ); // z
        codes.push(self.bytecode()); // code
        return codes
    }
}


/*************************************/


pub struct IRNodeParaSingle {
    pub code: Bytecode,
    pub para: Vec<u8>,
    pub subx: Box<dyn IRNode>,
}

impl IRNode for IRNodeParaSingle {
    fn bytecode(&self) -> u8 {
        self.code as u8
    }
    fn codegen(&self) -> Vec<u8> {
        let mut codes = self.subx.codegen(); // x
        codes.push(self.bytecode());
        codes.append( &mut self.para.clone() );
        return codes
    }
}



/*************************************/



pub struct IRNodeBlock {
    pub subs: Vec<Box<dyn IRNode>>,
}

impl IRNode for IRNodeBlock {
    fn bytecode(&self) -> u8 {
        0
    }
    fn codegen(&self) -> Vec<u8> {
        let mut codes = vec![];
        for sub in &self.subs {
            codes.append( &mut sub.codegen() );
        }
        return codes
    }
}

impl IRNodeBlock {
    // 
    fn new() -> IRNodeBlock {
        IRNodeBlock{
            subs: vec![],
        }
    }
    fn push(&mut self, sub: Box<dyn IRNode>) {
        self.subs.push(sub)
    }
}