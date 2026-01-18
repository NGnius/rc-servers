pub struct OffsetParser;

impl OffsetParser {
    pub fn with_cubes<'a, I: std::iter::Iterator<Item=&'a crate::persist::Cube>>(_iter: I) -> Self {
        Self
    }

    /// offset is (x, y, z)
    pub fn offset_inplace_by(&self, cubes: &mut [u8], colours: &mut [u8], offset: (i16, i16, i16)) {
        // this assumes cubes and colours are valid and length-prefixed
        let cubes = &mut cubes[4..];
        let colours = &mut colours[4..];
        for cube in cubes.chunks_mut(8) {
            // bytes 4, 5, 6 are x, y, z (respectively)
            cube[4] = (cube[4] as u16).saturating_add_signed(offset.0) as _;
            cube[5] = (cube[5] as u16).saturating_add_signed(offset.1) as _;
            cube[6] = (cube[6] as u16).saturating_add_signed(offset.2) as _;
        }
        for colour in colours.chunks_mut(4) {
            // last 3 bytes are x, y, z (respectively)
            colour[1] = (colour[1] as u16).saturating_add_signed(offset.0) as _;
            colour[2] = (colour[2] as u16).saturating_add_signed(offset.1) as _;
            colour[3] = (colour[3] as u16).saturating_add_signed(offset.2) as _;
        }
    }

    pub fn offset_inplace_with_id(&self, cubes: &mut [u8], offset: (i8, i8, i8), target_id: u32) {
        let cubes = &mut cubes[4..];
        for cube in cubes.chunks_mut(8) {
            let id = u32::from_le_bytes([cube[0], cube[1], cube[2], cube[3]]);
            if id != target_id { continue; }
            // bytes 4, 5, 6 are x, y, z (respectively)
            cube[4] = cube[4].saturating_add_signed(offset.0);
            cube[5] = cube[5].saturating_add_signed(offset.1);
            cube[6] = cube[6].saturating_add_signed(offset.2);
        }
    }
}
