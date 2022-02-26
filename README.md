## Description
Woff decoder library to convert WOFF file to SFNT. Of course you can use this library only with rust but project also includes 'C/C++' header file with wrapper functions to call code from Rust library. You can find example project here: https://github.com/AlexKhomich/decoder_wrapper_test. Also a rust example, on how to use the library, was added to the project to the 'src/main.rs' file.

## Build android libs
Set android home and NDK home as env variables:
<br> export ANDROID_HOME= \<path to android sdk\> </br>
<br> export NDK_HOME=$ANDROID_HOME/ndk-bundle </br>

Move to the project directory and set path to the NDK bundle under cargo-config.toml file (change <path_to_ndk_bundle> to yours e.g. ~/Android/Sdk/ndk-bundle). This file will be copied to the ~/.cargo directory after running prepare_workspace script. 
 Run prepare_workspace.sh (you can set api version for android SDK - 29 by default and processor architecture). After that just run the build_android.sh script. 
 
 ## Build iOS libs
 You can read about it here: https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html
 Building libs for iOS much easier than building libs for android.
 
 Good luck and have fun :)

