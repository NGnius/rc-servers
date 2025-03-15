use polariton::operation::Typed;

pub struct AutoRegenHealthConfig {
    pub seconds_to_wait_for_heal: f32,
    pub seconds_to_full_heal: f32,
    pub threshold_to_start_sound: f32,
    pub enable_auto_heal: bool,
}

impl AutoRegenHealthConfig {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        let mut bytes = Vec::with_capacity(13);
        bytes.extend_from_slice(&self.seconds_to_wait_for_heal.to_le_bytes());
        bytes.extend_from_slice(&self.seconds_to_full_heal.to_le_bytes());
        bytes.extend_from_slice(&self.threshold_to_start_sound.to_le_bytes());
        bytes.push(self.enable_auto_heal as u8);
        Typed::Bytes(bytes.into())
    }
}
