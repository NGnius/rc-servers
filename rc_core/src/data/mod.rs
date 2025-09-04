pub mod auto_regen;
pub mod campaign;
pub mod client_config;
pub mod cube_list;
pub mod game_mode;
pub mod garage_bay;
pub mod movement_list;
pub mod player_data;
pub mod tech_tree;
pub mod voting;
pub mod weapon_list;
pub mod weapon_upgrade;
pub mod crf;
pub mod channel;
pub mod sanction;
pub mod robot_data;
pub mod lobby;
pub mod battle_arena_config;

pub mod error_codes;

pub fn encode_7_bit_i32(mut src: i32) -> Vec<u8> {
    if src == 0 { return vec![0] }
    let mut out = Vec::with_capacity(5);
    while src != 0 {
        let last_7 = (src & 0x7F) as u8;
        src >>= 7;
        if src != 0 {
            out.push(last_7 | 0x80);
        } else {
            out.push(last_7);
        }
    }
    out
}

pub fn decode_7_bit_i32(reader: &mut dyn std::io::Read) -> std::io::Result<i32> {
    let mut buf = [0u8; 1];
    let mut out: i32 = 0;
    for _ in 0..5 {
        reader.read_exact(&mut buf)?;
        let byte = buf[0];
        let has_more = byte & 0x80;
        let number = byte & 0x7F;
        out = (out << 7) | (number as i32);
        if has_more == 0 {
            return Ok(out);
        }
    }
    Ok(out)
}

pub fn write_str_for_binreader(s: &str, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
    let s_bytes = s.as_bytes();
    let mut total_len = writer.write(&encode_7_bit_i32(s_bytes.len() as i32))?;
    total_len += writer.write(s_bytes)?;
    Ok(total_len)
}

pub fn read_str_for_binwriter(reader: &mut dyn std::io::Read) -> std::io::Result<String> {
    let len = decode_7_bit_i32(reader)?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

pub fn cube_id_to_str(id: u32) -> String {
    hex::encode(id.to_be_bytes())
}
