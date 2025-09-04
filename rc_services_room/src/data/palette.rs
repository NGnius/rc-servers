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
        reader.read_exact(&mut buf)?;
        let r = buf[0];
        reader.read_exact(&mut buf)?;
        let g = buf[0];
        reader.read_exact(&mut buf)?;
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
        reader.read_exact(&mut buf)?;
        let premium = buf[0] != 0;
        Ok(Self {
            index, diffuse, specular, overlay, premium,
        })
    }

    pub fn read_many<R: std::io::Read>(reader: &mut R) -> std::io::Result<Vec<Self>> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
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

    pub fn default_many() -> &'static [Self] {
        &[
            Colour { index: 0x0, diffuse: ColourValue { r: 255, g: 255, b: 255, a: 255 }, specular: ColourValue { r: 200, g: 200, b: 200, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: false },
    Colour { index: 0x1, diffuse: ColourValue { r: 111, g: 111, b: 111, a: 255 }, specular: ColourValue { r: 111, g: 111, b: 237, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: false },
    Colour { index: 0x4, diffuse: ColourValue { r: 255, g: 129, b: 32, a: 255 }, specular: ColourValue { r: 251, g: 129, b: 32, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: false },
    Colour { index: 0x7, diffuse: ColourValue { r: 17, g: 167, b: 253, a: 255 }, specular: ColourValue { r: 17, g: 167, b: 253, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: false },
    Colour { index: 0x2, diffuse: ColourValue { r: 0, g: 0, b: 0, a: 255 }, specular: ColourValue { r: 89, g: 89, b: 89, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: false },
    Colour { index: 0x3, diffuse: ColourValue { r: 223, g: 28, b: 2, a: 255 }, specular: ColourValue { r: 223, g: 28, b: 2, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: false },
    Colour { index: 0x5, diffuse: ColourValue { r: 254, g: 222, b: 25, a: 255 }, specular: ColourValue { r: 254, g: 222, b: 25, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: false },
    Colour { index: 0x6, diffuse: ColourValue { r: 53, g: 188, b: 28, a: 255 }, specular: ColourValue { r: 53, g: 188, b: 28, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: false },
    Colour { index: 0x1d, diffuse: ColourValue { r: 236, g: 2, b: 194, a: 255 }, specular: ColourValue { r: 236, g: 2, b: 194, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: true },
    Colour { index: 0x8, diffuse: ColourValue { r: 12, g: 72, b: 221, a: 255 }, specular: ColourValue { r: 12, g: 72, b: 221, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: false },
    Colour { index: 0x9, diffuse: ColourValue { r: 151, g: 40, b: 216, a: 255 }, specular: ColourValue { r: 151, g: 40, b: 216, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: false },
    Colour { index: 0xc, diffuse: ColourValue { r: 161, g: 80, b: 26, a: 255 }, specular: ColourValue { r: 161, g: 80, b: 26, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: true },
    Colour { index: 0x14, diffuse: ColourValue { r: 175, g: 188, b: 56, a: 255 }, specular: ColourValue { r: 175, g: 188, b: 56, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x13, diffuse: ColourValue { r: 4, g: 137, b: 64, a: 255 }, specular: ColourValue { r: 4, g: 137, b: 64, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: true },
    Colour { index: 0x10, diffuse: ColourValue { r: 237, g: 210, b: 161, a: 255 }, specular: ColourValue { r: 237, g: 210, b: 161, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x1f, diffuse: ColourValue { r: 253, g: 163, b: 193, a: 255 }, specular: ColourValue { r: 253, g: 163, b: 193, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0xa, diffuse: ColourValue { r: 224, g: 51, b: 16, a: 255 }, specular: ColourValue { r: 224, g: 51, b: 16, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0xb, diffuse: ColourValue { r: 131, g: 52, b: 34, a: 255 }, specular: ColourValue { r: 107, g: 43, b: 27, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0xd, diffuse: ColourValue { r: 128, g: 87, b: 86, a: 255 }, specular: ColourValue { r: 128, g: 87, b: 86, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0xe, diffuse: ColourValue { r: 188, g: 140, b: 152, a: 255 }, specular: ColourValue { r: 188, g: 140, b: 152, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0xf, diffuse: ColourValue { r: 199, g: 159, b: 64, a: 255 }, specular: ColourValue { r: 199, g: 159, b: 64, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x11, diffuse: ColourValue { r: 9, g: 90, b: 43, a: 255 }, specular: ColourValue { r: 9, g: 90, b: 43, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x12, diffuse: ColourValue { r: 73, g: 84, b: 42, a: 255 }, specular: ColourValue { r: 73, g: 84, b: 42, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x15, diffuse: ColourValue { r: 39, g: 48, b: 63, a: 255 }, specular: ColourValue { r: 39, g: 48, b: 63, a: 255 }, overlay: ColourValue { r: 255, g: 255, b: 255, a: 255 }, premium: true },
    Colour { index: 0x16, diffuse: ColourValue { r: 14, g: 68, b: 106, a: 255 }, specular: ColourValue { r: 4, g: 55, b: 60, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x17, diffuse: ColourValue { r: 66, g: 112, b: 200, a: 255 }, specular: ColourValue { r: 66, g: 112, b: 200, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x18, diffuse: ColourValue { r: 88, g: 107, b: 141, a: 255 }, specular: ColourValue { r: 96, g: 129, b: 195, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x19, diffuse: ColourValue { r: 118, g: 142, b: 190, a: 255 }, specular: ColourValue { r: 122, g: 138, b: 216, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x1a, diffuse: ColourValue { r: 138, g: 211, b: 228, a: 255 }, specular: ColourValue { r: 138, g: 211, b: 228, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x1b, diffuse: ColourValue { r: 72, g: 59, b: 103, a: 255 }, specular: ColourValue { r: 30, g: 3, b: 96, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x1c, diffuse: ColourValue { r: 201, g: 69, b: 189, a: 255 }, specular: ColourValue { r: 54, g: 27, b: 39, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
    Colour { index: 0x1e, diffuse: ColourValue { r: 213, g: 150, b: 242, a: 255 }, specular: ColourValue { r: 225, g: 120, b: 255, a: 255 }, overlay: ColourValue { r: 40, g: 40, b: 40, a: 255 }, premium: true },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAME_CAP_MANY_COLOURS: &[u8] = &[0x20, 0x0, 0x0, 0x0, 0xff, 0xff, 0xff, 0xc8, 0xc8, 0xc8, 0x28, 0x28, 0x28, 0x0, 0x6f, 0x6f, 0x6f, 0x6f, 0x6f, 0xed, 0xff, 0xff, 0xff, 0x0, 0xff, 0x81, 0x20, 0xfb, 0x81, 0x20, 0x28, 0x28, 0x28, 0x0, 0x11, 0xa7, 0xfd, 0x11, 0xa7, 0xfd, 0xff, 0xff, 0xff, 0x0, 0x0, 0x0, 0x0, 0x59, 0x59, 0x59, 0xff, 0xff, 0xff, 0x0, 0xdf, 0x1c, 0x2, 0xdf, 0x1c, 0x2, 0xff, 0xff, 0xff, 0x0, 0xfe, 0xde, 0x19, 0xfe, 0xde, 0x19, 0x28, 0x28, 0x28, 0x0, 0x35, 0xbc, 0x1c, 0x35, 0xbc, 0x1c, 0xff, 0xff, 0xff, 0x0, 0xec, 0x2, 0xc2, 0xec, 0x2, 0xc2, 0xff, 0xff, 0xff, 0x1, 0xc, 0x48, 0xdd, 0xc, 0x48, 0xdd, 0xff, 0xff, 0xff, 0x0, 0x97, 0x28, 0xd8, 0x97, 0x28, 0xd8, 0xff, 0xff, 0xff, 0x0, 0xa1, 0x50, 0x1a, 0xa1, 0x50, 0x1a, 0xff, 0xff, 0xff, 0x1, 0xaf, 0xbc, 0x38, 0xaf, 0xbc, 0x38, 0x28, 0x28, 0x28, 0x1, 0x4, 0x89, 0x40, 0x4, 0x89, 0x40, 0xff, 0xff, 0xff, 0x1, 0xed, 0xd2, 0xa1, 0xed, 0xd2, 0xa1, 0x28, 0x28, 0x28, 0x1, 0xfd, 0xa3, 0xc1, 0xfd, 0xa3, 0xc1, 0x28, 0x28, 0x28, 0x1, 0xe0, 0x33, 0x10, 0xe0, 0x33, 0x10, 0x28, 0x28, 0x28, 0x1, 0x83, 0x34, 0x22, 0x6b, 0x2b, 0x1b, 0x28, 0x28, 0x28, 0x1, 0x80, 0x57, 0x56, 0x80, 0x57, 0x56, 0x28, 0x28, 0x28, 0x1, 0xbc, 0x8c, 0x98, 0xbc, 0x8c, 0x98, 0x28, 0x28, 0x28, 0x1, 0xc7, 0x9f, 0x40, 0xc7, 0x9f, 0x40, 0x28, 0x28, 0x28, 0x1, 0x9, 0x5a, 0x2b, 0x9, 0x5a, 0x2b, 0x28, 0x28, 0x28, 0x1, 0x49, 0x54, 0x2a, 0x49, 0x54, 0x2a, 0x28, 0x28, 0x28, 0x1, 0x27, 0x30, 0x3f, 0x27, 0x30, 0x3f, 0xff, 0xff, 0xff, 0x1, 0xe, 0x44, 0x6a, 0x4, 0x37, 0x3c, 0x28, 0x28, 0x28, 0x1, 0x42, 0x70, 0xc8, 0x42, 0x70, 0xc8, 0x28, 0x28, 0x28, 0x1, 0x58, 0x6b, 0x8d, 0x60, 0x81, 0xc3, 0x28, 0x28, 0x28, 0x1, 0x76, 0x8e, 0xbe, 0x7a, 0x8a, 0xd8, 0x28, 0x28, 0x28, 0x1, 0x8a, 0xd3, 0xe4, 0x8a, 0xd3, 0xe4, 0x28, 0x28, 0x28, 0x1, 0x48, 0x3b, 0x67, 0x1e, 0x3, 0x60, 0x28, 0x28, 0x28, 0x1, 0xc9, 0x45, 0xbd, 0x36, 0x1b, 0x27, 0x28, 0x28, 0x28, 0x1, 0xd5, 0x96, 0xf2, 0xe1, 0x78, 0xff, 0x28, 0x28, 0x28, 0x1];

    #[test]
    fn parse_game_cap() {
        let mut reader = std::io::Cursor::new(GAME_CAP_MANY_COLOURS);
        let colours = Colour::read_many(&mut reader).expect("Invalid game cap");
        fn colour_display_to_str(cv: &ColourValue) -> String {
            format!("ColourValue {{ r: {}, g: {}, b: {}, a: {} }}", cv.r, cv.g, cv.b, cv.a)
        }
        for colour in colours {
            println!("Colour {{ index: {}, diffuse: {}, specular: {}, overlay: {}, premium: {} }}",
                     colour.index,
                     colour_display_to_str(&colour.diffuse),
                     colour_display_to_str(&colour.specular),
                     colour_display_to_str(&colour.overlay),
                     colour.premium);
        }
    }
}
