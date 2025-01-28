#!/bin/bash
cargo build
RUST_LOG=debug sudo -HE ../target/debug/rc_static_data
