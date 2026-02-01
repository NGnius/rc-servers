#[allow(dead_code)]
pub struct ShopItemListFilters {
    pub page: u32,
    pub page_size: u32,
    pub weapon_filter: i32,
    pub movement_filter: i32,
    pub weapon_groups: String,
    pub movement_groups: String,
    pub player: bool,
    pub sort_mode: i32,
    pub min_cpu: i32,
    pub max_cpu: i32,
    pub min_robot_ranking: i32,
    pub max_robot_ranking: i32,
    pub text: String,
    pub text_search_field: i32,
    pub show_featured: bool,
    pub show_hidden: bool, // dev-only?
    pub no_filters: bool,
}

impl ShopItemListFilters {
    pub fn parse(r: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(Self {
            page: read_u32(r)?,
            page_size: read_u32(r)?,
            weapon_filter: read_i32(r)?,
            movement_filter: read_i32(r)?,
            weapon_groups: super::read_str_for_binwriter(r)?,
            movement_groups: super::read_str_for_binwriter(r)?,
            player: read_bool(r)?,
            sort_mode: read_i32(r)?,
            min_cpu: read_i32(r)?,
            max_cpu: read_i32(r)?,
            min_robot_ranking: read_i32(r)?,
            max_robot_ranking: read_i32(r)?,
            text: super::read_str_for_binwriter(r)?,
            text_search_field: read_i32(r)?,
            show_featured: read_bool(r)?,
            show_hidden: read_bool(r)?,
            no_filters: read_bool(r)?,
        })
    }

    pub fn into_core(self) -> libfj::robocraft::ListQuery {
        let weapon_groups = split_u32(&self.weapon_groups);
        let movement_groups = split_u32(&self.movement_groups);
        libfj::robocraft::ListQuery {
            page: self.page as _,
            page_size: self.page_size as _,
            order: libfj::robocraft::FactoryOrderType::try_from(self.sort_mode as u8).unwrap_or(libfj::robocraft::FactoryOrderType::Suggested),
            player_filter: self.player,
            movement_filter: movement_groups.clone(),
            movement_category_filter: movement_groups,
            weapon_filter: weapon_groups.clone(),
            weapon_category_filter: weapon_groups,
            minimum_cpu: if self.min_cpu <= 0 { 0 } else { self.min_cpu as _ },
            maximum_cpu: if self.max_cpu <= 0 { usize::MAX } else { self.max_cpu as _ },
            text_filter: self.text,
            text_search_field: libfj::robocraft::FactoryTextSearchField::try_from(self.text_search_field as u8).unwrap_or(libfj::robocraft::FactoryTextSearchField::All),
            buyable: true,
            prepend_featured_robot: true,
            featured_only: self.show_featured,
            default_page: self.no_filters,
        }
    }
}

pub struct ItemResult {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub style_rating: f64,
    pub combat_rating: f64,
    pub cpu: i32,
    pub total_robot_ranking: i32,
    pub expiry_date: i64, // ticks until expiry (from now)
    pub buyable: bool,
    pub added_by: String,
    pub added_by_display_name: String,
    pub added_date: i64, // tick until added (from now -- probably negative)
    pub rent_count: i32,
    pub buy_count: i32,
    pub featured: bool,
    pub banner_message: String,
    pub cube_counts: Vec<(u32, u32)>,
}

// a tick is 100ns

impl ItemResult {
    pub fn dump(&self, w: &mut dyn std::io::Write) -> std::io::Result<usize> {
        let mut total_len = write_i32(w, self.id)?;
        total_len += super::write_str_for_binreader(&self.name, w)?;
        total_len += super::write_str_for_binreader(&self.description, w)?;
        total_len += super::write_str_for_binreader(&self.thumbnail, w)?;
        total_len += write_f64(w, self.style_rating)?;
        total_len += write_f64(w, self.combat_rating)?;
        total_len += write_i32(w, self.cpu)?;
        total_len += write_i32(w, self.total_robot_ranking)?;
        total_len += write_i64(w, self.expiry_date)?;
        total_len += write_bool(w, self.buyable)?;
        total_len += super::write_str_for_binreader(&self.added_by, w)?;
        total_len += super::write_str_for_binreader(&self.added_by_display_name, w)?;
        total_len += write_i64(w, self.added_date)?;
        total_len += write_i32(w, self.rent_count)?;
        total_len += write_i32(w, self.buy_count)?;
        total_len += write_bool(w, self.featured)?;
        total_len += super::write_str_for_binreader(&self.banner_message, w)?;
        total_len += write_i32(w, self.cube_counts.len() as i32)?;
        for (key, val) in self.cube_counts.iter() {
            total_len += write_u32(w, *key)?;
            total_len += write_u32(w, *val)?;
        }
        Ok(total_len)
    }

    pub fn dump_many(items: &[Self], w: &mut dyn std::io::Write) -> std::io::Result<usize> {
        let mut total_len = write_i32(w, items.len() as _)?;
        for item in items.iter() {
            total_len += item.dump(w)?;
        }
        Ok(total_len)
    }

    pub fn as_transmissible<C>(items: &[Self]) -> polariton::operation::Typed<C> {
        let mut buf = Vec::new();
        Self::dump_many(items, &mut buf).unwrap();
        polariton::operation::Typed::Bytes(buf.into())
    }
}

fn ticks_from_now(time: &chrono::DateTime<chrono::Utc>) -> i64 {
    let dur = time.signed_duration_since(chrono::offset::Utc::now());
    dur.num_nanoseconds()
        .map(|x| x/100)
        .unwrap_or_else(|| dur.num_milliseconds() * 1_000_000 / 100)
}

impl std::convert::From<oj_rc_factory::VehicleQueryInfo> for ItemResult {
    fn from(value: oj_rc_factory::VehicleQueryInfo) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            thumbnail: value.thumbnail,
            style_rating: value.cosmetic_rating,
            combat_rating: value.combat_rating,
            cpu: value.cpu as i32,
            total_robot_ranking: value.total_robot_ranking as i32,
            expiry_date: ticks_from_now(&value.expiry_date),
            //expiry_date: i32::MAX as _,
            buyable: value.buyable,
            added_by: value.added_by,
            added_by_display_name: value.added_by_display_name,
            added_date: ticks_from_now(&value.added_date),
            rent_count: value.rent_count as _,
            buy_count: value.buy_count as _,
            featured: value.featured,
            banner_message: value.banner_message.unwrap_or_default(),
            cube_counts: value.cube_amounts.into_iter().collect(),
        }
    }
}

pub struct ItemData {
    pub index: i32,
    pub cube_data: Vec<u8>,
    pub colour_data: Vec<u8>,
}

impl ItemData {
    pub fn as_transmissible<C>(&self) -> polariton::operation::Typed<C> {
        polariton::operation::Typed::HashMap(vec![
            (polariton::operation::Typed::Str("itemIndex".into()), polariton::operation::Typed::Int(self.index)),
            (polariton::operation::Typed::Str("cubeData".into()), polariton::operation::Typed::Bytes(self.cube_data.clone().into())),
            (polariton::operation::Typed::Str("colourData".into()), polariton::operation::Typed::Bytes(self.colour_data.clone().into())),
        ].into())
    }
}

impl std::convert::From<oj_rc_factory::VehicleInfo> for ItemData {
    fn from(value: oj_rc_factory::VehicleInfo) -> Self {
        Self {
            index: value.id,
            cube_data: value.cube_data,
            colour_data: value.colour_data,
        }
    }
}

pub struct UploadData {
    pub version: String,
    pub slot: i32,
    pub name: String,
    pub description: String,
    pub thumbnail: Vec<u8>,
}

impl UploadData {
    pub fn from_transmissibles<C>(build_version: String, mut data: polariton::operation::Dict<C>) -> Result<Self, i16> {
        if let Some(slot_i) = data.items.iter().position(|(key, _)| typed_is_str(key, "SlotId")) {
            if let (_, polariton::operation::Typed::Int(slot)) = data.items.swap_remove(slot_i) {
                if let Some(name_i) = data.items.iter().position(|(key, _)| typed_is_str(key, "Name")) {
                    if let (_, polariton::operation::Typed::Str(name)) = data.items.swap_remove(name_i) {
                        if let Some(description_i) = data.items.iter().position(|(key, _)| typed_is_str(key, "Description")) {
                            if let (_, polariton::operation::Typed::Str(description)) = data.items.swap_remove(description_i) {
                                if let Some(thumb_i) = data.items.iter().position(|(key, _)| typed_is_str(key, "Thumbnail")) {
                                    if let (_, polariton::operation::Typed::Bytes(thumb)) = data.items.swap_remove(thumb_i) {
                                        return Ok(Self {
                                            version: build_version,
                                            slot,
                                            name: name.string,
                                            description: description.string,
                                            thumbnail: thumb.vec,
                                        });
                                    } else {
                                        log::warn!("Factory upload data Thumbnail is not Bytes");
                                    }
                                } else {
                                    log::warn!("Factory upload data is missing Thumbnail");
                                }
                            } else {
                                log::warn!("Factory upload data Description is not Str");
                            }
                        } else {
                            log::warn!("Factory upload data is missing Description");
                        }
                    } else {
                        log::warn!("Factory upload data Name is not Str");
                    }
                } else {
                    log::warn!("Factory upload data is missing Name");
                }
            } else {
                log::warn!("Factory upload data SlotId is not Int")
            }
        } else {
            log::warn!("Factory upload data is missing SlotId")
        }
        Err(crate::data::error_codes::WebServicesError::UnexpectedError as i16)
    }

    pub fn into_core(self) -> crate::persist::user::VehicleUploadData {
        crate::persist::user::VehicleUploadData {
            version: self.version,
            slot: self.slot,
            name: self.name,
            description: self.description,
            thumbnail: self.thumbnail,
        }
    }
}

#[inline]
fn read_i32(r: &mut dyn std::io::Read) -> std::io::Result<i32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

#[inline]
fn write_i32(w: &mut dyn std::io::Write, num: i32) -> std::io::Result<usize> {
    w.write_all(&num.to_le_bytes())?;
    Ok(4)
}

#[inline]
fn write_i64(w: &mut dyn std::io::Write, num: i64) -> std::io::Result<usize> {
    w.write_all(&num.to_le_bytes())?;
    Ok(8)
}

#[inline]
fn write_f64(w: &mut dyn std::io::Write, num: f64) -> std::io::Result<usize> {
    w.write_all(&num.to_le_bytes())?;
    Ok(8)
}

#[inline]
fn read_u32(r: &mut dyn std::io::Read) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

#[inline]
fn write_u32(w: &mut dyn std::io::Write, num: u32) -> std::io::Result<usize> {
    w.write_all(&num.to_le_bytes())?;
    Ok(4)
}

#[inline]
fn read_bool(r: &mut dyn std::io::Read) -> std::io::Result<bool> {
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf)?;
    Ok(buf[0] != 0)
}

#[inline]
fn write_bool(w: &mut dyn std::io::Write, b: bool) -> std::io::Result<usize> {
    w.write_all(&[b as u8])?;
    Ok(1)
}

#[inline]
fn split_u32(s: &str) -> Vec<u32> {
    s.split(',').filter_map(|x| x.parse().ok()).collect()
}

fn typed_is_str<C>(ty: &polariton::operation::Typed<C>, s: &str) -> bool {
    if let polariton::operation::Typed::Str(ty_s) = ty {
        ty_s.string == s
    } else {
        false
    }
}
