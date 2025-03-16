#!/bin/bash

RUST_BACKTRACE=1 RUST_LOG=debug cargo run -- -1 #&> ../data/rc_services_room.log
