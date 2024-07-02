

#[derive(Default)]
pub struct ASTLeaf {
    code: u8
}

impl Serialize for ASTLeaf {
    fn serialize(&self) -> Vec<u8> {
        vec![self.code]
    }
    fn size(&self) -> usize {
        1
    }
}


impl Parse for ASTLeaf {
    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        if buf.len() < 1 {
            return err_buf_short!()
        }
        self.code = buf[0];
        Ok(seek+1)
    }    
}


impl Field for ASTLeaf {

    
}


impl ASTLeaf {

    fn from(bt: u8) -> ASTLeaf {
        ASTLeaf{
            code: bt,
        }
    }

}

