#!/bin/bash

RUST_BACKTRACE=1 RUST_LOG=debug cargo run #&> ../data/rc_services_room.log
