#!/usr/bin/env bash

set -ex

cargo-build-sbf
solana program deploy target/deploy/rewards.so
