use std::fs::{File};
use std::io::Read;

const BUF_SIZE: usize = 256;
pub fn read_file(path: &str) -> Result<Vec<u8>, String> {
    let mut file = err(File::open(path))?;
    let mut result = Vec::new();
    err(file.read_to_end(&mut result))?;
    
    Ok(result)
}

fn err<T, E: std::fmt::Display>(e: Result<T, E>) -> Result<T, String> {
    match e {
        Err(ee) => Err(format!("{}", ee)),
        Ok(t) => Ok(t)
    }
}

pub const FLOAT_SIZE: usize = 4;
pub const UINT_SIZE: usize = 4;

// Assumes bytes.len() >= 4. If not, you're shit outta luck
pub fn read_float(bytes: &[u8]) -> f32 {
    let mut buf = [0; FLOAT_SIZE];
    buf.clone_from_slice(bytes);

    unsafe {
        return std::mem::transmute::<[u8; FLOAT_SIZE],f32>(buf)
    }
}

pub fn read_uint(bytes: &[u8]) -> u32 {
    let mut buf = [0; UINT_SIZE];
    buf.clone_from_slice(bytes);
    unsafe {
        std::mem::transmute::<[u8; FLOAT_SIZE], u32>(buf)
    }
}