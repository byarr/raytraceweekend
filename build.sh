#!/usr/bin/env bash

cargo build --release
./target/release/raytraceweekend > output/sphere.png