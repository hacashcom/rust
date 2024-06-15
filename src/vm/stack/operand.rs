

/*
* max size u16 = 65536 
*/
impl Stack {

    pub fn peek<'a>(&'a mut self) -> VmrtRes<&'a mut StackItem> {
        let n = self.datas.len();
        if n <= 0 {
            return Err(ItrErr::new(StackError, "Read empty stack"))
        }
        Ok(unsafe { self.datas.get_unchecked_mut(n - 1) })
    }

    pub fn pop(&mut self) -> VmrtRes<StackItem> {
        self.datas.pop()
        .ok_or_else(||ItrErr::new(StackError, "Pop empty stack"))
    }

    pub fn push(&mut self, it: StackItem) -> VmrtErr {
        if self.datas.len() >= self.limit {
            return itr_err_code!(OutOfStack)
        }
        self.datas.push(it);
        Ok(())
    }

    pub fn alloc(&mut self, num: u8) -> VmrtErr {
        let osz = self.datas.len();
        let tsz = osz + num as usize;
        if tsz >= self.limit {
            return itr_err_code!(OutOfStack)
        }
        self.datas.resize(tsz, StackItem::nil());
        Ok(())
    }

    pub fn save(&mut self, it: StackItem, idx: u16) -> VmrtErr {
        let idx = idx as usize;
        if idx >= self.datas.len() {
            return Err(ItrErr::new(LocalError, "Save local overflow"))
        }
        self.datas[idx] = it;
        Ok(())
    }

    pub fn load(&self, idx: u16) -> VmrtRes<StackItem> {
        let idx = idx as usize;
        if idx >= self.datas.len() {
            return Err(ItrErr::new(LocalError, "Read local overflow"))
        }
        Ok(self.datas[idx].clone())
    }
    
    pub fn last(&self) -> VmrtRes<StackItem> {
        self.lastn(0)
    }

    pub fn lastn(&self, n: u16) -> VmrtRes<StackItem> {
        let n = n as usize;
        let l = self.datas.len();
        if n >= l {
            return Err(ItrErr::new(StackError, "Read stack overflow"))
        }
        Ok(self.datas[l-n-1].clone())
    }


}