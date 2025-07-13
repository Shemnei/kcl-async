#!/usr/bin/env bash

set -eou pipefail

# Build app - Properties expect the binary in the `debug` target dir, so no release building without changing the properties !!!
cargo build

# Run app - Assumes it to be installed via `cargo install`
kcl-bootstrap --properties dev.properties --execute
