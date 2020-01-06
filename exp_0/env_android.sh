#! /bin/sh
#
# see https://medium.com/visly/rust-on-android-19f34a2fb43
#

function setup_rust_ndk() {
  rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android

  #  mkdir ~/.NDK
  #  local ndk_tools=$NDK/build/tools
  #  ${ndk_tools}/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir ~/.NDK/arm64
  #  ${ndk_tools}/make_standalone_toolchain.py --api 26 --arch arm --install-dir ~/.NDK/arm
  #  ${ndk_tools}/make_standalone_toolchain.py --api 26 --arch x86 --install-dir ~/.NDK/x86

  #WARNING:__main__:make_standalone_toolchain.py is no longer necessary. The
  #$NDK/toolchains/llvm/prebuilt/darwin-x86_64/bin directory contains target-specific scripts that perform
  #the same task. For example, instead of:
  #
  #    $ python $NDK/build/tools/make_standalone_toolchain.py \
          #        --arch x86 --api 26 --install-dir toolchain

  #    $ toolchain/bin/clang++ src.cpp
  #
  #Instead use:
  #
  #    $ $NDK/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android26-clang++ src.cpp

  # from https://crates.io/crates/cargo-cala
  #```
  #rustup target add arm-linux-androideabi aarch64-linux-android armv7-linux-androideabi i686-linux-android thumbv7neon-linux-androideabi x86_64-linux-android
  #mkdir ~/.cargo-dist/
  #cd ~/.cargo-dist/
  #wget https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip
  #unzip sdk-tools-linux-4333796.zip
  #wget https://dl.google.com/android/repository/android-ndk-r19c-linux-x86_64.zip
  #unzip android-ndk-r19c-linux-x86_64.zip
  #rm android-ndk-r19c-linux-x86_64.zip
  #rm sdk-tools-linux-4333796.zip
  #mv android-ndk-r19c/ android_ndk/
  #./android_ndk/build/tools/make-standalone-toolchain.sh --toolchain=arm-linux-androideabi --install-dir=arm-linux-androideabi
  #./android_ndk/build/tools/make-standalone-toolchain.sh --toolchain=aarch64-linux-android --install-dir=aarch64-linux-android
  #./android_ndk/build/tools/make-standalone-toolchain.sh --toolchain=x86-linux-android --install-dir=x86-linux-android
  #./android_ndk/build/tools/make-standalone-toolchain.sh --toolchain=x86_64-linux-android --install-dir=x86_64-linux-android
  #mkdir android_sdk/
  #cd android_sdk/
  #mv ../tools .
  #./tools/bin/sdkmanager "platform-tools" "platforms;android-18" "build-tools;26.0.1"
  #```

  mkdir -p .cargo
  cat >.cargo/config <<EOF
[target.aarch64-linux-android]
ar = "$ANDROID_NDK_TOOLCHAIN/bin/aarch64-linux-android-ar"
linker = "$ANDROID_NDK_TOOLCHAIN/bin/aarch64-linux-android26-clang"

[target.armv7-linux-androideabi]
ar = "$ANDROID_NDK_TOOLCHAIN/bin/arm-linux-androideabi-ar"
linker = "$ANDROID_NDK_TOOLCHAIN/bin/armv7a-linux-androideabi26-clang"

[target.i686-linux-android]
ar = "$ANDROID_NDK_TOOLCHAIN/bin/i686-linux-android-ar"
linker = "$ANDROID_NDK_TOOLCHAIN/bin/i686-linux-android26-clang"
EOF

}

. env_j8.sh

# install android standalone toolchain
#brew install intel-haxm
#brew install android-sdk
#brew install android-ndk

# update env vars
#export ANDROID_HOME=/usr/local/share/android-sdk
#export NDK_HOME=/usr/local/share/android-ndk

# install by Android Studio
export ANDROID_HOME=$HOME/Library/Android/sdk

# If the NDK libraries are installed by a previous version of Android Studio, do
#export ANDROID_NDK_HOME=$ANDROID_HOME/ndk-bundle
# If the NDK libraries are installed by Android Studio 3.5, do
#export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/20.0.5594570
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/20.1.5948944
export NDK_HOME=$ANDROID_NDK_HOME

#ANDROID_BUILD_TOOLS_VERSION=28.0.3
ANDROID_BUILD_TOOLS_VERSION=29.0.2
ANDROID_NDK_TOOLCHAIN=/toolchains/llvm/prebuilt/darwin-x86_64

TOOLS=$ANDROID_HOME/platform-tools:$TOOLS
TOOLS=$ANDROID_HOME/tools:$TOOLS
TOOLS=$ANDROID_NDK_HOME/toolchains/llvm/prebuild/*/bin:$TOOLS
TOOLS=$ANDROID_HOME/build-tools/$ANDROID_BUILD_TOOLS_VERSION:$TOOLS
TOOLS=$ANDROID_NDK_TOOLCHAIN/bin:$PATH

export PATH=$TOOLS:$PATH

setup_rust_ndk

# setup Oculus

export OVR_HOME=$HOME/bin/soft/ovr_sdk_mobile_1.29.0
