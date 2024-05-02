#!/usr/bin/env bash

clean() {
  rm -r ./dist
  rm -r ./wasm/pkg
  rm -r ./wasm/target
  rm -r ./site/target
}

build_wasm() {
  cd wasm
  wasm-pack build --target web
  cd ..
}

build_site() {
  cd site/src
  tsc
  cp -r *.html ../target/
  cp -r *.css ../target/
  cd ../..
}

deploy() {
  cp -rf ./site/target/ ./dist/
  cp -rf ./wasm/pkg/ ./dist/pkg/
}

if [[ $# -eq 0 ]]; then
  clean
  build_wasm
  build_site
  deploy
else
  for arg in "$@"; do
    case $arg in
      "-wasm")
        rm -r ./wasm/pkg
        rm -r ./wasm/target
        build_wasm
        ;;
      "-site")
        rm -r ./site/target
        build_site
        ;;
      *)
        echo "Invalid argument: $arg"
        ;;
    esac
    deploy
  done
fi

echo "Done!"
