const MAX_WEAPON_SLOTS: usize = 3;

struct WeaponInfo {
    category: crate::data::weapon_list::ItemCategory,
    tier: crate::data::cube_list::ItemTier,
}

impl WeaponInfo {
    fn weapon_order_key(&self) -> i32 {
        self.category.but_bigger() + (self.tier as i32)
    }
}

pub struct WeaponListParser {
    weapons: std::collections::HashMap<u32, WeaponInfo>,
}

impl WeaponListParser {
    pub fn with_cubes<'a, I: std::iter::Iterator<Item=&'a crate::persist::Cube>>(iter: I) -> Self {
        let mut weapons = std::collections::HashMap::new();
        for item in iter {
            if matches!(item.info.type_, crate::persist::ItemType::Weapon | crate::persist::ItemType::Module){
                weapons.insert(item.id, WeaponInfo {
                    category: item.info.category.into(),
                    tier: item.info.size.into(),
                });
            }
        }
        Self {
            weapons,
        }
    }

    pub fn guess_weapons(&self, r: &mut dyn std::io::Read) -> Vec<i32> {
        match super::parser::Cube::parse_list(r) {
            Ok(cubes) => {
                let mut keys = std::collections::HashSet::new();
                for cube in cubes {
                    if let Some(weapon) = self.weapons.get(&cube.id) {
                        keys.insert(weapon.weapon_order_key());
                        if keys.len() == MAX_WEAPON_SLOTS { break; }
                    }
                }
                let mut result = keys.into_iter().collect::<Vec<i32>>();
                result.sort();
                result
            }
            Err(e) => {
                log::error!("Failed to parse cube data to guess weapon order: {}", e);
                Vec::default()
            }
        }
    }
}
