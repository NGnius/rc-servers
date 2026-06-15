use serde::{Serialize, Deserialize};

pub(crate) fn actual_default_conversion() -> CubeConversionData {
    CubeConversionData {
        from: Vec::default(),
        to: None,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CubeConversionData {
    pub from: Vec<FromConversionData>,
    pub to: Option<ToConversionData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum FromConversionData {
    Simple(u32),
    Complex {
        id: u32,
        offset: Option<(i16, i16, i16)>,
        #[serde(default)]
        colour: u8,
    },
}

impl FromConversionData {
    pub(crate) fn id(&self) -> u32 {
        match self {
            Self::Simple(id) => *id,
            Self::Complex { id, .. } => *id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ToConversionData {
    Simple(u32),
    Complex {
        id: u32,
        offset: Option<(i16, i16, i16)>,
    },
}

pub(crate) fn expand_conversion_data(cube: &mut super::Cube) {
    if cube.conversion.is_some() { return; }
    let conversion = match cube.id {
        0 => CubeConversionData {
            from: (0..24).map(|i| FromConversionData::Complex {
                    id: 0,
                    offset: Some((i as _, (i * 4) as _, (i * -2) as _)),
                    colour: i as u8,
                }).collect(),
            to: None,
        },
        // cone
        1352234106 => CubeConversionData { // MediumCone
            from: vec![
                FromConversionData::Simple(914659855), // ArmoredConeT10
                FromConversionData::Complex { // ArmoredConeT10Orange
                    id: 903181232,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredConeT10Yellow
                    id: 2073756085,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredConeT10Gold
                    id: 420704,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(1837280721), // HallowConeT10
                FromConversionData::Complex { // ArmoredConeT10Blue
                    id: 603738000,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredConeT10DarkBlue
                    id: 2126078066,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(1727367602), // RareConeT10
                FromConversionData::Simple(311721892), // WolfConeT10
                FromConversionData::Simple(3504767087), // QuarterbackConeT10
                FromConversionData::Complex { // ArmoredConeT10C6B
                    id: 1558067038,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredConeT10MatteWhite
                    id: 914659773,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredConeT10MatteRed
                    id: 865911715,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredConeT10C6O
                    id: 2786019727,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredConeT10SType
                    id: 1503181726,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredConeT10MatteGreen
                    id: 107293578,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredConeT10MatteBlue
                    id: 1824768921,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredConeT10MattePurple
                    id: 1911941389,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                // no equivalent translation
            ],
            to: None,
        },
        1979640496 => only_to(914659855), // CompactCone to ArmoredConeT10
        3886136035 => only_to(914659855), // HeavytCone [sic]
        1422041599 => only_to(914659855), // LightCone
        552682446 => only_to(914659855), // NeonCone
        // full cubes
        1576857358 => reciprocal(2234346032), // CompactCube
        123901970 => CubeConversionData { // heavy cube
            from: vec![
                FromConversionData::Simple(1654632428), // HeavyChassisCube
                FromConversionData::Simple(366000), // special april fools heavy
            ],
            to: Some(ToConversionData::Simple(1654632428)),
        },
        227205318 => CubeConversionData { // medium cube
            from: vec![
                FromConversionData::Complex { // ArmoredCubeT10
                    id: 227205318,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredCubeT10Orange
                    id: 903181219,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredCubeT10Yellow
                    id: 2073756071,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredCubeT10Gold
                    id: 420691,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(81509462), // HallowCubeT10
                FromConversionData::Complex { // ArmoredCubeT10Blue
                    id: 603737987,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredCubeT10DarkBlue
                    id: 2126078053,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(1727367589), // RareArmoredCubeT3
                FromConversionData::Simple(3111721879), // WolfCubeT5
                FromConversionData::Simple(3504767074), // QuarterbackCubeT4
                FromConversionData::Complex { // ArmoredCubeT10MatteWhite
                    id: 2553833528,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredCubeT10MatteRed
                    id: 2688683104,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredCubeT10SType
                    id: 2688683104,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredCubeT10MatteGreen
                    id: 107293565,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredCubeT10MatteBlue
                    id: 1824768908,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredCubeT10MattePurple
                    id: 1911941376,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                // TODO colour-match the tiered cubes
                FromConversionData::Simple(936473914), // ArmoredCubeT9
                FromConversionData::Simple(3130466754), // ArmoredCubeT8
                FromConversionData::Simple(1602312433), // ArmoredCubeT7
                FromConversionData::Simple(3217516252), // ArmoredCubeT6
                FromConversionData::Simple(3902577141), // ArmoredCubeT5
                FromConversionData::Simple(148547544), // ArmoredCubeT4
                FromConversionData::Simple(1428281037), // ArmoredCubeT3
                FromConversionData::Simple(3523011797), // ArmoredCubeT2
                FromConversionData::Simple(3795589065), // ArmoredCubeT1
                // end tiered
                FromConversionData::Simple(2256925527), // ArmoredCubeT10White_STypeSymbol
                FromConversionData::Simple(1733271378), // ArmoredCubeT10SType_STypeSymbol
                // no equivalent translation
                // non-building blocks
                FromConversionData::Simple(3951497291), // Fusion Tower
                FromConversionData::Simple(606866102), // Protonium Clasp
                FromConversionData::Simple(3950293873), // Protonium Crystal

                FromConversionData::Simple(2000000000), // Camera Cube
            ],
            to: None,
        },
        1972224393 => reciprocal(2295832800), // light cube
        3367514907 => CubeConversionData { // CubewithC6Logo
            from: vec![
                FromConversionData::Complex { // ArmoredCubeT10C6B
                    id: 501172250,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredCubeT10C6O
                    id: 3461973291,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
            ],
            to: None,
        },
        2392661891 => CubeConversionData { // CubewithCARBONLetters
            from: vec![
                FromConversionData::Complex { // ArmoredCubeT10C6B_Six
                    id: 3367514907,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredCubeT10C6B_Carbon
                    id: 2392661891,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredCubeT10C6O_Six
                    id: 457113130,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredCubeT10C6O_Carbon
                    id: 1562104498,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
            ],
            to: Some(ToConversionData::Simple(3367514907)),
        },
        3327812742 => CubeConversionData { // GlassCube
            from: vec![
                FromConversionData::Simple(3969548222), // WindshieldCubeT10
                FromConversionData::Simple(3327812742), // WindshieldCube
            ],
            to: None,
        },
        2077855120 => only_to(227205318), // MediumCubeCardLife
        150161008 => only_to(227205318), // NeonCube
        183067052 => only_to(227205318), // RetroCube
        // inners
        2378018821 => CubeConversionData { // CompactInner
            from: vec![
                FromConversionData::Simple(314459661), // ArmoredInnerTX1
                FromConversionData::Complex { // GoldArmoredInnerTX1
                    id: 783649019,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
            ],
            to: None,
        },
        2577871627 => CubeConversionData { // HeavyInner
            from: vec![
                FromConversionData::Simple(4181414569), // HeavyChassisInner
                FromConversionData::Simple(366003), // special april fools heavy inner
            ],
            to: Some(ToConversionData::Simple(1654632428)),
        },
        3559488176 => CubeConversionData { // MediumInner
            from: vec![
                FromConversionData::Simple(3559488176), // ArmoredInnerT10
                FromConversionData::Complex { // ArmoredInnerT10Orange
                    id: 903181222,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredInnerT10Yellow
                    id: 2073756074,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredInnerT10Gold
                    id: 420694,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(2292045362), // HallowInnerT10
                FromConversionData::Complex { // ArmoredInnerT10Blue
                    id: 603737990,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredInnerT10DarkBlue
                    id: 2126078056,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(1478755203), // RareArmoredInnerT3
                FromConversionData::Simple(1730552257), // WolfInnerT5
                FromConversionData::Simple(3293765065), // QuarterbackInnerT4
                FromConversionData::Complex { // ArmoredInnerT10C6B
                    id: 4039842231,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredInnerT10MatteWhite
                    id: 3217177779,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredInnerT10MatteRed
                    id: 865911707,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredInnerT10C6O
                    id: 592303238,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredInnerT10SType
                    id: 1503181716,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredInnerT10MatteGreen
                    id: 107293568,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredInnerT10MatteBlue
                    id: 1824768911,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredInnerT10MattePurple
                    id: 1911941379,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                FromConversionData::Complex { // ArmoredInnerT10MatteGreen
                    id: 107293568,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                // TODO colour-match the tiered cubes
                FromConversionData::Simple(2704279673), // ArmoredInnerT9
                FromConversionData::Simple(1664400308), // ArmoredInnerT8
                FromConversionData::Simple(3737084984), // ArmoredInnerT7
                FromConversionData::Simple(1717894314), // ArmoredInnerT6
                FromConversionData::Simple(1772266812), // ArmoredInnerT5
                FromConversionData::Simple(3514383790), // ArmoredInnerT4
                FromConversionData::Simple(3558861316), // ArmoredInnerT3
                FromConversionData::Simple(140476067), // ArmoredInnerT2
                FromConversionData::Simple(1661086464), // ArmoredInnerT1
                // no equivalent translation
                // pyramid inner
                FromConversionData::Simple(366009), //  special april fools heavy pyramid inner
                FromConversionData::Simple(914659851), // ArmoredPyramidInnerT10
                FromConversionData::Simple(903181228), // ArmoredRoundPyramidInnerT10Orange
                FromConversionData::Simple(2073756081), // ArmoredPyramidInnerT10Yellow
                FromConversionData::Simple(420700), // ArmoredPyramidInnerT10Gold
                FromConversionData::Simple(1837280717), // HallowPyramidInnerT10
                FromConversionData::Simple(603737996), // ArmoredPyramidInnerT10Blue
                FromConversionData::Simple(2126078062), // ArmoredPyramidInnerT10DarkBlue
                FromConversionData::Simple(1727367598), // RarePyramidInnerT10
                FromConversionData::Simple(311721888), // WolfPyramidInnerT10
                FromConversionData::Simple(3504767083), // QuarterbackPyramidInnerT10
                FromConversionData::Simple(1558067034), // ArmoredPyramidInnerT10C6B
                FromConversionData::Simple(914659769), // ArmoredPyramidInnerT10MatteWhite
                FromConversionData::Simple(865911711), // ArmoredPyramidInnerT10MatteRed
                FromConversionData::Simple(2786019731), // ArmoredPyramidInnerT10C6O
                FromConversionData::Simple(1503181722), // ArmoredPyramidInnerT10SType
                FromConversionData::Simple(107293574), // ArmoredPyramidInnerT10MatteGreen
                FromConversionData::Simple(1824768917), // ArmoredPyramidInnerT10MatteBlue
                FromConversionData::Simple(1911941385), // ArmoredPyramidInnerT10MattePurple
            ],
            to: None,
        },
        1941682825 => reciprocal(3920646105), // light inner
        445857183 => CubeConversionData { // GlassInner
            from: vec![
                FromConversionData::Simple(2305492794), // WindshieldInnerT10
                FromConversionData::Simple(445857183), // WindshieldInner
            ],
            to: None,
        },
        1616217456 => only_to(3559488176), // NeonInner
        3985049715 => only_to(3559488176), // RetroInner
        // edges aka prisms
        327960333 => CubeConversionData { // CompactEdge
            from: vec![
                FromConversionData::Simple(3003461135), // ArmoredPrismTX1
                FromConversionData::Complex { // GoldArmoredPrismTX1
                    id: 776898512,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
            ],
            to: None,
        },
        1225928721 => CubeConversionData { // HeavyEdge
            from: vec![
                FromConversionData::Simple(2686466102), // HeavyChassisPrism
                FromConversionData::Simple(366001), // special april fools heavy prism
            ],
            to: Some(ToConversionData::Simple(1654632428)),
        },
        227917916 => CubeConversionData { // MediumEdge
            from: vec![
                FromConversionData::Simple(227917916), // ArmoredPrismT10
                FromConversionData::Complex { // ArmoredPrismT10Orange
                    id: 903181220,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredPrismT10Yellow
                    id: 2073756072,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredPrismT10Gold
                    id: 420692,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(1309620559), // HallowPrismT10
                FromConversionData::Complex { // ArmoredPrismT10Blue
                    id: 603737988,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredPrismT10DarkBlue
                    id: 2126078054,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(2662788350), // RareArmoredPrismT3
                FromConversionData::Simple(3197803309), // WolfPrismT5
                FromConversionData::Simple(46150324), // QuarterbackPrismT4
                FromConversionData::Complex { // ArmoredPrismT10C6B
                    id: 2752826359,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredPrismT10MatteWhite
                    id: 2067295656,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredPrismT10MatteRed
                    id: 1097117136,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredInnerT10C6O
                    id: 2006058694,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredPrismT10SType
                    id: 1503181714,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredPrismT10MatteGreen
                    id: 107293566,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredPrismT10MatteBlue
                    id: 1824768909,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredPrismT10MattePurple
                    id: 1911941377,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                FromConversionData::Complex { // ArmoredInnerT10MatteGreen
                    id: 107293568,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                // TODO colour-match the tiered cubes
                FromConversionData::Simple(3159626035), // ArmoredPrismT9
                FromConversionData::Simple(3129475416), // ArmoredPrismT8
                FromConversionData::Simple(117694164), // ArmoredPrismT7
                FromConversionData::Simple(3218622022), // ArmoredPrismT6
                FromConversionData::Simple(2954764240), // ArmoredPrismT5
                FromConversionData::Simple(147162946), // ArmoredPrismT4
                FromConversionData::Simple(228806888), // ArmoredPrismT3
                FromConversionData::Simple(3521365071), // ArmoredPrismT2
                FromConversionData::Simple(3133051372), // ArmoredPrismT1
                // no equivalent translation
            ],
            to: None,
        },
        4157111053 => reciprocal(2231909735), // light edge
        4235300269 => CubeConversionData { // GlassEdge
            from: vec![
                FromConversionData::Simple(2956906195), // WindshieldPrismT10
                FromConversionData::Simple(4235300269), // WindshieldPrism
            ],
            to: None,
        },
        348383505 => only_to(227917916), // MediumEdgeCardLife
        1183051379 => only_to(227917916), // NeonEdge
        1150929327 => only_to(227917916), // RetroEdge
        // pyramids aka pyramid tetras
        2686602224 => CubeConversionData { // MediumPyramid
            from: vec![
                FromConversionData::Simple(914659850), // ArmoredPyramidTetraT10
                FromConversionData::Complex { // ArmoredPyramidTetraT10Orange
                    id: 903181227,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredPyramidTetraT10Yellow
                    id: 2073756080,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredPyramidTetraT10Gold
                    id: 420699,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(1837280716), // HallowPyramidTetraT10
                FromConversionData::Complex { // ArmoredPyramidTetraT10Blue
                    id: 603737995,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredPyramidTetraT10DarkBlue
                    id: 2126078061,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(1727367597), // RarePyramidTetraT10
                FromConversionData::Simple(311721887), // WolfPyramidTetraT10
                FromConversionData::Simple(3504767082), // QuarterbackPyramidTetraT10
                FromConversionData::Complex { // ArmoredPyramidTetraT10C6B
                    id: 1558067033,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredPyramidTetraT10MatteWhite
                    id: 914659768,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredPyramidTetraT10MatteRed
                    id: 865911710,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredPyramidTetraT10C6O
                    id: 2786019732,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredPyramidTetraT10SType
                    id: 1503181721,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredPyramidTetraT10MatteGreen
                    id: 107293573,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredPyramidTetraT10MatteBlue
                    id: 1824768916,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredPyramidTetraT10MattePurple
                    id: 1911941384,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                // no equivalent translation
            ],
            to: None,
        },
        979123471 => only_to(914659850), // CompactPyramid to ArmoredPyramidTetraT10
        4184274974 => only_to(914659850), // HeavyPyramid to ArmoredPyramidTetraT10
        885415789 => only_to(914659850), // LightPyramid
        3447434946 => only_to(914659850), // NeonPyramid
        // no glass pyramid exists
        // wedges (just the headlamp)
        1367205243 => CubeConversionData { // Headlamp
            from: vec![
                FromConversionData::Simple(1367205243), // Headlamp
                FromConversionData::Simple(366007), //  special april fools heavy wedge
                FromConversionData::Simple(914659766), // ArmoredWedgeT10
                FromConversionData::Simple(903181226), // ArmoredWedgeT10Orange
                FromConversionData::Simple(2073756079), // ArmoredWedgeT10Yellow
                FromConversionData::Simple(420698), // ArmoredWedgeT10Gold
                FromConversionData::Simple(1837280715), // HallowWedgeT10
                FromConversionData::Simple(603737994), // ArmoredWedgeT10Blue
                FromConversionData::Simple(2126078060), // ArmoredWedgeT10DarkBlue
                FromConversionData::Simple(1727367596), // RareWedgeT10
                FromConversionData::Simple(311721886), // WolfWedgeT10
                FromConversionData::Simple(3504767081), // QuarterbackWedgeT10
                FromConversionData::Simple(1558067032), // ArmoredWedgeT10C6B
                FromConversionData::Simple(914659767), // ArmoredWedgeT10White
                FromConversionData::Simple(865911709), // ArmoredWedgeT10MatteRed
                FromConversionData::Simple(2786019733), // ArmoredWedgeT10C6O
                FromConversionData::Simple(1503181720), // ArmoredWedgeT10SType
                FromConversionData::Simple(107293572), // ArmoredWedgeT10MatteGreen
                FromConversionData::Simple(1824768915), // ArmoredWedgeT10MatteBlue
                FromConversionData::Simple(1911941383), // ArmoredWedgeT10MattePurple
            ],
            to: Some(ToConversionData::Simple(1367205243)),
        },
        // Inner round aka round inner (lol)
        1837286858 => CubeConversionData { // MediumInnerRound
            from: vec![
                FromConversionData::Simple(1837286858), // ArmoredInnerRoundT10
                FromConversionData::Complex { // ArmoredRoundInnerT10Orange
                    id: 903181225,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredRoundInnerT10Yellow
                    id: 2073756078,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredRoundInnerT10Gold
                    id: 420697,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(1837280714), // HallowInnerRoundT10
                FromConversionData::Complex { // ArmoredRoundInnerT10Blue
                    id: 603737993,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredRoundInnerT10DarkBlue
                    id: 2126078059,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(1727367595), // RareRoundInnerT10
                FromConversionData::Simple(311721885), // WolfRoundInnerT10
                FromConversionData::Simple(3504767080), // QuarterbackRoundInnerT10
                FromConversionData::Complex { // ArmoredRoundInnerT10C6B
                    id: 1558067031,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredRoundInnerT10MatteWhite
                    id: 3217177780,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredRoundInnerT10MatteRed
                    id: 865911708,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredRoundInnerT10C6O
                    id: 2786019734,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredRoundInnerT10SType
                    id: 1503181719,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredRoundInnerT10MatteGreen
                    id: 107293571,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredRoundInnerT10MatteBlue
                    id: 1824768914,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredRoundInnerT10MattePurple
                    id: 1911941382,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                // no equivalent translation
            ],
            to: None,
        },
        925214026 => only_to(1837286858), // CompactInnerRound to ArmoredInnerRoundT10
        665761850 => CubeConversionData {
            from: vec![
                FromConversionData::Simple(366006), // special april fools heavy round inner
            ],
            to: Some(ToConversionData::Simple(1837286858)), // ArmoredInnerRoundT10
        },
        1578378954 => only_to(1837286858), // LightInnerRound
        1774712137 => only_to(1837286858), // NeonInnerRound
        3066776961 => reciprocal(477710581), // GlassInnerRound to WindshieldRoundInnerT10
        // edge round aka round prism
        447126572 => CubeConversionData { // MediumEdgeRound
            from: vec![
                FromConversionData::Simple(447126572), // ArmoredPrismRoundT10
                FromConversionData::Complex { // ArmoredRoundPrismT10Orange
                    id: 903181223,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredRoundPrismT10Yellow
                    id: 2073756075,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredRoundPrismT10Gold
                    id: 420695,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(447124524), // HallowPrismRoundT10
                FromConversionData::Complex { // ArmoredRoundPrismT10Blue
                    id: 603737991,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredRoundPrismT10DarkBlue
                    id: 2126078057,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(1727367593), // RareRoundPrismT10
                FromConversionData::Simple(311721883), // WolfRoundPrismT10
                FromConversionData::Simple(3504767078), // QuarterbackRoundPrismT10
                FromConversionData::Complex { // ArmoredRoundPrismT10C6B
                    id: 1018457748,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredRoundPrismT10MatteWhite
                    id: 2067295657,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredRoundPrismT10MatteRed
                    id: 1097117137,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredRoundPrismT10C6O
                    id: 3328512085,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredRoundPrismT10SType
                    id: 1503181717,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredRoundPrismT10MatteGreen
                    id: 107293569,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredRoundPrismT10MatteBlue
                    id: 1824768912,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredRoundPrismT10MattePurple
                    id: 1911941380,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                // no equivalent translation
            ],
            to: None,
        },
        676446887 => only_to(447126572), // CompactEdgeRound to ArmoredPrismRoundT10
        368051733 => CubeConversionData {
            from: vec![
                FromConversionData::Simple(366006), // special april fools heavy round inner
            ],
            to: Some(ToConversionData::Simple(447126572)), // ArmoredPrismRoundT10
        },
        1993583577 => only_to(447126572), // LightEdgeRound
        2623263862 => only_to(447126572), // NeonEdgeRound
        3629216992 => reciprocal(2407011807), // GlassEdgeRound to WindshieldRoundPrismT10
        // corner round aka round tetra
        2589418111 => CubeConversionData { // MediumCornerRound
            from: vec![
                FromConversionData::Simple(2589418111), // ArmoredTetraRoundT10
                FromConversionData::Complex { // ArmoredRoundTetraT10Orange
                    id: 903181224,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredRoundTetraT10Yellow
                    id: 2073756076,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredRoundTetraT10Gold
                    id: 420696,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(2089416063), // HallowTetraRoundT10
                FromConversionData::Complex { // ArmoredRoundTetraT10Blue
                    id: 603737992,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredRoundTetraT10DarkBlue
                    id: 2126078058,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(1727367594), // RareRoundTetraT10
                FromConversionData::Simple(311721884), // WolfRoundTetraT10
                FromConversionData::Simple(3504767079), // QuarterbackRoundTetraT10
                FromConversionData::Complex { // ArmoredRoundTetraT10C6B
                    id: 4200979110,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredRoundTetraT10MatteWhite
                    id: 904092408,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredRoundTetraT10MatteRed
                    id: 2704843488,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredRoundTetraT10C6O
                    id: 11838567,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredRoundTetraT10SType
                    id: 1503181718,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredRoundTetraT10MatteGreen
                    id: 107293570,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredRoundTetraT10MatteBlue
                    id: 1824768913,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredRoundTetraT10MattePurple
                    id: 1911941381,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                // no equivalent translation
            ],
            to: None,
        },
        1728084091 => only_to(2589418111), // CompactCornerRound to ArmoredTetraRoundT10
        2148353926 => CubeConversionData { // HeavyCornerRound
            from: vec![
                FromConversionData::Simple(366005), // special april fools heavy round tetra
            ],
            to: Some(ToConversionData::Simple(2589418111)), // ArmoredTetraRoundT10
        },
        582052348 => only_to(2589418111), // LightCornerRound
        3195590950 => only_to(2589418111), // NeonCornerRound
        2380494106 => reciprocal(1907975281), // GlassCornerRound to WindshieldRoundTetraT10
        // inner slope aka sloped inner
        495400745 => CubeConversionData { // MediumInnerSlope
            from: vec![
                FromConversionData::Simple(914659854), // ArmoredSlopedInnerT10
                FromConversionData::Complex { // ArmoredSlopedInnerT10Orange
                    id: 903181231,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredSlopedInnerT10Yellow
                    id: 2073756084,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredSlopedInnerT10Gold
                    id: 420703,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(1837280720), // HallowInnerSlopedT10
                FromConversionData::Complex { // ArmoredSlopedInnerT10Blue
                    id: 603737999,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredSlopedInnerT10DarkBlue
                    id: 2126078065,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(1727367601), // RareSlopedInnerT10
                FromConversionData::Simple(311721891), // WolfSlopedInnerT10
                FromConversionData::Simple(3504767086), // QuarterbackSlopedInnerT10
                FromConversionData::Complex { // ArmoredSlopedInnerT10C6B
                    id: 1558067037,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredSlopedInnerT10MatteWhite
                    id: 914659772,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredSlopedInnerT10MatteRed
                    id: 865911714,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredSlopedInnerT10C6O
                    id: 2786019728,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredSlopedInnerT10SType
                    id: 1503181725,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredSlopedInnerT10MatteGreen
                    id: 107293577,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredSlopedInnerT10MatteBlue
                    id: 1824768920,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredSlopedInnerT10MattePurple
                    id: 1911941388,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                // no equivalent translation
            ],
            to: None,
        },
        1834966563 => only_to(914659854), // CompactInnerSlope to ArmoredSlopedInnerT10
        2111062867 => only_to(914659854), // HeavyInnerSlope
        2426642454 => only_to(914659854), // LightInnerSlope
        868027936 => only_to(914659854), // NeonInnerSlope
        // no glass inner slope
        // edge slope aka sloped prism
        1538778047 => CubeConversionData { // MediumEdgeSlope
            from: vec![
                FromConversionData::Simple(914659852), // ArmoredSlopedPrismT10
                FromConversionData::Complex { // ArmoredSlopedPrismT10Orange
                    id: 903181229,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredSlopedPrismT10Yellow
                    id: 2073756082,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredSlopedPrismT10Gold
                    id: 420701,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(1837280718), // HallowPrismSlopedT10
                FromConversionData::Complex { // ArmoredSlopedPrismT10Blue
                    id: 603737997,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredSlopedPrismT10DarkBlue
                    id: 2126078063,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(1727367599), // RareSlopedPrismT10
                FromConversionData::Simple(311721889), // WolfSlopedPrismT10
                FromConversionData::Simple(3504767084), // QuarterbackSlopedPrismT10
                FromConversionData::Complex { // ArmoredSlopedPrismT10C6B
                    id: 1558067035,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredSlopedPrismT10MatteWhite
                    id: 914659770,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredSlopedPrismT10MatteRed
                    id: 865911712,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredSlopedPrism10C6O
                    id: 2786019730,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredTetraT10SType [sic?]
                    id: 1503181723,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredSlopedPrismT10MatteGreen
                    id: 107293575,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredSlopedPrismT10MatteBlue
                    id: 1824768918,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredSlopedPrismT10MattePurple
                    id: 1911941386,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                // no equivalent translation
            ],
            to: None,
        },
        1915435470 => only_to(914659852), // CompactEdgeSlope to ArmoredSlopedPrismT10
        1334508924 => only_to(914659852), // HeavyEdgeSlope
        3093572869 => only_to(914659852), // LightEdgeSlope
        3324063519 => only_to(914659852), // NeonEdgeSlope
        // corner slope aka sloped tetra
        557016830 => CubeConversionData { // MediumCornerSlope
            from: vec![
                FromConversionData::Simple(914659852), // ArmoredSlopedTetraT10
                FromConversionData::Complex { // ArmoredSlopedTetraT10Orange
                    id: 903181230,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredSlopedTetraT10Yellow
                    id: 2073756083,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredSlopedTetraT10Gold
                    id: 420702,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(1837280719), // HallowTetraSlopedT10
                FromConversionData::Complex { // ArmoredSlopedTetraT10Blue
                    id: 603737998,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredSlopedTetraT10DarkBlue
                    id: 2126078064,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(1727367600), // RareSlopedTetraT10
                FromConversionData::Simple(311721890), // WolfSlopedTetraT10
                FromConversionData::Simple(3504767085), // QuarterbackSlopedTetraT10
                FromConversionData::Complex { // ArmoredSlopedTetraT10C6B
                    id: 1558067036,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredSlopedTetraT10MatteWhite
                    id: 914659771,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredSlopedTetraT10MatteRed
                    id: 865911713,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredSlopedTetraT10C6O
                    id: 2786019729,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredSlopedTetraT10SType
                    id: 1503181724,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredSlopedTetraT10MatteGreen
                    id: 107293576,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredSlopedTetraT10MatteBlue
                    id: 1824768919,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredSlopedTetraT10MattePurple
                    id: 1911941387,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                // no equivalent translation
            ],
            to: None,
        },
        1881278783 => only_to(914659853), // CompactCornerSlope to ArmoredSlopedTetraT10
        3665280239 => only_to(914659853), // HeavyCornerSlope
        3959877408 => only_to(914659853), // LightCornerSlope
        3825345103 => only_to(914659853), // NeonCornerSlope
        // corner aka tetra
        126353766 => CubeConversionData { // CompactCorner
            from: vec![
                FromConversionData::Simple(1362505337), // ArmoredTetraTX
                FromConversionData::Complex { // GoldArmouredTetraTX
                    id: 734598081,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
            ],
            to: None,
        },
        222074329 => CubeConversionData { // HeavyCorner
            from: vec![
                FromConversionData::Simple(2157492751), // HeavyChassisTetra
                FromConversionData::Simple(366002), // special april fools heavy tetra
            ],
            to: Some(ToConversionData::Simple(2157492751)),
        },
        1931676396 => CubeConversionData { // MediumCorner
            from: vec![
                FromConversionData::Simple(1931676396), // ArmoredTetraT10
                FromConversionData::Complex { // ArmoredTetraT10Orange
                    id: 903181221,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredTetraT10Yellow
                    id: 2073756073,
                    offset: NO_OFFSET,
                    colour: YELLOW,
                },
                FromConversionData::Complex { // ArmoredTetraT10Gold
                    id: 420693,
                    offset: NO_OFFSET,
                    colour: GOLD,
                },
                FromConversionData::Simple(1857534838), // HallowTetraT10
                FromConversionData::Complex { // ArmoredTetraT10Blue
                    id: 603737989,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredTetraT10DarkBlue
                    id: 2126078055,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Simple(3188616903), // RareArmoredTetraT3
                FromConversionData::Simple(3224129949), // WolfTetraT5
                FromConversionData::Simple(578334861), // QuarterbackTetraT4
                FromConversionData::Complex { // ArmoredTetraT10C6B
                    id: 2388981877,
                    offset: NO_OFFSET,
                    colour: BLACK,
                },
                FromConversionData::Complex { // ArmoredTetraT10MatteWhite
                    id: 904092407,
                    offset: NO_OFFSET,
                    colour: WHITE,
                },
                FromConversionData::Complex { // ArmoredTetraT10MatteRed
                    id: 2704843487,
                    offset: NO_OFFSET,
                    colour: RED,
                },
                FromConversionData::Complex { // ArmoredTetraT10C6O
                    id: 1575105860,
                    offset: NO_OFFSET,
                    colour: ORANGE,
                },
                FromConversionData::Complex { // ArmoredTetraT10SType
                    id: 1503181715,
                    offset: NO_OFFSET,
                    colour: STYPE,
                },
                FromConversionData::Complex { // ArmoredTetraT10MatteGreen
                    id: 107293567,
                    offset: NO_OFFSET,
                    colour: GREEN,
                },
                FromConversionData::Complex { // ArmoredTetraT10MatteBlue
                    id: 1824768910,
                    offset: NO_OFFSET,
                    colour: BLUE,
                },
                FromConversionData::Complex { // ArmoredTetraT10MattePurple
                    id: 1911941378,
                    offset: NO_OFFSET,
                    colour: PURPLE,
                },
                // TODO colour-match the tiered cubes
                FromConversionData::Simple(3265763652), // ArmoredTetraT9
                FromConversionData::Simple(3292449768), // ArmoredTetraT8
                FromConversionData::Simple(2041910372), // ArmoredTetraT7
                FromConversionData::Simple(3245247734), // ArmoredTetraT6
                FromConversionData::Simple(3467171168), // ArmoredTetraT5
                FromConversionData::Simple(1987268082), // ArmoredTetraT4
                FromConversionData::Simple(1930795608), // ArmoredTetraT3
                FromConversionData::Simple(2941587199), // ArmoredTetraT2
                FromConversionData::Simple(3288882012), // ArmoredTetraT1
                // end tiered
                // no equivalent translation
            ],
            to: None,
        },
        1347255791 => reciprocal(1754827363), // LightCorner to LightChassisTetra
        1094997616 => CubeConversionData { // GlassCorner
            from: vec![
                FromConversionData::Simple(1593042212), // WindshieldTetraT10
                FromConversionData::Simple(1094997616), // WindshieldTetra
            ],
            to: None,
        },
        2333317284 => only_to(1931676396), // NeonCorner
        1387792676 => only_to(1931676396), // RetroCorner
        // Helium (sort of a chassic cube)
        3226650954 => CubeConversionData { // Helium
            from: vec![
                FromConversionData::Simple(3425997036),
                FromConversionData::Simple(3305279060),
                FromConversionData::Simple(1914654544),
                FromConversionData::Simple(2001668174),
                FromConversionData::Simple(3226650954),
            ],
            to: Some(ToConversionData::Simple(3226650954)),
        },
        // country flags
        1385335421 => CubeConversionData { // Canada (also  used as fallback country flag)
            from: vec![
                FromConversionData::Simple(1385335421), // Canada
                FromConversionData::Simple(1958111701), // DominicanRepublic
                FromConversionData::Simple(1958111702), // Egypt
                FromConversionData::Simple(1958111703), // Estonia
                FromConversionData::Simple(1958111704), // Greece
                FromConversionData::Simple(1958111705), // HongKong
                FromConversionData::Simple(1958111706), // Hungary
                FromConversionData::Simple(1958111708), // Latvia
                FromConversionData::Simple(1958111709), // Liechtenstein
                FromConversionData::Simple(1958111710), // Lithuania
                FromConversionData::Simple(1958111711), // Mongolia
                FromConversionData::Simple(1958111712), // Sardinia
                FromConversionData::Simple(1958111713), // Taiwan
                FromConversionData::Simple(1958111714), // Thailand
                //FromConversionData::Simple(1958111715), // Turkey
                // Unknown countries
                FromConversionData::Simple(1395013882), // ???
                FromConversionData::Simple(2877144578), // ???
                FromConversionData::Simple(1875812779), // ???
                FromConversionData::Simple(2997211596), // ???
                FromConversionData::Simple(987493977), // ???
                FromConversionData::Simple(1122928047), // ???
                FromConversionData::Simple(1971217946), // ???
                FromConversionData::Simple(766565997), // ???
            ],
            to: None,
        },
        4065608086 => only_to(68277216), // Russia to Ukraine
        68277216 => CubeConversionData { // Ukraine
            from: vec![
                FromConversionData::Simple(68277216), // Ukraine
                FromConversionData::Simple(4065608086), // Russia
            ],
            to: Some(ToConversionData::Simple(68277216)),
        },
        4175295691 => reciprocal(4175295691), // UK
        3559676835 => reciprocal(3559676835), // USA
        2020064218 => reciprocal(2020064218), // Japan
        2730691972 => reciprocal(2730691972), // Poland
        2596479965 => reciprocal(2596479965), // Germany
        3330651823 => reciprocal(3330651823), // Kazakhstan
        1768151340 => reciprocal(1768151340), // Belarus
        4166201860 => reciprocal(4166201860), // Italy
        3447406366 => reciprocal(3447406366), // Ireland
        664490215 => reciprocal(664490215), // Australia
        3114896263 => reciprocal(3114896263), // Spain
        2243089188 => reciprocal(2243089188), // Brazil
        88051662 => reciprocal(88051662), // Netherlands
        354715741 => reciprocal(354715741), // Argentina
        1786715320 => reciprocal(1786715320), // Sweden
        3910121366 => reciprocal(3910121366), // China
        2373812955 => reciprocal(2373812955), // France
        460733827 => reciprocal(460733827), // South Korea
        // newer country flags (added after RC15)
        2855651676 => only_to(1385335421), // New Zealand to Canada
        932603502 => only_to(1385335421), // Denmark to Canada
        1796059677 => only_to(1385335421), // IcelandHoloflag
        1798659636 => only_to(1385335421), // BelgiumHoloFlag
        4070927930 => reciprocal(1958111715), // Turkiye/Turkey has a classic equivalent
        // general flags
        3024755361 => reciprocal(3024755361), // Humble Bundle
        1039436321 => reciprocal(1039436321), // Curse
        1958111892 => CubeConversionData { // Robocraft (also used as fallback flag)
            from: vec![
                FromConversionData::Simple(1958111892), // Robocraft
                // These are all sequential, so likely added by client mods
                //FromConversionData::Simple(1958111896), // pirate
                FromConversionData::Simple(1958111895), // mars
                FromConversionData::Simple(1958111897), // ???
                FromConversionData::Simple(1958111898), // ???
                FromConversionData::Simple(1958111901), // anniversary flag ???
                FromConversionData::Simple(1958111902), // anniversary flag ???
                FromConversionData::Simple(1958111903), // anniversary flag ???
                FromConversionData::Simple(1958111904), // anniversary flag ???
                FromConversionData::Simple(1754621522), // hex badge ???
                FromConversionData::Simple(1754621523), // hex badge ???
                FromConversionData::Simple(1754621523), // hex badge ???
                FromConversionData::Simple(1958111893), // ???
                FromConversionData::Simple(1958111707), // unused (for now?)
                FromConversionData::Simple(1958111897), // E14
                FromConversionData::Simple(1958111898), // CF-Alliance
            ],
            to: Some(ToConversionData::Simple(1958111892)),
        },
        4200145444 => reciprocal(4200145444), // Dev Supporter
        3370765914 => reciprocal(3370765914), // Gold Dev Supporter
        3996493866 => reciprocal(1754621525), // 3YearsHoloflag (to badge equiv)
        507555577 => reciprocal(1754621526), // 4YearsHoloflag (to badge equiv)
        4124816651 => reciprocal(1958111896), // PirateHoloflag
        // non-translatable flags
        3411783298 => only_to(1385335421), // RoboPassSeason2Holoflag
        2663263630 => only_to(1385335421), // BunnyHoloflag
        3131616996 => only_to(1385335421), // EasterEggHoloflag
        3266439135 => only_to(1385335421), // FlowersHoloflag
        216301182 => only_to(1385335421), // 2019Holoflag
        2412453676 => only_to(1385335421), // CandyCaneHoloflag
        2990880144 => only_to(1385335421), // SantaCrayHoloflag
        3058601525 => only_to(1385335421), // RoboPassSeason1Holoflag
        327459709 => only_to(1385335421), // SnowflakeHoloflag
        2537158398 => only_to(1385335421), // 6YearsHoloflag
        757806708 => only_to(1385335421), // TopHundredHoloflag
        1020482017 => only_to(1385335421), // OverwolfHoloflag
        1607707925 => only_to(1385335421), // AbleGamersCharityHoloflag
        4079824960 => only_to(1385335421), // YogscastHoloflag
        862329300 => only_to(1385335421), // ChronoGGHoloflag
        1042028710 => only_to(1385335421), // IntelHoloflag/Nyan Cray/Rainbow
        2345098755 => only_to(1385335421), // AlienwareHoloflag
        3210649465 => only_to(1385335421), // 5YearsHoloflag
        // badges
        1301942811 => only_to(1754621522), // BronzeLeagueBadge to some badge (yes I know league badges aren't the same shape)
        4157599523 => only_to(1754621522), // SilverLeagueBadge
        1893098069 => only_to(1754621522), // GoldLeagueBadge
        608517870 => only_to(1754621522), // DiamondLeagueBadge
        2046961989 => only_to(1754621522), // ProtoniumLeagueBadge
        1988022506 => only_to(1754621522), // ProtoniumLeagueFiveStarsBadge
        1551110695 => only_to(1754621522), // SilverLeagueBadgeArena
        2514755079 => only_to(1754621522), // GoldLeagueBadgeArena
        988387568 => only_to(1754621522), // BronzeLeagueBadgeArena
        // masks
        // cockpit
        2530225659 => reciprocal(2530225659),
        3970675339 => reciprocal(3970675339),
        1354711861 => reciprocal(1354711861),
        717544517 => reciprocal(717544517),
        2711624874 => reciprocal(2711624874),
        3688596442 => reciprocal(3688596442),
        // other masks
        451055140 => reciprocal(451055140), // ???
        1620644180 => reciprocal(1620644180), // ???
        3561288328 => reciprocal(3561288328), // ???
        2923392504 => reciprocal(2923392504), // ???
        2479898264 => reciprocal(2479898264), // ???
        3920339432 => reciprocal(3920339432), // ???
        2906446214 => reciprocal(2906446214), // ???
        3611772662 => reciprocal(3611772662), // ???
        1217647354 => reciprocal(1217647354), // ???
        854085002 => reciprocal(854085002), // ???
        604924218 => reciprocal(604924218), // ???
        1584789066 => reciprocal(1584789066), // ???
        4155139139 => reciprocal(4155139139), // ???
        2379348787 => reciprocal(2379348787), // ???
        2123945215 => reciprocal(2123945215), // ???
        82004879 => reciprocal(82004879), // ???
        1554012392 => reciprocal(1554012392), // ???
        651937688 => reciprocal(651937688), // ???
        1268201954 => reciprocal(1268201954), // ???
        837592722 => reciprocal(837592722), // ???
        1108501665 => reciprocal(1108501665), // ???
        946469841 => reciprocal(946469841), // ???
        1839067602 => reciprocal(1839067602), // ???
        400977570 => reciprocal(400977570), // ???
        1762483815 => reciprocal(1762483815), // ???
        326549783 => reciprocal(326549783), // ???
        670637166 => reciprocal(670637166), // ???
        1568834334 => reciprocal(1568834334), // ???
        438539744 => reciprocal(438539744), // ???
        1616382608 => reciprocal(1616382608), // ???
        4200383087 => reciprocal(4200383087), // ???
        2150063391 => reciprocal(2150063391), // ???
        3060186489 => reciprocal(3060186489), // ???
        3424461321 => reciprocal(3424461321), // ???
        1936618195 => reciprocal(1936618195), // ???
        152432035 => reciprocal(152432035), // ???
        2075163291 => reciprocal(2075163291), // ???
        30139883 => reciprocal(30139883), // ???
        2915102202 => reciprocal(2915102202), // ???
        3619385994 => reciprocal(3619385994), // ???
        665365647 => reciprocal(665365647), // ???
        1574122495 => reciprocal(1574122495), // ???
        // misc cosmetics
        688123941 => reciprocal(688123941), // SpikeDagger
        2782198650 => reciprocal(2782198650), // RobotNameBanner
        2820370110 => reciprocal(2820370110), // Altimeter
        3531325242 => reciprocal(3531325242), // Speedometer
        3610833057 => reciprocal(3610833057), // VaporTrailSingle
        3137009174 => only_to(3610833057), // VaporTrailSingleFlower
        1810319153 => only_to(3610833057), // VaporTrailSingleFirework
        3297983510 => only_to(3610833057), // VaporTrailSingleSnowflake
        236893100 => reciprocal(236893100), // VaporTrailTwin
        // electroplates
        3823106277 => reciprocal(3823106277), // ElectroshieldIL
        1889152405 => reciprocal(1889152405), // ElectroshieldIR
        3001609712 => reciprocal(3001609712), // ElectroshieldJL
        3365106304 => reciprocal(3365106304), // ElectroshieldJR
        991790322 => reciprocal(991790322), // ElectroshieldKL
        1097210754 => reciprocal(1097210754), // ElectroshieldKR
        726724318 => reciprocal(726724318), // T3ElectroShieldDLSpiked
        1361801646 => reciprocal(1361801646), // T3ElectroShieldDRSpiked
        3248328684 => reciprocal(3248328684), // T3ElectroShieldDLFootball
        3152416924 => reciprocal(3152416924), // T3ElectroShieldDRFootball
        // T1
        1264237222 => reciprocal(1264237222),
        2965095316 => reciprocal(2965095316),
        // T2
        2457663915 => reciprocal(2457663915),
        1771929753 => reciprocal(1771929753),
        // T3
        627128495 => reciprocal(627128495),
        3732947357 => reciprocal(3732947357),
        // T4
        540114865 => reciprocal(540114865),
        3687843459 => reciprocal(3687843459),
        // T5
        2536292021 => reciprocal(2536292021),
        1825425287 => reciprocal(1825425287),
        // T6
        1309343160 => reciprocal(1309343160),
        3052110986 => reciprocal(3052110986),
        // T7
        4179049660 => reciprocal(4179049660),
        49694094 => reciprocal(49694094),
        // T8
        1152021380 => reciprocal(1152021380),
        3209366198 => reciprocal(3209366198),
        // T9
        4088902272 => reciprocal(4088902272),
        139905970 => reciprocal(139905970),
        // Weapons
        // Nanos
        1761903423 => reciprocal(1761903423), // T2Nano
        1671695619 => variant_dual(1671695619, 3726197307), // T3Nano to T7/T8
        2364175165 => variant_dual(2364175165, 3568946183), // T4Nano to T9/T10
        // Plasma
        2655409492 => fallback_conversion(), // T0Plasma
        1293532710 => variant_dual(1412971690, 1293532710), // T1Plasma to T3/T4
        2446895092 => variant_dual(3810917806, 2446895092), // T2Plasma to T5/T6
        3953551555 => variant_dual(3866475184, 3953551555), // T3Plasma to T7/T8
        929526033 => variant_dual(1365801908, 929526033), // T4Plasma to T9/T10
        1593059007 => only_to(929526033), // T4PlasmaCarbon6
        171448327 => reciprocal(171448327), // T5Plasma (mega)
        2943284935 => reciprocal(2943284935), // T5PlasmaCarbon6 (mega)
        3594602569 => only_to(171448327), // T5PlasmaGolden
        // Rail
        1640043587 => reciprocal(1640043587), // T1Rail to T4
        2870850840 => variant_dual(360854742, 2870850840), // T2Rail to T5/T6
        1779831198 => variant_dual(282227656, 1779831198), // T3Rail to T7/T8
        2697638085 => variant_dual(2815406796, 2697638085), // T4Rail to T9/T10
        2083340214 => only_to(2697638085), // T5Rail to T10; mega does not exist in classic game
        447003593 => only_to(2697638085), // T5RailGolden
        // SMG front mounts
        3402350870 => variant_dual(1337963121, 3402350870), // T0FrontLaser to T1/T2
        2494440442 => variant_dual(776342052, 2494440442), // T1FrontLaser to T3/T4
        1584562849 => variant_dual(1497447090, 1584562849), // T2FrontLaser to T5/T6
        2675516967 => variant_dual(3341135633, 2675516967), // T3FrontLaser to T7/T8
        1436911484 => variant_dual(2955059079, 1436911484), // T4FrontLaser to T9/T10
        // SMG top mounts
        3809895395 => variant_dual(2929054885, 3809895395), // T0Laser to T1/T2
        3178463503 => variant_dual(2985370719, 3178463503), // T1Laser to T3/T4
        2007965780 => variant_dual(3338015945, 2007965780), // T2Laser to T5/T6
        3064235218 => variant_dual(1485996394, 3064235218), // T3Laser to T7/T8
        //2088248713 => variant_dual(798339580, 2088248713), // T4Laser to T9/T10
        2088248713 => CubeConversionData { // T4Laser
            from: vec![
                FromConversionData::Simple(798339580), // T9
                FromConversionData::Simple(2088248713), // T10
                FromConversionData::Simple(201410), // T10 retro
                FromConversionData::Simple(1201410), // T10 retro ???
                FromConversionData::Simple(8201410), // T10 retro gold
            ],
            to: Some(ToConversionData::Simple(2088248713)),
        },
        3632954889 => only_to(2088248713), // T4LaserCarbon6
        3209000021 => reciprocal(3209000021), // T5Laser (mega)
        1199458351 => reciprocal(1199458351), // T5LaserCarbon6
        3851710054 => only_to(3209000021), // T5LaserGolden
        // Tesla
        2017654588 => reciprocal(2017654588), // T2Tesla
        3479123512 => reciprocal(3479123512), // T3Tesla
        2703353899 => reciprocal(2703353899), // T4Tesla
        // Modern-only weapons
        3947892032 => fallback_conversion(), // T4Aeroflak
        73607413 => fallback_conversion(), // T5Aeroflak
        48150119 => fallback_conversion(), // T3Seeker
        350778877 => fallback_conversion(), // T4Seeker
        2419806013 => fallback_conversion(), // T5Seeker
        3437168371 => fallback_conversion(), // T5SeekerFirework
        3541341953 => fallback_conversion(), // T4Ion
        1184153980 => fallback_conversion(), // T5Ion
        2295212085 => fallback_conversion(), // T5IonCardLife
        105639928 => fallback_conversion(), // T5IonGolden
        1318129004 => fallback_conversion(), // T4Chaingun
        1795842454 => fallback_conversion(), // T5Chaingun
        1885850433 => fallback_conversion(), // T5Mortar
        194724715 => fallback_conversion(), // T5ChaingunGolden
        3832533619 => fallback_conversion(), // T5SeekerGolden
        2256680330 => fallback_conversion(), // T5MortarGolden
        3087866254 => fallback_conversion(), // MortarEggLauncher
        // TODO
        // Movement
        // Rudders
        2369778644 => reciprocal(2369778644), // T2Rudder to T4
        3808719325 => variant_dual(975319760, 3808719325), // T3Rudder to T5/T6
        3919904737 => variant_dual(1410969817, 3919904737), // T4Rudder to T7/T8
        2404311559 => reciprocal(2404311559), // T4RudderBat
        2608683011 => variant_dual(1589198565, 2608683011), // T5Rudder to T9/T10
        2928920064 => only_to(2608683011), // T5RudderBat
        // Wings/Aerofoils
        3980928804 => reciprocal(3980928804), // T2Wing to T4
        2205394221 => variant_dual(1515578912, 2205394221), // T3Wing to T5/T6
        2312317713 => variant_dual(879649833, 2312317713), // T4Wing to T7/T8
        2865954846 => reciprocal(2865954846), // T4WingBat
        2464192350 => variant_dual(1053747733, 2464192350), // T5Wing to T9/T10
        2863842421 => only_to(2464192350), // T5WingBat
        // Hover
        1851713730 => reciprocal(1851713730), // T0Hover to T2
        3170960304 => variant_dual(947628072, 3170960304), // T1Hover to T3/T4
        1639979618 => variant_dual(1333565630, 1639979618), // T2Hover to T5/T6
        465491285 => variant_dual(3508014365, 465491285), // T3Hover to T7/T8
        3347041415 => variant_dual(2787065227, 3347041415), // T4Hover to T10 Low/T10 Regular
        3835205776 => reciprocal(3835205776), // T5Hover (mega)
        2217471839 => only_to(3835205776), // T5HoverGolden
        // Ski
        347391203 => reciprocal(347391203), // T2Ski (why is this in hovers_common.xml!?)
        3080624795 => reciprocal(3080624795), // T2SteeringSki
        // Insect Legs
        2671394050 => reciprocal(2671394050), // T3InsectLegBladed
        1668760037 => variant_dual(1889322041, 1668760037), // T4InsectLeg to T9/T10 (Jumper)
        875393229 => only_to(1668760037), // T4InsectLegSpider
        127661231 => variant_dual(2535778622, 127661231), // T3InsectLeg to T7/T8 (Leaper)
        // Mech Legs
        3066502430 => fallback_conversion(), // T0MechLeg
        76192516 => fallback_conversion(), // T1MechLeg
        1789991181 => fallback_conversion(), // T2MechLeg
        1611765553 => reciprocal(706930465), // T3MechLeg to T10 Compact
        2452579137 => reciprocal(706930466), // T4MechLeg to T10
        4140335767 => reciprocal(706930467), // T5MechLeg to TX-1
        3660410821 => only_to(706930467), // T5MechLegGolden
        // Sprinter Legs
        3345472872 => fallback_conversion(), // T3SprinterLeg
        1169080311 => fallback_conversion(), // T4SprinterLeg
        2474131397 => fallback_conversion(), // T4SprinterLegCarbon6
        // Rotors
        3947193935 => variant_dual(845358402, 3947193935), // T2Rotor to T5/T6
        3789940851 => variant_dual(1549313867, 3789940851), // T3Rotor to T7/T8
        1802864184 => variant_dual(1459104119, 1802864184), // T4Rotor to T9/T10
        // Thrusters
        844778245 => variant_dual(2766875446, 844778245), // T1Thruster to T1/T2
        2498076128 => variant_dual(4276453881, 2498076128), // T2Thruster to T3/T4
        3764614578 => variant_dual(1572019677, 3764614578), // T3Thruster to T5/T6
        3938712462 => variant_dual(3285682302, 3938712462), // T4Thruster to T7/T8
        2969949587 => variant_dual(3033577704, 2969949587), // T5Thruster to T9/T10
        944347375 => reciprocal(944347375), // T5ThrusterCarbon6
        // Tracks
        4050988656 => fallback_conversion(), // T1TankTrack
        1691117057 => variant_dual(1637622047, 1691117057), // T2TankTrack to T3/T4
        184017928 => variant_dual(3553732357, 184017928), // T3TankTrack to T5/T6
        5728820 => variant_dual(3186238732, 5728820), // T4TankTrack to T7/T8
        431289744 => variant_dual(3075124016, 431289744), // T5TankTrack to T9/T10
        // NOTE: wheels are not the same IDs, presumably because of the wheels rework between classic and modern versions
        // Wheels steering
        932552373 => variant_dual(3769194911, 3158825472), // T0SteeringWheel to T5/T6
        169839823 => variant_dual(1526222854, 3539316064), // T1SteeringWheel to T5/T6
        2958657062 => variant_dual(3435681649, 1701822652), // T2SteeringWheel to T5/T6
        1177366035 => variant_dual(1868379119, 244400800), // T3SteeringWheel to T7/T8
        //345048522 => variant_dual(4184683672, 3104518012), // T4SteeringWheel to T9/T10
        345048522 => CubeConversionData { // T4SteeringWheel
            from: vec![
                FromConversionData::Simple(4184683672), // T9
                FromConversionData::Simple(3104518012), // T10
                FromConversionData::Simple(2682515905), // T10 Large
            ],
            to: Some(ToConversionData::Simple(3104518012)), // T10
        },
        32187483 => only_to(3104518012), // T4SteeringWheelCardLife
        48208435 => reciprocal(48208435), // T5SteeringWheel (mega)
        3546777424 => only_to(48208435), // T5SteeringWheelGolden
        // Wheels non-steering
        1810876016 => variant_dual(2866040116, 299251682), // T0Wheel
        4172855773 => variant_dual(182430853, 2137276546), // T1Wheel
        2924196438 => variant_dual(2632952818, 3372100958), // T2Wheel
        3028949761 => variant_dual(1065013100, 2734731074), // T3Wheel
        //1223384335 => variant_dual(, ), // T4Wheel
        1223384335 => CubeConversionData { // T4Wheel
            from: vec![
                FromConversionData::Simple(2840250395), // T9
                FromConversionData::Simple(345686686), // T10
                FromConversionData::Simple(3625806184), // T10 Large
            ],
            to: Some(ToConversionData::Simple(345686686)), // T10 Large
        },
        2105254905 => only_to(345686686), // T4WheelCardLife
        3301162279 => reciprocal(3301162279), // T5Wheel (mega)
        2305706992 => only_to(3301162279), // T5WheelGolden
        // Modern-only movement
        3286209719 => fallback_conversion(), // T3Propeller
        2784549582 => fallback_conversion(), // T4Propeller
        // TODO
        // Jammers
        198393630 => reciprocal(3805965640), // JammerSmall
        3106939802 => reciprocal(1439149132), // JammerMedium
        //575087658 => reciprocal(1352133458), // JammerLarge
        575087658 => CubeConversionData { // JammerLarge
            from: vec![
                FromConversionData::Simple(1352133458), // T7
                FromConversionData::Simple(3884591702), // T9 has no modern equivalent
            ],
            to: Some(ToConversionData::Simple(1352133458)),
        },
        // Radars
        3782348815 => reciprocal(3123991804), // RadarSmall to T1
        2115785469 => reciprocal(591201606), // RadarMedium to T5
        3370519355 => reciprocal(1413101008), // RadarLarge to T9
        // Radar Receivers
        2330463949 => reciprocal(2460830034), // ReceiverSmall to T5
        2312945847 => reciprocal(2549943884), // ReceiverMedium to T7
        2742818297 => reciprocal(551604040), // ReceiverLarge to T9
        // Flippers aka alignment rectifiers (why does classic only have 1?)
        2297667466 => reciprocal(2014070946), // FlipperLarge
        2707915966 => only_to(2014070946), // FlipperSmall
        3083647184 => only_to(2014070946), // FlipperMedium
        // Seats
        2325944783 => reciprocal(1692614192), // PilotSeatMega to Mega seat
        2843561040 => reciprocal(3186524732), // PilotSeatCray to actual pilot seat
        1427441647 => only_to(3186524732), // PilotSeatGene to actual pilot seat
        2743059293 => only_to(3186524732), // PilotSeatRetro to actual pilot seat
        1061758673 => only_to(3186524732), // PilotSeatBunny
        // Modules
        1692614192 => only_to(1692614192), // T5EnergyModule to Mega Seat
        3263542661 => fallback_conversion(), // T5ShieldModule
        3987245394 => fallback_conversion(), // T5BlinkModule
        4230376397 => fallback_conversion(), // T5GhostModule
        658488560 => fallback_conversion(), // T5EmpModule
        344080878 => only_to(1413101008), // T5WindowmakerModule to Radar T9
        // Rods
        1818686721 => fallback_conversion(), // Rod
        1312806203 => fallback_conversion(), // RodShort
        3489647384 => fallback_conversion(), // RodLong
        2454932271 => fallback_conversion(), // RodArc
        3568811832 => fallback_conversion(), // RodArcShort
        394503911 => fallback_conversion(), // RodDiagonal2D
        1316425218 => fallback_conversion(), // RodDiagonal2DShort
        3705632066 => fallback_conversion(), // RodDiagonal3D
        1750720388 => fallback_conversion(), // RodPlus
        651695911 => fallback_conversion(), // RodSkewedPlus
        4265819694 => fallback_conversion(), // RodCross
        3950939011 => fallback_conversion(), // RodSpring (cosmetic rod)
        2828600518 => fallback_conversion(), // RodShortSpring (cosmetic rod)
        1051732041 => fallback_conversion(), // RodLongSpring (cosmetic rod)
        2183972675 => fallback_conversion(), // FairyLight (cosmetic rod)
        // Struts
        4262490755 => fallback_conversion(), // Strut
        1648206355 => fallback_conversion(), // StrutShort
        3084940372 => fallback_conversion(), // StrutLong
        743433251 => fallback_conversion(), // StrutArc
        2126360617 => fallback_conversion(), // StrutSlice
        2608938070 => fallback_conversion(), // StrutSliceShort
        3999885169 => fallback_conversion(), // StrutRamp
        14694903 => fallback_conversion(), // StrutDiagonal2D
        3562021587 => fallback_conversion(), // StrutDiagonal2DShort
        2028802540 => fallback_conversion(), // StrutDiagonal3DL
        2002864041 => fallback_conversion(), // StrutDiagonal3DR
        511164535 => fallback_conversion(), // StrutPlus
        1521447199 => fallback_conversion(), // StrutSkewedPlus
        787409845 => fallback_conversion(), // StrutCross
        4206005137 => fallback_conversion(), // StrutArcShort
        3903776284 => fallback_conversion(), // StrutRampShort
        1319987665 => fallback_conversion(), // StrutDiagonal3DLShort
        3923036903 => fallback_conversion(), // StrutDiagonal3DRShort
        // misc masks and mask-likes
        1704793982 => fallback_conversion(), // VigilantEyeL
        3558508089 => fallback_conversion(), // VigilantEyeR
        767825476 => fallback_conversion(), // CatEyeL
        683945560 => fallback_conversion(), // CatEyeR
        1609561971 => fallback_conversion(), // CyborgEye
        108680691 => fallback_conversion(), // SpikePin
        955529870 => fallback_conversion(), // SpikeNeedle
        1991524976 => fallback_conversion(), // SpikeClaw
        38268230 => fallback_conversion(), // EagleNeckL
        2017227318 => fallback_conversion(), // EagleNeckR
        2829693731 => fallback_conversion(), // EagleFeathersL
        3536989267 => fallback_conversion(), // EagleFeathersR
        1797279669 => fallback_conversion(), // EagleFaceL
        291229893 => fallback_conversion(), // EagleFaceR
        699401910 => fallback_conversion(), // AlienwareMaskEarL
        3136672710 => fallback_conversion(), // AlienwareMaskEarR
        3969591264 => fallback_conversion(), // AlienwareMaskHeadL
        2145317520 => fallback_conversion(), // AlienwareMaskHeadR
        1314825875 => fallback_conversion(), // AlienwareMaskJawL
        3709563875 => fallback_conversion(), // AlienwareMaskJawR
        1390502124 => fallback_conversion(), // NinjaMaskBandanaL
        3248797084 => fallback_conversion(), // NinjaMaskBandanaR
        989441293 => fallback_conversion(), // NinjaMaskHeadL
        2847701117 => fallback_conversion(), // NinjaMaskHeadR
        2503490917 => fallback_conversion(), // NinjaMaskL
        108914709 => fallback_conversion(), // NinjaMaskR
        799302218 => fallback_conversion(), // TrexHeadL
        981970925 => fallback_conversion(), // TrexHeadR
        4069461414 => fallback_conversion(), // TrexNoseL
        669458802 => fallback_conversion(), // TrexNoseR
        3618891370 => fallback_conversion(), // TrexJawL
        3670645745 => fallback_conversion(), // TrexJawR
        // silly blocks
        149580437 => fallback_conversion(), // ExhaustBlower
        3489925411 => fallback_conversion(), // ExhaustStackL
        1679554409 => fallback_conversion(), // ExhaustStackR
        138993629 => fallback_conversion(), // PresentSmall
        568116457 => fallback_conversion(), // PresentLarge
        2484686408 => fallback_conversion(), // BubbleBlower
        346272734 => fallback_conversion(), // Balloon
        // dev blocks
        3950293873 => fallback_conversion(), // ProtoniumCrystal
        2771514423 => fallback_conversion(), // Equalizer
        606866102 => fallback_conversion(), // ProtoniumClasp
        3951497291 => fallback_conversion(), // FusionTower
        unhandled_id => {
            log::warn!("Unimplemented modern ID {}", unhandled_id);
            fallback_conversion()
        }
    };
    cube.conversion = Some(conversion);
}

fn reciprocal(classic_id: u32) -> CubeConversionData {
    CubeConversionData {
        from: vec![
            FromConversionData::Simple(classic_id),
        ],
        to: Some(ToConversionData::Simple(classic_id)),
    }
}

fn only_to(classic_id: u32) -> CubeConversionData {
    CubeConversionData {
        from: Vec::default(),
        to: Some(ToConversionData::Simple(classic_id)),
    }
}

fn variant_dual(lower_tier: u32, higher_tier: u32) -> CubeConversionData {
    CubeConversionData {
        from: vec![
            FromConversionData::Simple(lower_tier),
            FromConversionData::Simple(higher_tier),
        ],
        to: Some(ToConversionData::Simple(higher_tier)),
    }
}

fn fallback_conversion() -> CubeConversionData {
    CubeConversionData {
        from: Vec::default(),
        to: Some(ToConversionData::Simple(2000000000)), // camera cube
    }
}

// these are positions in the array of colours, not the UI position... thanks FJ
const NO_OFFSET: Option<(i16, i16, i16)> = Some((0, 0, 0));
const ORANGE: u8 = 3;
//const ORANGE: u8 = 11;
const YELLOW: u8 = 6;
const GOLD: u8 = 6;
//const LIGHT_BLUE: u8 = 3;
const BLUE: u8 = 9;
//const DARK_BLUE: u8 = 24;
const WHITE: u8 = 0;
const BLACK: u8 = 4;
const RED: u8 = 5;
const STYPE: u8 = 6; // yellow-ish?
const GREEN: u8 = 7;
const PURPLE: u8 = 10;
