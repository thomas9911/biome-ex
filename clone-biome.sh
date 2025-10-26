#! /bin/bash

pushd native/biomejs_native
git clone https://github.com/biomejs/biome.git
pushd biome
git reset --hard fe90c785e244b2a17ba8650972fb7eb6ddc6907f
popd
popd
