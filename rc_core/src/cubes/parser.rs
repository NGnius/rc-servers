#![allow(dead_code)]

use std::io::Write;

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

    pub fn dump(&self, w: &mut dyn std::io::Write) -> std::io::Result<usize> {
        w.write_all(&self.id.to_le_bytes())?;
        w.write_all(&[self.x, self.y, self.z, self.orientation])?;
        Ok(8)
    }

    pub fn dump_list(items: Vec<Self>) -> std::io::Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(4 + (items.len() * 8));
        let mut dumped = std::io::Cursor::new(&mut buf);
        dumped.write_all(&(items.len() as u32).to_le_bytes())?;
        for item in items {
            item.dump(&mut dumped)?;
        }
        Ok(buf)
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

    pub fn dump(&self, w: &mut dyn std::io::Write) -> std::io::Result<usize> {
        w.write_all(&[self.colour, self.x, self.y, self.z])?;
        Ok(4)
    }

    pub fn dump_list(items: Vec<Self>) -> std::io::Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(4 + (items.len() * 8));
        let mut dumped = std::io::Cursor::new(&mut buf);
        dumped.write_all(&(items.len() as u32).to_le_bytes())?;
        for item in items {
            item.dump(&mut dumped)?;
        }
        Ok(buf)
    }
}
