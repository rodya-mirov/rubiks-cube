#!/bin/zsh

echo "Building in release mode ..."

cargo build --release -q

echo "Running samples ..."

OUTFILE=benchmark_results.txt

./target/release/rubiks-cube > $OUTFILE

echo "All done! Results in ${OUTFILE}"
