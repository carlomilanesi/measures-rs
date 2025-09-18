#!/bin/sh

# Ensure the tool format-measure-n-d is built
cargo b --release

# Find all .rs files in src/inner excluding mod.rs files
FILELIST=`find src/inner -name '*.rs' | grep -v "/mod.rs$"`

# First, comment out the first three lines and the last two lines.
for FILEPATH in $FILELIST
do
    target/release/format-measure-n-d comment $FILEPATH
done

# Format the code with cargo fmt
cargo fmt

# Then, uncomment the first three lines and the last two lines.
for FILEPATH in $FILELIST
do
    target/release/format-measure-n-d uncomment $FILEPATH
done

# Finally, check that the resulting code is properly formatted
cargo fmt --check
