#!/bin/sh

COMPILER_TOOL_PATH=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin

cargo clean
CC=$COMPILER_TOOL_PATH/aarch64-linux-android29-clang CXX=$COMPILER_TOOL_PATH/aarch64-linux-android29-clang++ AR=$COMPILER_TOOL_PATH/aarch64-linux-android-ar cargo build --target aarch64-linux-android --release && "$COMPILER_TOOL_PATH"/aarch64-linux-android-strip target/aarch64-linux-android/release/libwoffdecoder.so
CC=$COMPILER_TOOL_PATH/armv7a-linux-androideabi29-clang CXX=$COMPILER_TOOL_PATH/armv7a-linux-androideabi29-clang++ AR=$COMPILER_TOOL_PATH/arm-linux-androideabi-ar cargo build --target armv7-linux-androideabi --release && "$COMPILER_TOOL_PATH"/armv7a-linux-androideabi-strip target/armv7-linux-androideabi/release/libwoffdecoder.so
CC=$COMPILER_TOOL_PATH/i686-linux-android29-clang CXX=$COMPILER_TOOL_PATH/i686-linux-android29-clang++ AR=$COMPILER_TOOL_PATH/i686-linux-android-ar cargo build --target i686-linux-android --release && "$COMPILER_TOOL_PATH"/i686-linux-android-strip target/i686-linux-android/release/libwoffdecoder.so
CC=$COMPILER_TOOL_PATH/x86_64-linux-android29-clang CXX=$COMPILER_TOOL_PATH/x86_64-linux-android29-clang++ AR=$COMPILER_TOOL_PATH/x86_64-linux-android-ar cargo build --target x86_64-linux-android --release && "$COMPILER_TOOL_PATH"/x86_64-linux-android-strip target/x86_64-linux-android/release/libwoffdecoder.so
