// common fn

fn bytesfixed_to_uint(tip: &str, bts: &[u8], tsz: usize, len: usize) -> u64 {
    let sz = len;
    if sz > tsz || sz > 8 {
        panic!(tip.to_owned()+" size cannot over "+&tsz.to_string())
    }
    let mut vbts = [0u8; 8];
    let left = tsz - sz;
    let mut i = 0;
    for k in left..tsz {
        vbts[k] = bts[i];
        i += 1;
    }
    u64::from_be_bytes(vbts) 
}

fn bytesfixed_from_uint(tip: &str, val: u64, tsz: usize, len: usize) -> Vec<u8> {
    let sz = len;
    if sz > tsz {
        panic!(tip.to_owned()+" size cannot over "+&tsz.to_string())
    }
    let rlbt = val.to_be_bytes();
    let mut vbts = [0u8; 8];
    let left = tsz - sz;
    let mut i = 0;
    for k in left..tsz {
        vbts[i] = rlbt[k];
        i += 1;
    }
    vbts[0..sz].to_vec()
}

fn bytesfixed_to_readable_string(bts: &[u8]) -> String {
    let ss: Vec<u8> = bts.iter().map(|x|match x {
        32..=126 => *x,
        _ => ' ' as u8,
    }).collect();
    let resstr = String::from_utf8(ss).ok().unwrap();
    resstr.trim_end().to_string()
}

fn bytesfixed_from_readable_string(bts: &mut [u8], s: &String, len: usize) {
    let sz = len;
    let rs = s.clone().into_bytes();
    for i in 0.. s.len() {
        if i >= sz {
            break
        }
        bts[i] = rs[i];
    }
}

fn bytesfixed_from_hex(tip: &str, s: &String, len: usize) -> Result<Vec<u8>, Error> {
    let rsz = s.len();
    let tsz = len * 2;
    if rsz != tsz {
        return Err("FieldHex::".to_owned()+tip+" from_hex size error need "+&rsz.to_string()+" but got "+&rsz.to_string())
    }
    let bts = hex::decode(s);
    match bts {
        Ok(b) => Ok(b),
        Err(e) => Err(e.to_string()),
    }
}

/******************************/


// create BytesFixed macro
macro_rules! create_bytesfixed_struct_and_impl{
    ($tip:expr, $name:ident, $size:expr) => (


#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct $name {
    bytes: [u8; $size],
}


impl fmt::Display for $name{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"{}",self.to_hex())
    }
}

impl Index<usize> for $name {
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.bytes[idx]
    }
}

impl IndexMut<usize> for $name {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output{
        &mut self.bytes[idx]
    }
}

impl Deref for $name {
    type Target = [u8; $size];
    fn deref(&self) -> &[u8; $size] {
        &self.bytes
    }
}


impl Serialize for $name {

     fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
        let seek2 = parse_move_seek_or_buf_too_short_error!($tip, seek, $size, buf);
        let sv = &buf[seek..seek2];
        self.bytes = sv.try_into().unwrap();
        Ok(seek2)
    }

     fn serialize(&self) -> Vec<u8> {
        if $size != self.bytes.len() {
            panic!($tip.to_owned()+" serialize size not match.")
        }
        self.bytes.to_vec()
    }

     fn size(&self) -> usize {
        $size
    }

}

impl Describe for $name {

    fn describe(&self) -> String {
        "".to_string()
    }

    fn to_json(&self) -> String {
        "".to_string()
    }

    fn from_json(&mut self, _: &String) -> Option<Error> {
        None
    }

}

impl Field for $name {

    fn new() -> $name {
        $name{
            bytes: [0u8; $size],
        }
    }

    fn from_uint<T>(nt: T) -> Self where Self: Sized, T: std::ops::Add<u64, Output = u64> { 
        let num: u64 = nt + 0u64;
        let mut obj = <$name>::new();
        let sz = $size;
        if sz <= 1 && num < 256 {
            obj.from_u8(num as u8)
        }else if sz <= 2 && num < 65536 {
            obj.from_u16(num as u16)
        }else if sz <= 3 && num < 16777216 {
            obj.from_u32(num as u32)
        }else if sz <= 4 && num < 4294967296 {
            obj.from_u32(num as u32)
        }else if sz <= 8 {
            obj.from_u64(num as u64)
        }else {
            panic!("from_uint size cannot over 8")
        }
        obj
    }

    fn from_string(buf: &String) -> Self where Self: Sized {
        if buf.len() != $size {
            panic!("size error")
        }
        let bts = buf.clone().into_bytes().to_vec();
        // obj
        let mut obj = <$name>::new();
        let err = obj.from_vec_u8(&bts);
        if let Some(e) = err {
            panic!(e)
        }
        // ok
        obj
    }

}

impl FieldHex for $name {

    fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    fn from_hex(&mut self, s: &String) -> Option<Error> {
        let res = bytesfixed_from_hex($tip, s, $size);
        let bts = match res {
            Ok(b) => b,
            Err(e) => return Some(e),
        };
        self.bytes = bts.try_into().unwrap();
        None
    }

}

impl FieldBytes for $name {

    fn to_vec_u8(&self) -> Vec<u8> {
        self.serialize()
    }

    fn from_vec_u8(&mut self, buf: &Vec<u8>) -> Option<Error> {
        self.parse(buf, 0).err()
    }

}

impl FieldNumber for $name {

    fn to_u8(&self) -> u8 {
        let rv = bytesfixed_to_uint($tip, &self.bytes[..], 1, $size);
        rv as u8
    }

    fn from_u8(&mut self, val: u8) {
        let bts = bytesfixed_from_uint($tip, val as u64, 1, $size);
        self.bytes = bts.try_into().unwrap();
    }

    fn to_u16(&self) -> u16 {
        let rv = bytesfixed_to_uint($tip, &self.bytes[..], 2, $size);
        rv as u16
    }

    fn from_u16(&mut self, val: u16) {
        let bts = bytesfixed_from_uint($tip, val as u64, 2, $size);
        self.bytes = bts.try_into().unwrap();
    }

    fn to_u32(&self) -> u32 {
        let rv = bytesfixed_to_uint($tip, &self.bytes[..], 4, $size);
        rv as u32
    }

    fn from_u32(&mut self, val: u32) {
        let bts = bytesfixed_from_uint($tip, val as u64, 4, $size);
        self.bytes = bts.try_into().unwrap();
    }

    fn to_u64(&self) -> u64 {
        bytesfixed_to_uint($tip, &self.bytes[..], 8, $size)
    }

    fn from_u64(&mut self, val: u64) {
        let bts = bytesfixed_from_uint($tip, val, 8, $size);
        self.bytes = bts.try_into().unwrap();
    }

}

impl FieldReadableString for $name {
    
    fn to_readable_string(&self) -> String {
        bytesfixed_to_readable_string(&self.bytes[..])
    }
    
    fn from_readable_string(&mut self, s: &String) -> Option<Error> {
        bytesfixed_from_readable_string(&mut self.bytes[..], s, $size);
        None
    }

}

impl $name {

    const fn length() -> usize {
        $size
    }
}





    )
}



// create_bytesfixed_struct_and_impl!("BytesFixed4 ", BytesFixed4 ,  4usize);


// create 
create_bytesfixed_struct_and_impl!("BytesFixed1 ", BytesFixed1 ,  1usize);
create_bytesfixed_struct_and_impl!("BytesFixed2 ", BytesFixed2 ,  2usize);
create_bytesfixed_struct_and_impl!("BytesFixed3 ", BytesFixed3 ,  3usize);
create_bytesfixed_struct_and_impl!("BytesFixed4 ", BytesFixed4 ,  4usize);
create_bytesfixed_struct_and_impl!("BytesFixed5 ", BytesFixed5 ,  5usize);
create_bytesfixed_struct_and_impl!("BytesFixed6 ", BytesFixed6 ,  6usize);
create_bytesfixed_struct_and_impl!("BytesFixed8 ", BytesFixed8 ,  8usize);
create_bytesfixed_struct_and_impl!("BytesFixed10", BytesFixed10, 10usize);
create_bytesfixed_struct_and_impl!("BytesFixed12", BytesFixed12, 12usize);
create_bytesfixed_struct_and_impl!("BytesFixed14", BytesFixed14, 14usize);
create_bytesfixed_struct_and_impl!("BytesFixed15", BytesFixed15, 15usize);
create_bytesfixed_struct_and_impl!("BytesFixed16", BytesFixed16, 16usize);
create_bytesfixed_struct_and_impl!("BytesFixed17", BytesFixed17, 17usize);
create_bytesfixed_struct_and_impl!("BytesFixed18", BytesFixed18, 18usize);
create_bytesfixed_struct_and_impl!("BytesFixed21", BytesFixed21, 21usize);
create_bytesfixed_struct_and_impl!("BytesFixed24", BytesFixed24, 24usize);
create_bytesfixed_struct_and_impl!("BytesFixed32", BytesFixed32, 32usize);
create_bytesfixed_struct_and_impl!("BytesFixed33", BytesFixed33, 33usize);
create_bytesfixed_struct_and_impl!("BytesFixed64", BytesFixed64, 64usize);