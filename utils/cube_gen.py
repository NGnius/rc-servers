#!/bin/env python3

import sys
import json

# weapon required data
WEAPONS = {
    "Laser" : {
        "T0": {
            "damage_inflicted": 42,
        },
        "T1": {
            "damage_inflicted": 420,
        },
        "T2": {
            "damage_inflicted": 4200,
        },
        "T3": {
            "damage_inflicted": 42000,
        },
        "T4": {
            "damage_inflicted": 420000,
        },
        "T5": {
            "damage_inflicted": 4200000,
        },
    },
}

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

def guess_category(name: str, sprite: str, cat: int) -> str:
    name = name.lower()
    sprite = sprite.lower()
    for variant in CATEGORIES:
        variant_sanitized = variant.lower()
        if variant_sanitized in name or variant_sanitized in sprite:
            return variant
    return CATEGORIES[0]

def guess_type(name: str, sprite: str, cat: int) -> str:
    category = guess_category(name, sprite, cat)
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
    return guess_tier_by_name_str_key(name) or guess_tier_by_size(sprite) or "NoTier"

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
        new_entry = {
            "id": int(cube["0 unsigned int itemCodeValue"]),
            "info": {
                "category": guess_category(cube["1 string nameStrKey"], cube["1 string spriteName"], int(cube["0 PersistentCubeData cubeData"]["0 int category"])),
                "placements": placements_to_int(cube["0 PersistentCubeData cubeData"]["0 CubeFaces selectableFaces"]),
                "stats": stats, # required
                "description": str(cube["1 string description"]), # required
                "size": guess_tier(cube["1 string nameStrKey"], cube["1 string spriteName"]), # required
                "type": guess_type(cube["1 string nameStrKey"], cube["1 string spriteName"], int(cube["0 PersistentCubeData cubeData"]["0 int category"])),
                "active": int(cube["1 UInt8 active"]) != 0, # ignored
            },
            # ignored
            "spriteName": cube["1 string spriteName"],
            "nameStrKey": cube["1 string nameStrKey"],
            "mirrorCubeId": cube["0 PersistentCubeData cubeData"]["1 string mirrorCubeId"],
            "hexId": str(cube["1 string itemCode"]),
        }
        if "CPU LOAD" in stats:
            new_entry["info"]["cpu"] = int(stats["CPU LOAD"].strip().split(" ")[0].strip())
        if "ARMOR" in stats:
            new_entry["info"]["health"] = int(stats["ARMOR"].replace(",", "").strip())
        if len(new_entry["info"]["description"].strip()) == 0:
            new_entry["info"]["description"] = name + " (" + str(cube["0 unsigned int itemCodeValue"]) + "_10|" + str(cube["1 string itemCode"]) + "_16) without a description"
        if new_entry["info"]["category"] in WEAPONS and new_entry["info"]["type"] == "Weapon":
            if new_entry["info"]["size"] in WEAPONS[new_entry["info"]["category"]]:
                new_entry["weapon"] = WEAPONS[new_entry["info"]["category"]][new_entry["info"]["size"]]
        print(f"processed cube {i} into {new_entry}")
        cubes_out["cubes"][new_key] = new_entry
    with open("../assets/robocraft/cubes.json", "w") as f:
        json.dump(cubes_out, f, indent=4)
    print(f"processed {len(cubes)} cubes")

if __name__ == "__main__":
    main()
