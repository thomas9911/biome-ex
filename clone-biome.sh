#! /bin/bash

pushd native/biomejs_native
git clone https://github.com/biomejs/biome.git
pushd biome
git reset --hard c372484473b64c9c84ad3361d71b2c419345b45b
popd
popd
