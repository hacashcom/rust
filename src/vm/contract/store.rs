



// Contract address list
StructFieldList!(ContractAddrsssList, count, Uint1, addrs, Address);

// Contract Head
StructFieldStruct!{ ContractHead, 
    vrsn: Fixed1 // 2bit4=version, 6bit, 
	marks: Fixed5
	inherits: ContractAddrsssList
    librarys: ContractAddrsssList
	mexts: Fixed2
}

// Contract System Call
StructFieldStruct!{ ContractSystemCall, 
    mark: Fixed1
    vrsn: Fixed1 // 5bit, 3bit8=codetype
	sign: Fixed1
    code: BytesW2
}

// Contract User Func
StructFieldStruct!{ ContractClientFunc, 
    mark: Fixed3
    vrsn: Fixed1 // 5bit, 3bit8=codetype
	sign: Fixed4
    code: BytesW2
}

// Func List
StructFieldList!(ContractSystemCallList, fnums, Uint1, funcs, ContractSystemCall);
StructFieldList!(ContractClientFuncList, fnums, Uint2, funcs, ContractClientFunc);


//////////////////////////////////////


// Contract
StructFieldStruct!{ ContractStorage, 
	contlhead: ContractHead
	sytmcalls: ContractSystemCallList
	userfuncs: ContractClientFuncList
    morextend: Fixed2
}



