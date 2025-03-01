#![allow(dead_code)]

use std::io::Write;

use polariton::operation::Typed;

pub struct DailyQuestsInfo {
    pub can_remove_quest: bool,
    pub player_quests: Vec<QuestInfo>,
    pub completed_quests: Vec<QuestInfo>,
}

impl DailyQuestsInfo {
    pub fn as_transmissible(&self) -> Typed {
        let mut buf = Vec::new();
        let mut writer = std::io::Cursor::new(&mut buf);
        self.dump(&mut writer).unwrap();
        Typed::Bytes(buf.into())
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        let mut total_len = writer.write(&[self.can_remove_quest as u8])?;
        total_len += QuestInfo::dump_many(writer, &self.player_quests)?;
        total_len += QuestInfo::dump_many(writer, &self.completed_quests)?;
        Ok(total_len)
    }


}

pub struct QuestInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub xp: i32,
    pub premium_xp: i32,
    pub robits: i32,
    pub premium_robits: i32,
    pub progress_count: i32,
    pub target_count: i32,
    pub seen: bool,
}

impl QuestInfo {
    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        let mut total_len = super::write_str_for_binreader(&self.id, writer)?;
        total_len += super::write_str_for_binreader(&self.name, writer)?;
        total_len += super::write_str_for_binreader(&self.description, writer)?;
        total_len += writer.write(&self.xp.to_le_bytes())?;
        total_len += writer.write(&self.premium_xp.to_le_bytes())?;
        total_len += writer.write(&self.robits.to_le_bytes())?;
        total_len += writer.write(&self.premium_robits.to_le_bytes())?;
        total_len += writer.write(&self.progress_count.to_le_bytes())?;
        total_len += writer.write(&self.target_count.to_le_bytes())?;
        total_len += writer.write(&[self.seen as u8])?;
        Ok(total_len)
    }

    fn dump_many(writer: &mut dyn Write, quests: &[QuestInfo]) -> std::io::Result<usize> {
        let mut total_len = writer.write(&(quests.len() as i16).to_le_bytes())?;
        for quest in quests.iter() {
            total_len += quest.dump(writer)?;
        }
        Ok(total_len)
    }
}
