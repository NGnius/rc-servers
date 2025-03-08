use polariton::{operation::{Dict, Typed}, serdes::TypePrefix};

pub struct TauntsData {
    pub taunts: Vec<TauntData>,
}

impl TauntsData {
    pub fn as_transmissible(&self) -> Typed {
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::Any,
            items: self.taunts.iter().map(|t| (Typed::Str(t.group_name.clone().into()), t.as_transmissible())).collect(),
        })
    }
}

pub struct TauntData {
    pub group_name: String, // parent key
    // Dict<str, obj>
    pub assets: AssetData,
    pub animation_offset_x: f32,
    pub animation_offset_y: f32,
    pub animation_offset_z: f32,
    pub cubes: Vec<CubeData>,
}

impl TauntData {
    pub fn as_transmissible(&self) -> Typed {
        let mut items = vec![
            (Typed::Str("defaultAnimOffsetx".into()), Typed::Float(self.animation_offset_x)),
            (Typed::Str("defaultAnimOffsety".into()), Typed::Float(self.animation_offset_y)),
            (Typed::Str("defaultAnimOffsetz".into()), Typed::Float(self.animation_offset_z)),
            (Typed::Str("cubes".into()), Typed::Dict(Dict {
                key_ty: TypePrefix::Str,
                val_ty: TypePrefix::Any,
                items: self.cubes.iter().enumerate().map(|(i, cube)| (Typed::Str(i.to_string().into()), cube.as_transmissible())).collect(),
            })),
        ];
        items.append(&mut self.assets.as_transmissible());
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::Any,
            items,
        })
    }
}

pub struct AssetData {
    pub idle_effect: String,
    pub active_effect: String,
    pub sound_effect: String,
}

impl AssetData {
    pub fn as_transmissible(&self) -> Vec<(Typed, Typed)> {
        vec![
            (Typed::Str("idleEffect".into()), Typed::Str(self.idle_effect.clone().into())),
            (Typed::Str("tauntEffect".into()), Typed::Str(self.active_effect.clone().into())),
            (Typed::Str("tauntSoundEffect".into()), Typed::Str(self.sound_effect.clone().into())),
        ]
    }
}

pub struct CubeData {
    pub cube_id: u32, // hex
    pub position_x: i32,
    pub position_y: i32,
    pub position_z: i32,
    pub rotation: u8,
}

impl CubeData {
    pub fn as_transmissible(&self) -> Typed {
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::Any,
            items: vec![
                (Typed::Str("cubeid".into()), Typed::Str(hex::encode((self.cube_id as i32).to_le_bytes()).into())),
                (Typed::Str("positionx".into()), Typed::Int(self.position_x)),
                (Typed::Str("positiony".into()), Typed::Int(self.position_y)),
                (Typed::Str("positionz".into()), Typed::Int(self.position_z)),
                (Typed::Str("rotation".into()), Typed::Byte(self.rotation)),
            ]
        })
    }
}
