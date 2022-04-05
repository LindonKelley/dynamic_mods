#!/bin/bash

echo "building loader"
cargo build --manifest-path loader/Cargo.toml || exit
echo "building mod a"
cargo build --manifest-path mod_a/Cargo.toml || exit
echo "building mod b"
cargo build --manifest-path mod_b/Cargo.toml || exit
echo "building mod c"
cargo build --manifest-path mod_c/Cargo.toml || exit

mkdir -p "output/mods"
(
mv loader/target/debug/loader output/
mv mod_a/target/debug/libmod_a.so output/mods/mod_a.so
mv mod_b/target/debug/libmod_b.so output/mods/mod_b.so
mv mod_c/target/debug/libmod_c.so output/mods/mod_c.so
) 2>/dev/null
exec ./output/loader
