# FMP3+TECS/Rust
FMP3+TECS/Rust is a framework that combines [FMP+TECS](https://github.com/azu-lab/FMP3-TECS) (which applies TECS to [TOPPERS/FMP3](https://www.toppers.jp/fmp3-kernel.html) to componentize the kernel) with [TECS/Rust](https://github.com/Nagi70/TECS-Rust), an extension of TECS supporting Rust. Together they provide a component-oriented environment for integrating Rust code with the FMP3 real-time kernel.

#### Sample Programs
There are multiple sample programs; the easiest to understand is `tr_uart_led_sample2`, a simple example combining UART and an LED.

#### Development Environment
1. Vitis & Vivado

   - Download Vitis and Vivado from [here](https://japan.xilinx.com/support/download.html).
   - The sample program runs in a minimal configuration without using the PL (programmable logic) part.

2. Target Board

   - Zybo Z7_10 Zynq-7010 ARM development board
   - Dual-core ARM Cortex-A9 SoC.

3. Toolchains

   - Install Rust to compile the Rust code.
   - Rust installation instructions: [https://rust-lang.org/tools/install/](https://rust-lang.org/tools/install/)
   - Verified with: `rustc 1.89.0-nightly (bf64d66bd 2025-05-21)`
   - Ruby is required for the TECS generator and kernel configurator.
   - Verified with: `ruby 2.7.8p225`. Versions 3.0 and above may not work correctly.

#### Build & Run Steps
All operations below are performed within XSDK.
1. Build

   - Change to the `tr_uart_led_sample2` directory and run `make`.
   - This performs Rust code generation, optimization of mutual exclusion handling, and linking with the FMP3 kernel to produce `fmp.elf`.

2. Run

   - Use Vitis to load `fmp.elf` onto the board to execute the program.

