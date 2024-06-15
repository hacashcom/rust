



impl CallFrameExec<'_> {

    pub fn call(&mut self) -> VmrtRes<StackItem> {

        // test
        let gas_limit = 1000000000000i64;
        let mut gas_usable = gas_limit;
        let gas_table = vec![1].repeat(256);

        let result = execute_code(
            &self.codes,
            &gas_table,
            &mut gas_usable,
            self.pc,
            self.local,
            self.stack,
        )?;
        Ok( StackItem::U8(0) )
    }

}