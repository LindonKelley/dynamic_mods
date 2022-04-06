#!/bin/bash

echo "compiling loader"
cargo build --manifest-path loader/Cargo.toml || exit
rm -r "output" 2>/dev/null
mkdir -p "output/mods"
mv loader/target/debug/loader output/ 2>/dev/null

echo "compiling mods"
for mod in "$@"; do
  [ -d "mods/$mod" ] || { echo "mods/$mod does not exist or is not a directory"; exit; }
  if [ -f "mods/$mod/Cargo.toml" ]; then
    echo "compiling $mod via Rust"
    cargo build --manifest-path mods/"$mod"/Cargo.toml || exit
    mv mods/"$mod"/target/debug/lib"$mod".so output/mods/"$mod".so 2>/dev/null
  else
    echo "mods/$mod/Cargo.toml not found, unable to compile $mod"
    exit
  fi
done

exec ./output/loader
