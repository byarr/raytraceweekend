#!/usr/bin/env bash

set -ex

cargo build --release
./target/release/two_spheres > output/two_spheres.png
./target/release/two_spheres 100 > output/two_spheres_anti.png