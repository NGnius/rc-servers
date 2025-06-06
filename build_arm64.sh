#!/bin/bash
# requires cargo cross (cargo install cross)
cross build --release --all --target aarch64-unknown-linux-musl

./utils/package_release.py ./target/aarch64-unknown-linux-musl/release/ --output openjam_servers_linux_arm64.zip
