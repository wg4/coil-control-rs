# coil-control-rs

Firmware for an STM32F3 to control coils to accelarate a steel ball for the
ViralBallTrack.

See https://github.com/wg4/viralballtrack and
https://github.com/Makers-Im-Zigerschlitz/ViralBallTrack for more information
about ViralballTrack.

## Building

 1. Install Rust: https://rustup.rs
 2. Install the ARM toolchain:
     ```
     rustup target add thumbv7em-none-eabihf
     ```
 3. Run `cargo build`

## Running

 1. Install debugger and openocd
    ```bash
    # ArchLinux
    sudo pacman -S arm-none-eabi-gdb openocd
    ```
 2. Run `cargo run`
