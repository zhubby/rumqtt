#!/bin/sh
set -e

cargo run --bin mqttbytesparser --release | tee parserbenchmarks.txt