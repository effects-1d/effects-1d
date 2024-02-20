#!/bin/bash

set -eu

SCRIPTPATH=$( cd "$(dirname "$(readlink -f "$0")")" || exit 1 ; pwd -P )
cd "$SCRIPTPATH"

npm install

rm -rf pkg/ dist/


npx wasm-pack build "../" --target web --out-name web --out-dir web/pkg --release
npm run build
