#!/usr/bin/env bash


set -ex

cd programs;
scripts/deploy.sh
cd -
node js/main.js 0
node js/main.js 1
