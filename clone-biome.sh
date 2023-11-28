#! /bin/bash

pushd native/biomejs_native
git clone https://github.com/biomejs/biome.git
pushd biome
git reset --hard 7bac8fda8f
popd
popd
