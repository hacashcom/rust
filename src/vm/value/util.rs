
use std::*;
use num_traits::FromBytes;

fn buf_is_not_zero(buf: &[u8]) -> bool {
    if buf.len() == 0 {
        return false // empty is zero
    }
    for a in buf {
        if *a != 0 {
            return true
        }
    }
    false
}

fn buf_drop_left_zero(buf: &[u8]) -> Vec<u8> {
    let n = buf.len();
    if n == 0 {
        return vec![]
    }
    let mut l = 0;
    for i in 0..n {
        l = i;
        if buf[i] != 0 {
            break
        }
    }
    // ok
    buf[l..].into()
}

fn buf_fill_left_zero(buf: &[u8], zn: usize) -> Vec<u8> {
    let res = buf[..].into();
    let sz = buf.len();
    if sz >= zn {
        return res
    }
    let pdn = zn - sz;
    [vec![0].repeat(pdn), res].concat()
}

