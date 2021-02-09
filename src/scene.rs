use super::util::*;
use super::Object;
use super::bsdf::BSDF;
use super::face::Face;

use std::fs::File;
use std::io::{Read, BufReader};

pub struct Scene {
    faces: Vec<Face>,
}


impl Object for Scene {
    // This reads like C code. Oh well.
    // Bad C code at that. oops.
    fn deserialize<R: Read>(r: &mut R) -> Result<Scene, String> {
        let version = u8::deserialize(r)?;

        if version != 0x1 {
            return Err("Invalid version byte".to_owned())
        }

        let num_vertices = u32::deserialize(r)?;
        let num_bsdfs = u32::deserialize(r)?;
        let num_faces = u32::deserialize(r)?;


        let mut vertices: Vec<Point3f> = Vec::with_capacity(num_vertices as usize);

        for _ in 0..num_vertices {
            vertices.push(Point3f::deserialize(r)?);
        }

        let vertices = vertices; // Remove mutability
        let mut bsdfs: Vec<BSDF> = Vec::with_capacity(num_bsdfs as usize);

        for _ in 0..num_bsdfs {
            let bsdf = BSDF::deserialize(r)?;
            bsdfs.push(bsdf)
        }

        let bsdfs = bsdfs;

        let mut faces: Vec<Point3<u32>> = Vec::with_capacity(num_faces as usize);
        for _ in 0..num_faces {
            let face = Point3u::deserialize(r)?;
            faces.push(face)
        }

        let faces = faces;

        // todo
        let faces_processed = Vec::with_capacity(faces.len());

        Ok(Scene{faces: faces_processed})
    }
}

impl Scene {
    pub fn read(path: &str) -> Result<Scene, String> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e))
        };
        let mut reader = BufReader::new(file);
        Scene::deserialize(&mut reader)
    }
}