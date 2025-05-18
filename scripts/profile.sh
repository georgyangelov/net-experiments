#!/bin/bash -e

cargo build --profile profiling

samply record -r 1000000 ./target/profiling/net-experiments