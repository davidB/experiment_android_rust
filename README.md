# Experiment android & rust (& oculus quest ?)

This repo store my journey/exploration about using Rust with android (maybe to create an Oculus quest app mainly in rust).

## Setup

Install:

- openjdk 1.8 from [AdoptOpenJDK - Open source, prebuilt OpenJDK binaries](https://adoptopenjdk.net/)
- rust via [rustup.rs - The Rust toolchain installer](https://rustup.rs/)
- Android Studio from [Download Android Studio and SDK tools  |  Android Developers](https://developer.android.com/studio/)
  
  via `Android Studio > Preferences > Appearance & Behaviour > Android SDK > SDK Tools`

  - Android SDK Tools
  - NDK
  
  There are other way to install `android-sdk` and `android-ndk` (download + unzip or brew or ...) this is the most common way.

## Experimentations

### `exp_0`

I try to create shell script to setup the environment variables (PATH, platform,...) like in articles or some samples I found.
But it was not enough reliable/portable to my taste.

## Links

### Articles

- [Building and Deploying a Rust library on Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html), the basic and could be simplify / automate with tools

### Tools & libs

- [mozilla/rust-android-gradle](https://github.com/mozilla/rust-android-gradle) Cross compile Rust Cargo projects for Android targets.
- [cargo-apk](https://crates.io/crates/cargo-apk) Cargo subcommand that allows you to build Android packages (last release 0.4.0 2017-12)
- [android-ndk](https://crates.io/crates/android-ndk) Safe Rust bindings to the Android NDK
- [ffizer/ffizer: ffizer is a files and folders initializer / generator. Create any kind (or part) of project from template.](https://github.com/ffizer/ffizer/), I'll try to create template for project on android+rust & android+rust+oculus

### Sample projects

- [makepad/rustquest: This is a minimal example project for the Oculus Quest, written completely in Rust.](https://github.com/makepad/rustquest), made to build on linux, I  tied to adapt it for mac osx but got some strange error, so I decided to make my journey from scratch.
