#![allow(dead_code)]
pub struct ColourValue {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColourValue {
    fn read_no_alpha<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 1];
        reader.read(&mut buf)?;
        let r = buf[0];
        reader.read(&mut buf)?;
        let g = buf[0];
        reader.read(&mut buf)?;
        let b = buf[0];
        Ok(Self {
            r, g, b, a: u8::MAX,
        })
    }

    fn write_no_alpha<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
        writer.write(&[self.r, self.g, self.b])
    }
}

pub struct Colour {
    pub index: u8,
    pub diffuse: ColourValue,
    pub specular: ColourValue,
    pub overlay: ColourValue,
    pub premium: bool,
}

impl Colour {
    fn read_with_index<R: std::io::Read>(index: u8, reader: &mut R) -> std::io::Result<Self> {
        let diffuse = ColourValue::read_no_alpha(reader)?;
        let specular = ColourValue::read_no_alpha(reader)?;
        let overlay = ColourValue::read_no_alpha(reader)?;
        let mut buf = [0u8; 1];
        reader.read(&mut buf)?;
        let premium = buf[0] != 0;
        Ok(Self {
            index, diffuse, specular, overlay, premium,
        })
    }

    pub fn read_many<R: std::io::Read>(reader: &mut R) -> std::io::Result<Vec<Self>> {
        let mut buf = [0u8; 4];
        reader.read(&mut buf)?;
        let count = i32::from_le_bytes(buf);
        let mut results = Vec::with_capacity(count as _);
        for i in 0..count {
            results.push(Self::read_with_index(i as u8, reader)?);
        }
        Ok(results)
    }

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
        let mut total = 0;
        total += self.diffuse.write_no_alpha(writer)?;
        total += self.specular.write_no_alpha(writer)?;
        total += self.overlay.write_no_alpha(writer)?;
        total += writer.write(&[self.premium as u8])?;
        Ok(total)
    }

    pub fn write_many<W: std::io::Write>(items: &[Self], writer: &mut W) -> std::io::Result<usize> {
        let mut total = writer.write(&(items.len() as i32).to_le_bytes())?;
        for item in items.iter() {
            total += item.write(writer)?;
        }
        Ok(total)
    }
}
