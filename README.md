# Experiment android & rust (& oculus quest ?)

This repo store my journey/exploration about using Rust with android (maybe to create an Oculus quest app mainly in rust).

## Setup

Install:

- OpenJDK 1.8 from [AdoptOpenJDK - Open source, prebuilt OpenJDK binaries](https://adoptopenjdk.net/)
- Android Studio from [Download Android Studio and SDK tools  |  Android Developers](https://developer.android.com/studio/)
  
  via `Android Studio > Preferences > Appearance & Behaviour > Android SDK > SDK Tools`

  - Android SDK Tools
  - NDK
  
  There are other way to install `android-sdk` and `android-ndk` (download + unzip or brew or ...) this is the most common way.
- rust via [rustup.rs - The Rust toolchain installer](https://rustup.rs/)

  and also install some android target platform:

    ```sh
    rustup target install armv7-linux-androideabi
    rustup target install arm-linux-androideabi
    rustup target install aarch64-linux-android
    rustup target install i686-linux-android
    rustup target install x86_64-linux-android
    ```

## Experimentations

### `exp_0`

I try to create shell script to setup the environment variables (PATH, platform,...) like in articles or some samples I found.
But it was not enough reliable/portable to my taste.

### `exp_1`

A simple Android App that display a "hello world" string computed on rust side, based on the article [Building and Deploying a Rust library on Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html) but

- without shell script to setup environment
- with use of [mozilla/rust-android-gradle](https://github.com/mozilla/rust-android-gradle) to build rust code from gradle, the rust lib/module is named `rust`

The project can be setup, build (and launch) from Android Studio, or from shell `./gradlew test`, ...  like a 100% Java / Kotlin android project project.

*Note: Do not write the jvm code to bind to rust (via jni) into Kotlin because it generate not friendly name for rust function to implement (longer than Java and with number), so stay with Java for this class.*

A cool point is that Android studio know where is `android-sdk`, `android-ndk`,... and share the information with gradle via `local.properties` so it's become more portable project.

### TODO

- allow to run some test on desktop
- create templates for future projects
- explore logger (from ndk) [android_logger](https://crates.io/crates/android_logger)
- explore interaction with java api via [MaulingMonkey/jni-bindgen](https://github.com/MaulingMonkey/jni-bindgen/tree/master/jni-android-sys) or other crates
- explore oculus vr mobile sdk
  - bind and launch in headset
  - display a cude
  - use vulkan
  - use a graphics library like rendy
  
## Links

### Articles

- [Building and Deploying a Rust library on Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html), the basic and could be simplify / automate with tools

### Tools & libs

- [mozilla/rust-android-gradle](https://github.com/mozilla/rust-android-gradle) Cross compile Rust Cargo projects for Android targets.
- [bbqsrc/cargo-ndk: Compile Rust projects against the Android NDK without hassle](https://github.com/bbqsrc/cargo-ndk)
- [cargo-apk](https://crates.io/crates/cargo-apk) Cargo subcommand that allows you to build Android packages (last release 0.4.0 2017-12)
- [snipsco/dinghy: Easier cross-compilation for phones and single boards computers](https://github.com/snipsco/dinghy) - [android-ndk](https://crates.io/crates/android-ndk) Safe Rust bindings to the Android NDK
- [MaulingMonkey/jni-bindgen: Generate Rust JVM FFI wrappers around APIs defined by .jar or .class files, because maintaining your own hand-written bindings is an exercise in boredom, soundness bugs, and pain.](https://github.com/MaulingMonkey/jni-bindgen)
- [ffizer/ffizer: ffizer is a files and folders initializer / generator. Create any kind (or part) of project from template.](https://github.com/ffizer/ffizer/), I'll try to create template for project on android+rust & android+rust+oculus
- [mb64/android-ndk-rs: Rust bindings to the Android NDK](https://github.com/mb64/android-ndk-rs)
- [Nercury/android_logger-rs: A Rust logging implementation for `log` which hooks to android log output](https://github.com/Nercury/android_logger-rs)

### Sample projects

- [makepad/rustquest: This is a minimal example project for the Oculus Quest, written completely in Rust.](https://github.com/makepad/rustquest), made to build on linux, I  tied to adapt it for mac osx but got some strange error, so I decided to make my journey from scratch.
