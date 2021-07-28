use std::ops::{Add, Sub, Mul, AddAssign, Div, DivAssign};
use std::io::Read;
use std::fmt::Debug;
use std::mem::{transmute, size_of};
use super::Object;
use crate::float;

const EPSILON: float = 1e-9;
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point3<T>(pub [T; 3]);

pub type Point3f = Point3<float>;
pub type Point3u = Point3<u32>;

impl<T: Copy> Point3<T> {
    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn z(&self) -> T {
        self.0[2]
    }
}

impl <T: Default + Copy> Point3<T> {
    pub fn new() -> Point3<T> {
        let underlying = [T::default(); 3];
        Point3(underlying)
    }
}

impl<'a, T: Copy> IntoIterator for &'a Point3<T> {
    type Item = T;
    type IntoIter = PointIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        PointIterator {
            point: self,
            index: 0,
        }
    }

}

pub struct PointIterator<'a, T> {
    point: &'a Point3<T>,
    index: usize
}

impl <'a, T: Copy> Iterator for PointIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.index < self.point.0.len() {
            self.index += 1;
            Some(self.point.0[self.index-1])
        } else {
            None
        }
    }
}

impl <U: Default + Copy, T: Sub<Output = U> + Copy> Sub for Point3<T> {
    type Output = Vec3<U>;
    fn sub(self, rhs: Point3<T>) -> Vec3<U> {
        let mut underlying = [U::default(); 3];
        for i in 0..3 {
            underlying[i] = self.0[i] - rhs.0[i];
        }
        Vec3(underlying)
    }
}

impl<T: Object> Object for Point3<T> {
    fn deserialize<R: Read>(r: &mut R) -> Result<Point3<T>, String> {
        let (x,y,z) = read_3tuple(r)?;
        let array = [x, y, z];
        Ok(Point3(array))       
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec3<T>(pub [T; 3]);
pub type Vec3f = Vec3<float>;

impl<T: DivAssign<float>> DivAssign<float> for Vec3<T> {
    fn div_assign(&mut self, rhs: float) {
        for e in &mut self.0 {
            *e /= rhs;
        }
    }
}

impl<T, U> Vec3<T>
where T: Mul<Output = U>  + Copy,
      U: Default + AddAssign + Sub<Output = U> + Copy {

    pub fn dot(&self, other: &Vec3<T>) -> U {
        let mut res = U::default();
        for i in 0..3 {
            res += self.0[i] * other.0[i]
        }
        res
    }


    pub fn cross(&self, other: &Vec3<T>) -> Vec3<U> {
        let mut res = [U::default(); 3];
        for i in 0..3 {
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;
            res[i] = self.0[j] * other.0[k] - self.0[k] * other.0[j]
        }
        Vec3(res)
    }
}

impl Vec3<float> {
    pub fn normalised(&self) -> Vec3f {
        let mut new = self.clone();
        new.normalise();
        new
    }

    pub fn normalise(&mut self) {
        let norm = self.dot(self).sqrt();
        if norm > EPSILON {
            *self /= norm
        } else {
            *self = Vec3([0.; 3])
        }
    }
}

#[derive(Debug)]
pub struct Colour<T: Debug>(pub T, pub T, pub T); // Kept generic for Colour<u8>
pub type ColF = Colour<float>;

impl Object for ColF {
    fn deserialize<R: Read>(r: &mut R) -> Result<ColF, String> {
        let (x,y,z) = read_3tuple(r)?;
        Ok(Colour(x,y,z))
    }
}

const FLOAT_SIZE: usize = size_of::<float>();
const UINT_SIZE: usize = size_of::<u32>();

fn read<R: Read>(r: &mut R, buf: &mut [u8]) -> Result<(), String> {
    match r.read(buf) {
        Ok(n) if n == buf.len() => Ok(()),
        Ok(_) => Err(format!("Encountered EOF before being able to read")),
        Err(e) => Err(format!("{}", e))
    }
}

impl Object for float {
    fn deserialize<R: Read>(r: &mut R) -> Result<float, String> {
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

pub trait VecArith<T> {
    fn _0(&self) -> T;
    fn _1(&self) -> T;
    fn _2(&self) -> T;
}
