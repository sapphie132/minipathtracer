use super::util::*;
use super::Object;
use super::bsdf::BSDF;
use super::face::Face;

use std::fs::File;
use std::io::{Read, BufReader};

#[derive(Debug)]
pub struct Scene {
    faces: Vec<Face>,
    bsdfs: Vec<BSDF>,
}


impl<'a> Object for Scene {
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

        let bsdfs = bsdfs; // Remove mutability

        // Each Point3u contains 3 indexes in the vertex array
        let mut faces: Vec<(Point3<u32>, u32)> = Vec::with_capacity(num_faces as usize);
        for _ in 0..num_faces {
            let face = Point3u::deserialize(r)?;
            let bsdf_idx = u32::deserialize(r)?;
            faces.push((face, bsdf_idx));
        }

        let faces = faces;

        let mut faces_processed = Vec::with_capacity(faces.len());

        for face in faces {
            let mut point_faces = [Point3f::new(); 3];
            for i in 0..3 {
                let idx = face.0.0[i] as usize;
                debug_assert!(idx < vertices.len());
                let face_point = vertices[idx];
                point_faces[i] = face_point;
            }
            let bsdf_idx = face.1;
            let face_processed = Face::new(point_faces, bsdf_idx as usize);
            faces_processed.push(face_processed);
        }

        Ok(Scene{faces: faces_processed, bsdfs})
    }
}

impl<'a> Scene {
    pub fn read(path: &str) -> Result<Scene, String> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e))
        };
        let mut reader = BufReader::new(file);
        Scene::deserialize(&mut reader)
    }
}

#[cfg(test)] 
mod test {
    use super::Object;
    use crate::util::{Point3, Vec3};
    #[test]
    fn name() {
        let f: Vec<u8> = vec!(0x1, // version
                              0x03, 0x00, 0x00, 0x00, // Num vertices
                              0x00, 0x00, 0x00, 0x00, // Num bsdfs
                              0x01, 0x00, 0x00, 0x00, // Num faces
                              0x00, 0x00, 0x00, 0x00, // Vertex 0, x
                              0x00, 0x00, 0x00, 0x00, // Vertex 0, y
                              0x00, 0x00, 0x00, 0x00, // Vertex 0, z
                              0x00, 0x00, 0x80, 0x3F, // Vertex 1, x
                              0x00, 0x00, 0x00, 0x00, // Vertex 1, y
                              0x00, 0x00, 0x00, 0x00, // Vertex 1, z
                              0x00, 0x00, 0x80, 0x3F, // Vertex 2, x
                              0x00, 0x00, 0x00, 0x40, // Vertex 2, y
                              0x00, 0x00, 0x00, 0x00, // Vertex 2, z
                              0x00, 0x00, 0x00, 0x00, // Face 0, x
                              0x01, 0x00, 0x00, 0x00, // Face 0, y
                              0x02, 0x00, 0x00, 0x00, // Face 0, z
                              0x00, 0x00, 0x00, 0x00, // Face 0, bsdf
        );

        let mut bytes: &[u8] = &f;
        let r = super::Scene::deserialize(&mut bytes);
        match r {
            Err(s) => {
                panic!("{}", s)
            }
            Ok(s) => {
                let faces = s.faces;
                debug_assert_eq!(faces.len(), 1);
                debug_assert_eq!(faces[0].vertices[0], Point3([0., 0., 0.]));
                debug_assert_eq!(faces[0].vertices[1], Point3([1., 0., 0.]));
                debug_assert_eq!(faces[0].vertices[2], Point3([1., 2., 0.]));
                debug_assert_eq!(faces[0].normal, Vec3([0., 0., 1.]));
            }
        }

    }
}