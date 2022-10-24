#!/usr/bin/env bash

set -ex

program=$(solana address -k target/deploy/rewards-keypair.json)
solana logs $program
