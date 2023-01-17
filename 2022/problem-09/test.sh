#!/bin/bash

# Simple script for testing a rust program's time
echo Building program...
cargo build > /dev/null 2>&1
echo Running program...
START="$(date +%s)"
cargo run > /dev/null 2>&1
DURATION=$[ $(date +%s) - ${START}]
echo Program took: ${DURATION} seconds