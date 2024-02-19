#!/bin/bash

set -eu

SCRIPTPATH=$( cd "$(dirname "$(readlink -f "$0")")" || exit 1 ; pwd -P )
cd "$SCRIPTPATH"

npm install

rm -rf pkg/ dist/


npx wasm-pack build "../" --target web --out-name web --out-dir web/pkg --release
npm run build

rm -rf hosting_repo
git clone git@github.com:Finomnis/effects-1d-demo.git hosting_repo -b gh-pages
(
    cd hosting_repo
    rm *
    cp ../dist/* .
    git add .
    git commit -m 'Update content'
    git push
)
rm -rf hosting_repo
