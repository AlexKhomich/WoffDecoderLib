#!/bin/sh

cp cargo-config.toml ~/.cargo/config

rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
