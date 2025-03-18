pub mod player_data;

pub(self) fn encode_7_bit_i32(mut src: i32) -> Vec<u8> {
    if src == 0 { return vec![0] }
    let mut out = Vec::with_capacity(5);
    while src != 0 {
        let last_7 = (src & 0x7F) as u8;
        src = src >> 7;
        if src != 0 {
            out.push(last_7 | 0x80);
        } else {
            out.push(last_7);
        }
    }
    out
}

pub(self) fn write_str_for_binreader(s: &str, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
    let s_bytes = s.as_bytes();
    let mut total_len = writer.write(&encode_7_bit_i32(s_bytes.len() as i32))?;
    total_len += writer.write(s_bytes)?;
    Ok(total_len)
}
