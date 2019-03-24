## Description
Woff decoder library to convert WOFF file to SFNT. Project includes C header with wrapper functions to call code from Rust library. You can find example project here: https://github.com/AlexKhomich/decoder_wrapper_test

## Build android libs
Set android home and NDK home as env variables:
export ANDROID_HOME=<path to android sdk>
export NDK_HOME=$ANDROID_HOME/ndk-bundle

Move to project directory and set path to NDK bundle under cargo-config.toml file (change <path_to_ndk_bundle> to your e.g. ~/Android/Sdk/ndk-bundle). This file will be copied to ~/.cargo after running prepare_workspace script. 
 Run prepare_workspace.sh (you can set api version for android SDK - 26 by default and processor architecture) to create standalone NDKs. After that will be created NDK folder under project directory (this may take some time). 
 After that just run build_android.sh script. 
 
 ## Build iOS libs
 You can read about it here: https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html
 Building libs for iOS much easier than building libs for android.
 
 Good luck and have fun)

