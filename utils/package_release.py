#!/bin/env python3

# it is assumed this is run from the root of the project

import argparse
import zipfile
import os

def generate_ip_script(binary_name: str) -> str:
    return f"""#!/bin/bash
export RUST_LOG=info

exec ./{binary_name} --ip "0.0.0.0"
"""

def generate_plain_script(binary_name: str) -> str:
    return f"""#!/bin/bash
export RUST_LOG=info

exec ./{binary_name}
"""

def generate_ip_redirect_script(binary_name: str, port: int) -> str:
    return f"""#!/bin/bash
export RUST_LOG=info

exec ./{binary_name} --ip "0.0.0.0" --redirect "domain_or_ip_address:{port}"
"""

BINARIES_TO_INCLUDE = {
    #"oj_auth": generate_plain_script,
    "oj_rc_auth": generate_ip_script,
    "oj_cdn": generate_ip_script,
    "oj_rc_chat": lambda name: generate_ip_redirect_script(name, 4537), "oj_rc_chat_room": generate_ip_script,
    "oj_rc_microtransactions": generate_plain_script,
    "oj_rc_services": lambda name: generate_ip_redirect_script(name, 4533), "oj_rc_services_room": generate_ip_script,
    "oj_rc_singleplayer": lambda name: generate_ip_redirect_script(name, 4539), "oj_rc_singleplayer_room": generate_ip_script,
    "oj_rc_social": lambda name: generate_ip_redirect_script(name, 4535), "oj_rc_social_room": generate_ip_script,
    "oj_rc_lobby": lambda name: generate_ip_redirect_script(name, 4541), "oj_rc_lobby_room": lambda name: generate_ip_redirect_script(name, 4542),
    "oj_rc_multiplayer": generate_ip_script,
}

PROJECT_FOLDERS_TO_INCLUDE = [
    "assets",
]

# extra assets; not ones in one of the listed folders
PROJECT_ASSETS_TO_INCLUDE = [
    "utils/cube_gen.py",
    "LICENSE",
    "README.md",
]

BLANK_FOLDERS_TO_CREATE = [
    "data/robocraft",
    "data/robocraft/brawldata",
    "data/robocraft/campaigndata",
    "data/robocraft/clanavatar",
    "data/robocraft/customavatars",
    "data/robocraft/factorythumbnails",
]

def add_folder_to_zip(archive: zipfile.ZipFile, root_dir: str):
    archive.mkdir(root_dir)
    for (root, dirs, files) in os.walk(root_dir):
        for f in files:
            archive.write(os.path.join(root, f))
        for d in dirs:
            archive.mkdir(os.path.join(root, d))

def main(build_root: str, outfile: str):
    print("Packaging binaries in", build_root)
    archive = zipfile.ZipFile(outfile, mode="w", compression=zipfile.ZIP_LZMA, compresslevel=9)
    # handle release binaries
    print("Adding", len(BINARIES_TO_INCLUDE), "binaries")
    for binary_file in BINARIES_TO_INCLUDE.keys():
        fp = os.path.join(build_root, binary_file)
        folder_name = binary_file.replace("oj_", "")
        archive.write(fp, arcname=os.path.join(folder_name, binary_file))
        script_generator = BINARIES_TO_INCLUDE[binary_file]
        if script_generator is not None:
            script = BINARIES_TO_INCLUDE[binary_file](binary_file)
            info = zipfile.ZipInfo(os.path.join(folder_name, "run.sh"))
            info.external_attr = 0o0755 << 16 # unix file permissions
            archive.writestr(info, script)
    # handle release assets
    print("Adding", len(PROJECT_FOLDERS_TO_INCLUDE), "project folders")
    for folder in PROJECT_FOLDERS_TO_INCLUDE:
        add_folder_to_zip(archive, folder)
    print("Adding", len(PROJECT_ASSETS_TO_INCLUDE), "extra assets")
    for asset in PROJECT_ASSETS_TO_INCLUDE:
        archive.write(asset)
    # handle project setup niceties
    print("Creating", len(BLANK_FOLDERS_TO_CREATE), "blank folders")
    for folder in BLANK_FOLDERS_TO_CREATE:
        archive.mkdir(folder)
    archive.close()


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("build_dir")
    parser.add_argument("--output", default="package.zip")
    args = parser.parse_args()
    main(args.build_dir, args.output)
