pub mod cube_list;
pub mod special_item;
pub mod premium_config;
pub mod palette;
pub mod client_config;
pub mod crf_config;
pub mod weapon_list;
pub mod movement_list;
pub mod damage_boost;
pub mod battle_arena_config;
pub mod cpu_limits;
pub mod cosmetic_limits;
pub mod taunts_config;
pub mod customisation_info;
pub mod garage_bay;
pub mod custom_games;
pub mod tech_tree;
pub mod item_shop_bundle;
pub mod player_robopass_season;
pub mod weapon_upgrade;
pub mod player_rank;
pub mod robot_data;
pub mod quest;

pub(self) fn encode_7_bit_i32(mut src: i32) -> Vec<u8> {
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
