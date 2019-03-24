#!/bin/sh

cargo clean
CC=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-clang CXX=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-androideabi-clang++ AR=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar cargo build --target aarch64-linux-android --release
CC=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-clang CXX=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-clang++ AR=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar cargo build --target armv7-linux-androideabi --release
CC=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-clang CXX=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-clang++ AR=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar cargo build --target i686-linux-android --release
