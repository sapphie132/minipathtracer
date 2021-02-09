use super::util::*;
use super::Object;

use std::io::Read;

pub enum BSDF {
    Mirror,
    Diffuse(Colour<f32>),
    Emitter(Colour<f32>)
}

impl Object for BSDF {
    fn deserialize<R: Read>(r: &mut R) -> Result<BSDF, String> {
        use BSDF::*;
        let mut kind = [0];
        r.read(&mut kind);
        match kind[0] {
            1 => Ok(Mirror),
            2 => {
                let c = Colour::deserialize(r)?;
                if c.0 <= 1. && c.1 <= 1. && c.2 <= 1. &&
                    c.0 + c.1 + c.2 <= 1. {
                    Ok(Diffuse(c))
                } else {
                    Err(format!("Deserialized albedo has invalid values {:?}", c))
                }
            }
            _ => Err(format!("Invalid first byte in deserialising BSDF"))
        }
    }
}