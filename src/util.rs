use std::io::Read;
use std::fmt::Debug;
use std::mem::{transmute, size_of};
use super::Object;


#[derive(Debug)]
pub struct Point3<T>(pub T, pub T, pub T);
pub type Point3f = Point3<f32>;
pub type Point3u = Point3<u32>;

impl<T: Object> Object for Point3<T> {
    fn deserialize<R: Read>(r: &mut R) -> Result<Point3<T>, String> {
        let (x,y,z) = read_3tuple(r)?;
        Ok(Point3(x,y,z))       
    }
}

#[derive(Debug)]
pub struct Vec3<T>(pub T, pub T, pub T);

#[derive(Debug)]
pub struct Colour<T: Debug>(pub T, pub T, pub T); // Kept generic for Colour<u8>
pub type ColF = Colour<f32>;

impl Object for ColF {
    fn deserialize<R: Read>(r: &mut R) -> Result<ColF, String> {
        let (x,y,z) = read_3tuple(r)?;
        Ok(Colour(x,y,z))
    }
}


const FLOAT_SIZE: usize = size_of::<f32>();
const UINT_SIZE: usize = size_of::<u32>();

fn read<R: Read>(r: &mut R, buf: &mut [u8]) -> Result<(), String> {
    match r.read(buf) {
        Ok(n) if n == buf.len() => Ok(()),
        Ok(_) => Err(format!("Encountered EOF before being able to read")),
        Err(e) => Err(format!("{}", e))
    }
}

impl Object for f32 {
    fn deserialize<R: Read>(r: &mut R) -> Result<f32, String> {
        let mut buf = [0u8; FLOAT_SIZE];
        read(r, &mut buf)?;
        unsafe {
            Ok(transmute(buf))
        }
    }
}


impl Object for u32 {
    fn deserialize<R: Read>(r: &mut R) -> Result<u32, String> {
        let mut buf = [0u8; UINT_SIZE];
        read(r, &mut buf)?;
        unsafe {
            Ok(transmute(buf))
        }
    }
}

// This feels silly, but in the long term it's convenient
impl Object for u8 {
    fn deserialize<R: Read>(r: &mut R) -> Result<u8, String> {
        let mut buf = [0u8];
        read(r, &mut buf)?;
        Ok(buf[0])
    }
}

pub fn read_3tuple<R: Read, T: Object>(r: &mut R) -> Result<(T, T, T), String> {
    let x = T::deserialize(r)?;
    let y = T::deserialize(r)?;
    let z = T::deserialize(r)?;
    Ok((x, y, z))
}
