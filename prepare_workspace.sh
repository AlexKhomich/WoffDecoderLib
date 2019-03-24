#!/bin/sh

mkdir NDK

${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir NDK/arm64
${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 26 --arch arm --install-dir NDK/arm
${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 26 --arch x86 --install-dir NDK/x86

cp cargo-config.toml ~/.cargo/config

rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
