#!/usr/bin/env bash

clean() {
  rm -r ./dist
  rm -r ./target
}

build_wasm() {
  wasm-pack build --target web --out-dir dist/pkg --release
}

build_site() {
  tsc
  cp ./web/*.html ./dist
  cp ./web/*.css ./dist
  cp ./web/*.js ./dist
}

if [[ $# -eq 0 ]]; then
  clean
  build_site
  build_wasm
else
  for arg in "$@"; do
    case $arg in
      "-wasm")
        rm -r ./dist/pkg
        rm -r ./target
        build_wasm
        ;;
      "-site")
        rm ./dist/*.html
        rm ./dist/*.css
        rm ./dist/*.js
        build_site
        ;;
      *)
        echo "Invalid argument: $arg"
        ;;
    esac
  done
fi

echo "Done!"
