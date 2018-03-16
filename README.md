# Prerequisites
* 64-bit CPU
* You need Xargo, the cross-compiling build tool wrapper around Cargo. Get it with `cargo install xargo`.
# Building
You must be on the nightly toolchain for this to build. If you are not on the nightly toolchain, please set it (just for this project) with `rustup override set nightly`.

In order to create a bootimage, we use the already made bootimage package that automatically downloads a bootloader and links it with our kernel.
Download it with `cargo install bootimage`.
## Windows
```
$ set RUST_TARGET_PATH=%cd%
$ bootimage --target x86_64-atomic
```
## Linux

[//]: # (cargo rustc -- -Z pre-link-arg=-nostartfiles (may need again?))

```
$ RUST_TARGET_PATH=$(pwd) bootimage --target x86_64-atomic
```
## MacOS

[//]: # (cargo rustc -- -Z pre-link-arg=-lSystem (may need again?))

```
$ RUST_TARGET_PATH=$(pwd) bootimage --target x86_64-atomic
```
# Running
Running the kernel is easiest with Qemu, after building with `bootimage`.
```
qemu-system-x86_64 -drive format=raw,file=bootimage.bin
```
