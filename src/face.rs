use crate::util::*;
use crate::bsdf::BSDF;
#[derive(Debug)]
pub struct Face {
    pub normal: Vec3f,
    pub vertices: [Point3f; 3],
    pub bsdf_idx: usize
}

impl Face {
    pub fn new(vertices: [Point3f; 3], bsdf_idx: usize) -> Face {
        let ab = dbg!(vertices[1] - vertices[0]);
        let ac = dbg!(vertices[2] - vertices[0]);
        let normal = ab.cross(&ac).normalised();
        Face{normal, vertices, bsdf_idx}
    }
}