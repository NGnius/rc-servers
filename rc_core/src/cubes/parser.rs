#![allow(dead_code)]

pub struct Cube {
    pub id: u32,
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub orientation: u8,
}

impl Cube {
    pub fn parse(r: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        let id = u32::from_le_bytes(buf);
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        Ok(Self {
            id,
            x: buf[0],
            y: buf[1],
            z: buf[2],
            orientation: buf[3],
        })
    }

    pub fn parse_list(r: &mut dyn std::io::Read) -> std::io::Result<Vec<Self>> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        let count = u32::from_le_bytes(buf);
        let mut cubes = Vec::with_capacity(count as _);
        for _ in 0..count {
            let cube = Self::parse(r)?;
            cubes.push(cube);
        }
        Ok(cubes)
    }
}

pub struct Colour {
    pub colour: u8,
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Colour {
    pub fn parse(r: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        Ok(Self {
            colour: buf[0],
            x: buf[1],
            y: buf[2],
            z: buf[3],
        })
    }

    pub fn parse_list(r: &mut dyn std::io::Read) -> std::io::Result<Vec<Self>> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        let count = u32::from_le_bytes(buf);
        let mut cubes = Vec::with_capacity(count as _);
        for _ in 0..count {
            let cube = Self::parse(r)?;
            cubes.push(cube);
        }
        Ok(cubes)
    }
}
