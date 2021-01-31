pub struct Scene;
use super::util;

const SIZE_RESERVED: usize = 32;
const MAGIC_NUMBER: u16 = 0x420;
impl Scene {
    // This reads like C code. Oh well.
    // Bad C code at that. oops.
    pub fn read(path: &str) -> Result<Scene, String> {
        let bin = util::read_file(path)?;
        if bin.len() < SIZE_RESERVED {
            return Err("Invalid input file".to_owned())
        }

        let mut ptr = 0;

        if bin[ptr] != 0x1 {
            return Err("Invalid version byte".to_owned())
        }

        ptr+=1;

        fn read_float(p: &mut usize, b: &[u8]) -> f32 {
            let r = util::read_float(&b[*p..]);
            *p += util::FLOAT_SIZE;
            r
        }

        fn read_uint(p: &mut usize, b: &[u8]) -> u32 {
            let r = util::read_uint(&b[*p..]);
            *p += util::UINT_SIZE;
            r
        }

        let num_vertices = read_uint(&mut ptr, &bin);
        let mut vertices: Vec<(f32, f32, f32)> = Vec::with_capacity(num_vertices as usize);

        ptr = SIZE_RESERVED;

        for _ in 0..num_vertices {
            let mut xyz = [0.0; 3];
            for i in 0..3 {
                // Rust is a pain with closures
                xyz[i] = read_float(&mut ptr, &bin);
            }
            vertices.push((xyz[0], xyz[1], xyz[2]))
        }

        Ok(Scene)
    }
}