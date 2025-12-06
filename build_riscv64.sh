#!/bin/bash
# requires cargo cross (cargo install cross)
cross build --release --all --target riscv64gc-unknown-linux-gnu 

./utils/package_release.py ./target/riscv64gc-unknown-linux-gnu/release/ --output rc_servers_linux_riscv64gc.zip
