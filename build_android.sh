#!/bin/sh

cargo clean
CC=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang CXX=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-androideabi26-clang++ AR=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar cargo build --target aarch64-linux-android --release && strip target/aarch64-linux-android/release/libwoffdecoder.so
CC=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang CXX=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang++ AR=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar cargo build --target armv7-linux-androideabi --release && strip target/armv7-linux-androideabi/release/libwoffdecoder.so
CC=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang CXX=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang++ AR=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar cargo build --target i686-linux-android --release && strip target/i686-linux-android/release/libwoffdecoder.so
