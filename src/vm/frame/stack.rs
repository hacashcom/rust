
#[derive(Debug)]
pub struct CallStack {
    frames: Vec<Frame>,
}


impl CallStack {

    pub fn new() -> CallStack{
        CallStack {
            frames: vec![],
        }
    }


    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn pop(&mut self) -> Option<Frame> {
        self.frames.pop()
    }

    pub fn push(&mut self, frame: Frame) -> VmrtErr {
        self.frames.push(frame);
        Ok(())
    }

    pub fn unpkg(self) -> Vec<Frame> {
        self.frames
    }
    



}

