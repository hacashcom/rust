
trait ContractFunction {
    fn code_type(&self) -> CodeType;
    fn code_data(&self) -> &[u8];
}


macro_rules! impl_contract_function {
    ($class: ty) => {
        impl ContractFunction for $class {
            fn code_type(&self) -> CodeType {
                let ct = self.vrsn[0] & 0b00000011;
                unsafe_std_mem_transmute!(ct)
            }
            fn code_data(&self) -> &[u8] {
                self.code.as_ref()
            }
        }
        
    }
}


