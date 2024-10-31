#!/bin/bash -e
# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# verify that git repo is clean
if [[ -n $(git status -s) ]]; then
  echo "Working directory is not clean. Please commit all changes before running this script."
  echo $(git status -s)
  exit 1
fi

# apply git patch
git apply ./scripts/simtest/config-patch

root_dir=$(git rev-parse --show-toplevel)
export SIMTEST_STATIC_INIT_MOVE=$root_dir"/examples/move/basics"

cargo llvm-cov clean

MSIM_WATCHDOG_TIMEOUT_MS=60000 MSIM_TEST_SEED=1 cargo +nightly llvm-cov --ignore-run-fail --branch --html nextest \
  --cargo-profile simulator \
  --workspace \
  --exclude iota-e2e-tests \
  --exclude iota-json-rpc-tests \
  --exclude iota-faucet

# remove the patch
git checkout .cargo/config Cargo.toml Cargo.lock
