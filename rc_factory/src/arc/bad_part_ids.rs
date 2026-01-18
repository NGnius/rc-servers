use std::collections::HashSet;
use std::sync::OnceLock;

pub const BAD_IDS: &[u32] = &[
    11838567,
    31120410,
    31677565,
    46150324,
    78846819,
    81509462,
    184010445,
    237466288,
    293027766,
    314459661,
    338131112,
    457113130,
    467120522,
    496862599,
    501172250,
    543064582,
    551604040,
    578334861,
    591201606,
    592303238,
    676556157,
    721292346,
    731082751,
    784541921,
    816829485,
    845358402,
    890194475,
    928867324,
    974084625,
    1018457748,
    1090038504,
    1128696351,
    1168698870,
    1240629822,
    1309620559,
    1315241999,
    1325949898,
    1352133458,
    1362505337,
    1403070239,
    1413101008,
    1439149132,
    1459104119,
    1478755203,
    1493281059,
    1536521034,
    1549313867,
    1558067031,
    1562104498,
    1575105860,
    1598408503,
    1637622047,
    1669509564,
    1684930476,
    1702383688,
    1727367589,
    1730552257,
    1749849161,
    1787337066,
    1834283861,
    1857534838,
    1876483700,
    1877040659,
    2006058694,
    2048719521,
    2135735743,
    2234346032,
    2276594985,
    2287632793,
    2292045362,
    2324128786,
    2366806805,
    2367586215,
    2388981877,
    2460830034,
    2537767682,
    2549943884,
    2581522917,
    2626596603,
    2638983060,
    2648952126,
    2662788350,
    2752826359,
    2759478449,
    2766875446,
    2786019734,
    2792029874,
    2898948238,
    3003461135,
    3013040640,
    3014580839,
    3070139769,
    3075124016,
    3111721879,
    3123991804,
    3186238732,
    3186524732,
    3188616903,
    3197803309,
    3224129949,
    3293765065,
    3328512085,
    3360783547,
    3393310392,
    3439410085,
    3461973291,
    3504767074,
    3553732357,
    3567218872,
    3594602569,
    3607909941,
    3636797296,
    3662230097,
    3719092233,
    3799999755,
    3805965640,
    3837584923,
    3884591702,
    3918108448,
    3994839079,
    4039842231,
    4071959794,
    4162167502,
    4200979110,
    4277093178,
    3795589065, // Armored Cube T1
    3523011797, // Armored Cube T2
    1428281037, // Armored Cube T3
    148547544,  // Armored Cube T4
    3902577141, // Armored Cube T5
    3217516252, // Armored Cube T6
    1602312433, // Armored Cube T7
    3130466754, // Armored Cube T8
    936473914,  // Armored Cube T9
    1661086464, // Armored Inner T1
    140476067,  // Armored Inner T2
    3558861316, // Armored Inner T3
    3514383790, // Armored Inner T4
    1772266812, // Armored Inner T5
    1717894314, // Armored Inner T6
    3737084984, // Armored Inner T7
    1664400308, // Armored Inner T8
    2704279673, // Armored Inner T9
    3133051372, // Armored Prism T1
    3521365071, // Armored Prism T2
    228806888,  // Armored Prism T3
    147162946,  // Armored Prism T4
    2954764240, // Armored Prism T5
    3218622022, // Armored Prism T6
    117694164,  // Armored Prism T7
    3129475416, // Armored Prism T8
    3159626035, // Armored Prism T9
    3288882012, // Armored Tetra T1
    2941587199, // Armored Tetra T2
    1930795608, // Armored Tetra T3
    1987268082, // Armored Tetra T4
    3467171168, // Armored Tetra T5
    3245247734, // Armored Tetra T6
    2041910372, // Armored Tetra T7
    3292449768, // Armored Tetra T8
    3265763652, // Armored Tetra T9
    1654632428, // Heavy Chassis Cube
    4181414569, // Heavy Chassis Inner
    2686466102, // Heavy Chassis Prism
    2157492751, // Heavy Chassis Tetra
    2295832800, // Light Chassis Cube
    3920646105, // Light Chassis Inner
    2231909735, // Light Chassis Prism
    1754827363, // Light Chassis Tetra
    3305279060, // Armored Helium T4
    1914654544, // Armored Helium T6
    2001668174, // Armored Helium T8
    3769194911, // Wheel Steering T1
    3158825472, // Wheel Steering T2
    1526222854, // Wheel Steering T3
    3539316064, // Wheel Steering T4
    3435681649, // Wheel Steering T5
    1701822652, // Wheel Steering T6
    1868379119, // Wheel Steering T7
    244400800,  // Wheel Steering T8
    4184683672, // Wheel Steering T9
    3104518012, // Wheel Steering T10
    2866040116, // Wheel T1
    299251682,  // Wheel T2
    182430853,  // Wheel T3
    2137276546, // Wheel T4
    2632952818, // Wheel T5
    3372100958, // Wheel T6
    1065013100, // Wheel T7
    2734731074, // Wheel T8
    2840250395, // Wheel T9
    345686686,  // Wheel T10
    2535778622, // Walker Leg T7
    1889322041, // Walker Leg T9
    947628072,  // Hover Blade T3
    1333565630, // Hover Blade T5
    3508014365, // Hover Blade T7
    2787065227, // Hover Blade T9
    1515578912, // Aerofoil T5
    879649833,  // Aerofoil T7
    1053747733, // Aerofoil T9
    975319760,  // Rudder T5
    1410969817, // Rudder T7
    1589198565, // Rudder T9
    4276453881, // Thruster T3
    1572019677, // Thruster T5
    3285682302, // Thruster T7
    3033577704, // Thruster T9
    1337963121, // Front Mount SMG T1
    776342052,  // Front Mount SMG T3
    1497447090, // Front Mount SMG T5
    3341135633, // Front Mount SMG T7
    2955059079, // Front Mount SMG T9
    2929054885, // Top Mount SMG T1
    2985370719, // Top Mount SMG T3
    3338015945, // Top Mount SMG T5
    1485996394, // Top Mount SMG T7
    798339580,  // Top Mount SMG T9
    1412971690, // Plasma Launcher T3
    3810917806, // Plasma Launcher T5
    3866475184, // Plasma Launcher T7
    1365801908, // Plasma Launcher T9
    360854742,  // Rail Cannon T5
    282227656,  // Rail Cannon T7
    2815406796, // Rail Cannon T9
    3726197307, // Nanotech Disruptor T7
    3568946183, // Nanotech Disruptor T9
    2860532867, // Electroplate L T3
    2949645213, // Electroplate L T4
    416137881,  // Electroplate L T5
    3253436820, // Electroplate L T6
    1995849872, // Electroplate L T7
    3410694056, // Electroplate L T8
    2086063788, // Electroplate L T9
    3746649421, // Electroplate R T3
    3657503315, // Electroplate R T4
    1830605655, // Electroplate R T5
    3023736922, // Electroplate R T6
    52908382,   // Electroplate R T7
    3197771366, // Electroplate R T8
    159899490,  // Electroplate R T9
];

pub fn bad_part_id_set() -> &'static HashSet<u32> {
    static SET: OnceLock<HashSet<u32>> = OnceLock::new();
    SET.get_or_init(|| BAD_IDS.iter().copied().collect())
}
