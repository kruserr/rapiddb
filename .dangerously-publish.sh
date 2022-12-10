#!/usr/bin/env bash

git push --follow-tags

cd rapiddb
cargo publish
