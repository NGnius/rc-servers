#!/bin/bash

cargo clippy --all --fix -- \
    -A clippy::collapsible_if \
    -A clippy::type_complexity \
    -A clippy::wrong_self_convention
