#!/bin/bash
# requires cargo cross (cargo install cross)
cross build --release --all --target x86_64-unknown-linux-musl

./utils/package_release.py ./target/x86_64-unknown-linux-musl/release/ --output openjam_servers_linux_x86_64.zip
