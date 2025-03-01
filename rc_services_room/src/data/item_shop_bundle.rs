#![allow(dead_code)]

use std::io::Write;

use polariton::operation::Typed;

pub struct ItemShopBundle {
    pub sku: String,
    pub bundle_name_key: String,
    pub sprite: String,
    pub is_sprite_full_size: bool,
    pub category: ItemShopCategory,
    pub currency: CurrencyType, // str
    pub price: i32,
    pub discount_time: i64, // seconds since unix epoch
    pub discount_price: i32,
    pub recurrence: ItemShopRecurrence,
    pub owns_required_cube: bool,
    //pub is_discounted: bool,
    pub is_limited_edition: bool,
}

impl ItemShopBundle {
    pub fn as_transmissible(&self) -> Typed {
        let mut buf = Vec::new();
        let mut writer = std::io::Cursor::new(&mut buf);
        self.dump(&mut writer).unwrap();
        Typed::Bytes(buf.into())
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        let sku_bytes = self.sku.as_bytes();
        let mut total_len = writer.write(&super::encode_7_bit_i32(sku_bytes.len() as i32))?;
        total_len += writer.write(sku_bytes)?;

        let bundle_name_key_bytes = self.bundle_name_key.as_bytes();
        total_len += writer.write(&super::encode_7_bit_i32(bundle_name_key_bytes.len() as i32))?;
        total_len += writer.write(bundle_name_key_bytes)?;

        let sprite_bytes = self.sprite.as_bytes();
        total_len += writer.write(&super::encode_7_bit_i32(sprite_bytes.len() as i32))?;
        total_len += writer.write(sprite_bytes)?;

        total_len += writer.write(&[self.is_sprite_full_size as u8])?;

        let currency_bytes = self.currency.as_str().as_bytes();
        total_len += writer.write(&super::encode_7_bit_i32(currency_bytes.len() as i32))?;
        total_len += writer.write(currency_bytes)?;

        total_len += writer.write(&self.price.to_le_bytes())?;

        total_len += writer.write(&self.discount_time.to_le_bytes())?;

        total_len += writer.write(&self.discount_price.to_le_bytes())?;

        total_len += writer.write(&(self.recurrence as i32).to_le_bytes())?;

        total_len += writer.write(&[self.owns_required_cube as u8])?;

        total_len += writer.write(&(self.category as i32).to_le_bytes())?;

        total_len += writer.write(&[self.is_limited_edition as u8])?;

        Ok(total_len)
    }

    pub fn as_transmissible_vec(items: Vec<Self>) -> Typed {
        let mut buf = Vec::new();
        let mut writer = std::io::Cursor::new(&mut buf);
        writer.write(&(items.len() as i32).to_le_bytes()).unwrap();
        for item in items.iter() {
            item.dump(&mut writer).unwrap();
        }
        Typed::Bytes(buf.into())
    }
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum ItemShopCategory {
    Cube = 0,
    GarageBaySkin = 1,
    Bundle = 2,
    DeathEffect = 3,
    SpawnEffect = 4,
    Emotigram = 5,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum ItemShopRecurrence {
    Daily = 0,
    Weekly = 1,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CurrencyType {
    Robits = 0,
    CosmeticCredits = 1,
}

impl CurrencyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Robits => "Robits",
            Self::CosmeticCredits => "CosmeticCredits",
        }
    }
}
