# AT32-RS-Experiments

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
![Experimental](https://img.shields.io/badge/status-experimental-red.svg)

> **Experimental**: Things may break, change, or burn (your MCU). Use at your own risk.

Rust experiments on **Artery AT32F403A** — a Cortex-M4 MCU

This is a personal playground to learn embedded Rust, explore the AT32 peripherals, and see if something usable grows out of it

## Current goals

- [x] Minimal cortex-m crate building and flashing
- [x] Basic GPIO (blinky)
- [ ] UART communication
- [ ] Timer/PWM

## Requirements

```bash
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-embed
cargo install probe-rs-tools
```
