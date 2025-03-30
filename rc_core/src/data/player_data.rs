use polariton::operation::Typed;

pub struct PlayerData {
    pub name: String,
    pub display_name: String,
    pub mastery: i32,
    pub tier: i32,
    pub robot_name: String,
    pub robot_map: Vec<u8>,
    // -- unused i32 here --
    pub team: i32,
    pub has_premium: bool,
    pub robot_uuid: String,
    pub cpu: i32,
    pub weapon_order: Vec<i32>,
    pub colour_map: Vec<u8>,
    pub is_ai: bool,
    pub spawn_effect: String,
    pub death_effect: String,
    pub player_rank: i32,
    pub weapon_rank: std::collections::HashMap<i32, i32>,
}

impl PlayerData {
    fn dump(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
        let mut total_len = super::write_str_for_binreader(&self.name, writer)?;
        total_len += super::write_str_for_binreader(&self.display_name, writer)?;
        writer.write_all(&self.mastery.to_le_bytes())?;
        writer.write_all(&self.tier.to_le_bytes())?;
        total_len += super::write_str_for_binreader(&self.robot_name, writer)?;
        writer.write_all(&(self.robot_map.len() as i32).to_le_bytes())?;
        writer.write_all(&self.robot_map)?;
        writer.write_all(&[0xDE, 0xAD, 0xBE, 0xEF])?;
        writer.write_all(&self.team.to_le_bytes())?;
        writer.write_all(&[self.has_premium as u8])?;
        total_len += super::write_str_for_binreader(&self.robot_uuid, writer)?;
        writer.write_all(&self.cpu.to_le_bytes())?;
        writer.write_all(&(self.weapon_order.len() as i32).to_le_bytes())?;
        for weapon_key in self.weapon_order.iter() {
            writer.write_all(&weapon_key.to_le_bytes())?;
        }
        writer.write_all(&(self.colour_map.len() as i32).to_le_bytes())?;
        writer.write_all(&self.colour_map)?;
        writer.write_all(&[self.is_ai as u8])?;
        total_len += super::write_str_for_binreader(&self.spawn_effect, writer)?;
        total_len += super::write_str_for_binreader(&self.death_effect, writer)?;
        writer.write_all(&self.player_rank.to_le_bytes())?;
        writer.write_all(&(self.weapon_rank.len() as i32).to_le_bytes())?;
        for (key, val) in self.weapon_rank.iter() {
            writer.write_all(&key.to_le_bytes())?;
            writer.write_all(&val.to_le_bytes())?;
        }
        Ok(42 + self.robot_map.len() + (self.weapon_order.len() * 4) + self.colour_map.len() + (self.weapon_rank.len() * 8) + total_len)
    }
}

pub struct PlayerDatas {
    pub players: Vec<PlayerData>,
}

impl PlayerDatas {
    fn dump(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
        writer.write_all(&(self.players.len() as i32).to_le_bytes())?;
        let mut total_len = 4;
        for data in self.players.iter() {
            total_len += data.dump(writer)?;
        }
        Ok(total_len)
    }

    pub fn as_transmissible<C>(&self) -> Typed<C> {
        let mut buf = Vec::new();
        let write_size = self.dump(&mut std::io::Cursor::new(&mut buf)).unwrap();
        log::debug!("PlayerDatas serialized to {} bytes: {:?}", write_size, buf);
        Typed::Bytes(buf.into())
    }
}
