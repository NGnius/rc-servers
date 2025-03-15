#!/bin/env python3

import sys
import json

# weapon required data
WEAPONS = {
    "Laser" : {
        "T0": {
            "damage_inflicted": 42,
            "group_fire_scales": [1.0],
        },
        "T1": {
            "damage_inflicted": 420,
            "group_fire_scales": [1.0],
        },
        "T2": {
            "damage_inflicted": 4200,
            "group_fire_scales": [1.0],
        },
        "T3": {
            "damage_inflicted": 42000,
            "group_fire_scales": [1.0],
        },
        "T4": {
            "damage_inflicted": 420000,
            "group_fire_scales": [1.0],
        },
        "T5": {
            "damage_inflicted": 4200000,
            "group_fire_scales": [1.0],
        },
    },
}

STATS_TRANSLATIONS = {
    "CPU LOAD": "strCPU",
    "CPU": "strCPU",
    "MASS": "strMass",
    "ARMOR": "strHealth",
    "ROBOT RANKING": "strRobotRanking",
    "ROBOT RATING": "strRobotRanking",
    "MAX LIFT": "strLiftDS",
    "LIFT": "strLiftDS",
    "MAX SPEED": "strMaxSpeedDS",
    "CARRYING CAPACITY": "strCapacity",
    "LOAD CAPACITY PER WING": "strCapacity",
    "TOP SPEED": "strMaxSpeedDS",
    "DAMAGE AT 160M": "strDamageNearDS",
    "DAMAGE AT 320M": "strDamageFarDS",
    "DAMAGE": "strDamageDS",
    "BLAST": "strBlastRadiusDS",
    "DAMAGE RATE": "strWeaponDamageRateDS",
    "HEAL RATE": "strHealRate",
}

STATS_VALUE_REPLACEMENTS = {
    "pFLOP": "[strPFlops]",
    "Kg": "[strKilograms]",
    "kg": "[strKilograms]",
}

IGNORED_STATS = [
    "OVERCLOCK",
    "OVERCLOCKER",
    "THRUST",
    "SHIELD",
    "LIGHT OUTPUT",
]

# different from the enum
'''CATEGORIES = {
    0: "NotAFunctionalItem",
    1: "Wheel",
    2: "Hover",
    3: "Wing",
    4: "Rudder",
    5: "Thruster",
    6: "InsectLeg",
    7: "MechLeg",
    8: "Ski",
    9: "TankTrack",
    10: "Rotor",
    11: "SprinterLeg",
    12: "Propeller",
    100: "Laser",
    200: "Plasma",
    250: "Mortar",
    300: "Rail",
    400: "Nano",
    500: "Tesla",
    600: "Aeroflak",
    650: "Ion",
    701: "Seeker",
    750: "Chaingun",
    800: "ShieldModule",
    801: "GhostModule",
    802: "BlinkModule",
    803: "EmpModule",
    804: "WindowmakerModule",
    900: "EnergyModule",
}'''
CATEGORIES = [
    "NotAFunctionalItem",
    "Wheel",
    "Hover",
    "Wing",
    "Rudder",
    "Thruster",
    "InsectLeg",
    "MechLeg",
    "Ski",
    "TankTrack",
    "Rotor",
    "SprinterLeg",
    "Propeller",
    "Laser", #13
    "Plasma",
    "Mortar",
    "Rail",
    "Nano",
    "Tesla",
    "Aeroflak",
    "Ion",
    "Seeker",
    "Chaingun",
    "ShieldModule", #23
    "GhostModule",
    "BlinkModule",
    "EmpModule",
    "WindowmakerModule",
    "EnergyModule",
]

CATEGORY_IGNORES = [
    "VaporTrail",
    "Vapor_Trail",
    "FusionTower",
]

ALL_FACES = 63;

CATEGORIES_PLACEMENTS = {
    "NotAFunctionalItem": ALL_FACES,
    "Wheel": 0b00001100,
    "Hover": 0b00111100,
    "Wing": 0b00111100,
    "Rudder": 0b00111100,
    "Thruster": ALL_FACES,
    "InsectLeg": 0b00111100,
    "MechLeg": 0b00000011,
    "Ski": 0b00000011,
    "TankTrack": 0b00111100,
    "Rotor": 0b00111100,
    "SprinterLeg": 0b00000011,
    "Propeller": ALL_FACES,
    "Laser": None,
    "Plasma": ALL_FACES,
    "Mortar": 0b00000011,
    "Rail": ALL_FACES,
    "Nano": ALL_FACES,
    "Tesla": ALL_FACES,
    "Aeroflak": ALL_FACES,
    "Ion": ALL_FACES,
    "Seeker": ALL_FACES,
    "Chaingun": ALL_FACES,
    "ShieldModule": ALL_FACES,
    "GhostModule": ALL_FACES,
    "BlinkModule": ALL_FACES,
    "EmpModule": ALL_FACES,
    "WindowmakerModule": ALL_FACES,
    "EnergyModule": ALL_FACES,
}

def guess_category(name: str, sprite: str) -> str:
    name = name.lower()
    sprite = sprite.lower()
    for variant in CATEGORIES:
        variant_sanitized = variant.lower()
        if (variant_sanitized in name and not str_contains_any(name, CATEGORY_IGNORES)) or (variant_sanitized in sprite and not str_contains_any(sprite, CATEGORY_IGNORES)):
            return variant
    return CATEGORIES[0]

def str_contains_any(s: str, l: list) -> bool:
    for variant in l:
        variant_sanitized = variant.lower()
        if variant_sanitized in s:
            print(s + " contains " + variant)
            return True
    return False


def guess_type(category: str) -> str:
    cat_i = CATEGORIES.index(category)
    if cat_i == 0:
        return "NotAFunctionalItem"
    elif cat_i >= 23:
        return "Module"
    elif cat_i >= 13:
        return "Weapon"
    elif cat_i >= 1:
        return "Movement"
    else:
        return "NotAFunctionalItem"

def guess_tier(name: str, sprite: str) -> str:
    if guess_category(name, sprite) == "NotAFunctionalItem" and "medium" in name.lower(): # Medium cube variants
        return "NoTier"
    return guess_tier_by_name_str_key(name) or guess_tier_by_size(sprite) or guess_tier_by_name_str_key(sprite) or "NoTier"

def guess_tier_by_size(sprite: str) -> str:
    sprite = sprite.lower()
    if "tiny" in sprite:
        return "T0"
    elif "small" in sprite:
        return "T1"
    elif "medium" in sprite:
        return "T2"
    elif "large" in sprite:
        return "T3"
    elif "huge" in sprite:
        return "T4"
    elif "mega" in sprite:
        return "T5"
    else:
        return None

def guess_tier_by_name_str_key(name: str) -> str:
    name = name.upper()
    if "T0" in name:
        return "T0"
    elif "T1" in name:
        return "T1"
    elif "T2" in name:
        return "T2"
    elif "T3" in name:
        return "T3"
    elif "T4" in name:
        return "T4"
    elif "T5" in name:
        return "T5"
    else:
        return None

def placements_to_int(placements: dict) -> int:
    return int(placements["1 UInt8 up"]) | \
    int(placements["1 UInt8 down"]) << 1 | \
    int(placements["1 UInt8 left"]) << 2 | \
    int(placements["1 UInt8 right"]) << 3 | \
    int(placements["1 UInt8 back"]) << 4 | \
    int(placements["1 UInt8 front"]) << 5

def guess_placement(placements: dict, category: str, name: str) -> int:
    by_category = CATEGORIES_PLACEMENTS[category]
    if by_category is not None:
        return by_category
    name = name.lower()
    if "front" in name:
        return 0b0011000000
    elif category == "Laser":
        return ALL_FACES
    return placements_to_int(placements)

def translate_stat_key(key: str) -> str:
    return STATS_TRANSLATIONS[key.upper()]

def replace_stat_values(value: str) -> str:
    for replace in STATS_VALUE_REPLACEMENTS.keys():
        value = value.replace(replace, STATS_VALUE_REPLACEMENTS[replace])
    return value

VARIANT_STRINGS = [
    "frontlaser",
    "golden",
    "carbon6",
    "egglauncher",
    "cardlife",
    "seekerfirework",
    "rudderbat",
    "wingbat",
    "legspider",
]

def is_variant_guess(name: str, sprite: str) -> bool:
    name_lower = name.lower()
    for s in VARIANT_STRINGS:
        if s in name_lower:
            return True
    return False

def main():
    print(sys.argv)
    filename_in = sys.argv[1]
    with open(filename_in) as f:
        cubes_asset = json.load(f)
    name = cubes_asset["0 MonoBehaviour Base"]["1 string m_Name"]
    print(f"found name (expected to be empty): `{name}`")
    cubes = cubes_asset["0 MonoBehaviour Base"]["0 CubeTypeData cubeTypes"]["0 Array Array"]
    print(f"found {len(cubes)} cubes to process")
    cubes_out = {
        "cubes": dict(),
        "movement": dict(),
        "lerp_value": 10.0,
    }
    last_tech_tree_id = 0
    tech_tree_index = 0
    tech_tree_specials_index = 0
    for i in range(len(cubes)):
        #print(f"processing cube {i}")
        cube = cubes[i]["0 CubeTypeData data"]
        name = str(cube["1 string nameStrKey"])
        if name.startswith("str"):
            name = name[3:]
        if name.endswith("Name"):
            name = name[:-4]
        new_key = name + " hexCode:" + str(cube["1 string itemCode"]) + " intCode:" + str(cube["0 unsigned int itemCodeValue"])
        stats = dict()
        for stat_i in range(1, 7): # 1 to 6 (inclusive)
            stat_key = "1 string stat" + str(stat_i)
            if stat_key in cube:
                stat_val = cube[stat_key]
                #print(f"stat {stat_key} -> {stat_val}")
                if len(stat_val) == 0:
                    continue
                if " = " in stat_val:
                    new_stat_key = str(cube[stat_key]).split(" = ")[0]
                    new_stat_val = str(cube[stat_key]).split(" = ")[1]
                else:
                    new_stat_key = str(cube[stat_key]).split(": ")[0]
                    new_stat_val = str(cube[stat_key]).split(": ")[1]
                stats[new_stat_key] = new_stat_val
        category = guess_category(cube["1 string nameStrKey"], cube["1 string spriteName"]);
        new_entry = {
            "id": int(cube["0 unsigned int itemCodeValue"]),
            "info": {
                "category": category,
                "placements": guess_placement(cube["0 PersistentCubeData cubeData"]["0 CubeFaces selectableFaces"], category, cube["1 string nameStrKey"]),
                "stats": stats, # required
                "description": str(cube["1 string description"]), # required
                "size": guess_tier(cube["1 string nameStrKey"], cube["1 string spriteName"]), # required
                "type": guess_type(category),
                "active": int(cube["1 UInt8 active"]) != 0, # ignored
            },
            # ignored
            "spriteName": cube["1 string spriteName"],
            "nameStrKey": cube["1 string nameStrKey"],
            "mirrorCubeId": cube["0 PersistentCubeData cubeData"]["1 string mirrorCubeId"],
            "hexId": str(cube["1 string itemCode"]),
        }
        if "protonium" in name.lower():
            new_entry["info"]["protonium"] = True
        if "CPU LOAD" in stats:
            new_entry["info"]["cpu"] = int(stats["CPU LOAD"].strip().split(" ")[0].strip())
        if "ARMOR" in stats:
            new_entry["info"]["health"] = int(stats["ARMOR"].replace(",", "").strip())
        translated_stats = dict()
        for (key, val) in new_entry["info"]["stats"].items():
            if key not in IGNORED_STATS:
                trans_key = translate_stat_key(key)
                translated_stats[trans_key] = replace_stat_values(val)
        new_entry["info"]["stats"] = translated_stats
        if len(new_entry["info"]["description"].strip()) == 0:
            new_entry["info"]["description"] = name + " (" + str(cube["0 unsigned int itemCodeValue"]) + "_10|" + str(cube["1 string itemCode"]) + "_16) without a description"
        else:
            new_entry["info"]["description"] += " (" + str(cube["0 unsigned int itemCodeValue"]) + "_10|" + str(cube["1 string itemCode"]) + "_16)"

        is_original = not is_variant_guess(cube["1 string nameStrKey"], cube["1 string spriteName"])
        # weapons
        if new_entry["info"]["type"] == "Weapon":
            if is_original and "module" not in new_entry["info"]["category"].lower(): # ignore variants and modules
                print(name, new_entry["info"]["category"], new_entry["info"]["size"])
                tier_num = int(new_entry["info"]["size"][1])
                new_entry["weapon"] = {
                    "damage_inflicted": i,
                    "group_fire_scales": [1.0],
                }
                new_entry["weapon_upgrade"] = {
                    "xp": tier_num + 1.0,
                    "rating": tier_num,
                    "rank": tier_num,
                    "power": 0,
                }
            if new_entry["info"]["category"] in WEAPONS:
                if new_entry["info"]["size"] in WEAPONS[new_entry["info"]["category"]]:
                    new_entry["weapon"] = WEAPONS[new_entry["info"]["category"]][new_entry["info"]["size"]]
        new_entry["info"]["ignore_in_weapon_list"] = new_entry["info"]["type"] != "Weapon"

        # tech tree
        if new_entry["info"]["category"] != "NotAFunctionalItem" and is_original:
            if last_tech_tree_id != 0:
                neighbours = [last_tech_tree_id]
            else:
                neighbours = []
            last_tech_tree_id = new_entry["id"]
            new_entry["tree"] = {
                "position_x": tech_tree_index % 16,
                "position_y": tech_tree_index // 16,
                "tech_points": i,
                "neighbours": neighbours,
                "requires": neighbours,
            }
            tech_tree_index += 1
        if not is_original:
            new_entry["tree"] = {
                "position_x": tech_tree_specials_index % 16,
                "position_y": (tech_tree_specials_index // 16) + 8,
                "tech_points": i,
                "neighbours": [],
                "requires": [227205318], # default cube (MediumCube)
            }
            tech_tree_specials_index += 1

        print(f"processed cube {i} into {new_entry}")
        cubes_out["cubes"][new_key] = new_entry
    with open("../assets/robocraft/cubes.json", "w") as f:
        json.dump(cubes_out, f, indent=4)
    print(f"processed {len(cubes)} cubes")

if __name__ == "__main__":
    main()
