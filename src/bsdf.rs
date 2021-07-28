use super::util::*;
use super::Object;

use std::io::Read;

#[derive(Debug)]
pub enum BSDF {
    Mirror,
    Diffuse(ColF),
    Emitter(ColF)
}


impl Object for BSDF {
    fn deserialize<R: Read>(r: &mut R) -> Result<BSDF, String> {
        use BSDF::*;
        let mut kind = [0];
        r.read(&mut kind).map_err(|e| format!("{:?}", e))?;
        match kind[0] {
            1 => Ok(Mirror),
            2 => {
                let c = Colour::deserialize(r)?;
                Ok(Diffuse(c))
            }
            3 => {
                let c = Colour::deserialize(r)?;
                Ok(Emitter(c))
            }
            _ => Err(format!("Invalid first byte in deserialising BSDF"))
        }
    }
}