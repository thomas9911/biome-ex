#! /bin/bash

# small fix in rustler deps to resolve biome workspaces
sed -i 's/current_spec\["dependencies"\]/(current_spec\["dependencies"\] || \[\])/g' deps/rustler/lib/rustler/compiler/config.ex
