#!/usr/bin/env bash

rm -r ./dist
rm -r ./wasm/pkg
rm -r ./wasm/target
rm -r ./site/target

cd wasm
wasm-pack build --target web
cd ..

cd site/src
tsc
cp -r *.html ../target/
cp -r *.css ../target/
cd ../..

cp -r ./site/target/ ./dist/
cp -r ./wasm/pkg/ ./dist/pkg/

echo "Done!"
