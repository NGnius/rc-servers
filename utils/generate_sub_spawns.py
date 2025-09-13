#!/bin/env python3

import json
import argparse
import requests
import urllib

def print_transform(child_json):
    local_x = child_json["m_LocalPosition"]["m_X"]
    local_y = child_json["m_LocalPosition"]["m_Y"]
    local_z = child_json["m_LocalPosition"]["m_Z"]
    rot_w = child_json["m_LocalRotation"]["m_W"]
    rot_x = child_json["m_LocalRotation"]["m_X"]
    rot_y = child_json["m_LocalRotation"]["m_Y"]
    rot_z = child_json["m_LocalRotation"]["m_Z"]
    # valid Rust code (to be inserted into a vec![here])
    print(f"""SpawnPoint {{
    team: None,
    x: {local_x:.3f},
    y: {local_y:.3f},
    z: {local_z:.3f},
}}/*.with_rotation(num_quaternion::Quaternion {{
    x: {rot_x:.6f},
    y: {rot_y:.6f},
    z: {rot_z:.6f},
    w: {rot_w:.6f},
}})*/,""")

def gen_by_children(path: str, start: int, end: int):
    path_json = json.loads(path)
    url_path = str(path)
    if "\"" in path:
        url_path = urllib.parse.quote(path)
    asset_resp = requests.get(f"http://127.0.0.1:38723/Assets/Json?Path={url_path}")
    asset_json = asset_resp.json()
    if end is None:
        end = len(asset_json["m_Children"])
    for (i, child) in enumerate(asset_json["m_Children"][start:end]):
        #print(i, child)
        path_json["D"] = child["m_PathID"]
        url_path = urllib.parse.quote(json.dumps(path_json))
        child_resp = requests.get(f"http://127.0.0.1:38723/Assets/Json?Path={url_path}")
        child_json = child_resp.json()
        print_transform(child_json)

def gen_by_mono(path: str, start: int, end: int):
    path_json = json.loads(path)
    url_path = str(path)
    if "\"" in path:
        url_path = urllib.parse.quote(path)
    asset_resp = requests.get(f"http://127.0.0.1:38723/Assets/Json?Path={url_path}")
    asset_json = asset_resp.json()
    points = asset_json["m_Structure"]["spawningPoints"]
    if end is None:
        end = len(points)
    for (i, child) in enumerate(points):
        #print(i, child)
        path_json["D"] = child["m_PathID"]
        url_path = urllib.parse.quote(json.dumps(path_json))
        gameobj_resp = requests.get(f"http://127.0.0.1:38723/Assets/Json?Path={url_path}")
        gameobj_json = gameobj_resp.json()
        path_json["D"] = gameobj_json["m_GameObject"]["m_PathID"]
        url_path = urllib.parse.quote(json.dumps(path_json))
        point_resp = requests.get(f"http://127.0.0.1:38723/Assets/Json?Path={url_path}")
        point_json = point_resp.json()
        path_json["D"] = point_json["m_Components"][0]["m_Component"]["m_PathID"]
        url_path = urllib.parse.quote(json.dumps(path_json))
        child_resp = requests.get(f"http://127.0.0.1:38723/Assets/Json?Path={url_path}")
        child_json = child_resp.json()
        print_transform(child_json)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--start", type=int, default=0)
    parser.add_argument("--end", type=int, default=None)
    parser.add_argument("--mono", action='store_true')
    parser.add_argument("initial_path")
    args = parser.parse_args()
    if args.mono:
        # this works better
        gen_by_mono(args.initial_path, args.start, args.end)
    else:
        # maybe for desperate measures
        gen_by_children(args.initial_path, args.start, args.end)
