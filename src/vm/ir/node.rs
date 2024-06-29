

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

pub struct IRNodeExtAction {
    pub code: Bytecode,
    pub kind: u16,
    pub body: Vec<u8>,
}


impl IRNode for IRNodeExtAction {
    fn bytecode(&self) -> u8 {
        self.code as u8
    }
    fn codegen(&self) -> Vec<u8> {
        let mut codes = vec![];
        let mut bdbts = self.body.clone();
        let bdlen = bdbts.len();
        if bdlen == 0 {
            codes.push(Bytecode::PUSHNBUF as u8); // push buf empth
        } else if bdlen <= 256 {
            codes.push(Bytecode::PUSHBUF as u8); // push buf
            codes.push(bdlen as u8 - 1);
        } else if bdlen <= 65536 {
            codes.push(Bytecode::PUSHBUFL as u8); // push buf long
            codes.append(&mut (bdlen as u16 - 1).to_be_bytes().to_vec());
        } else {
            panic!("{}", "IRNodeExtAction codegen: ext action length too long")
        }
        codes.append( &mut bdbts );
        let mut kindbts = self.kind.to_be_bytes().to_vec();
        codes.append( &mut kindbts );
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
        let preres = compile_double(self.code, &self.subx, &self.suby);
        if let Some(codes) = preres {
            return codes
        }
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
        let preres = compile_triple(self.code, &self.subx, &self.suby, &self.subz);
        if let Some(codes) = preres {
            return codes
        }
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
            codes.push(POP as u8); // pop
        }
        if codes.len() > 0 {
            codes.pop(); // drop tail pop
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