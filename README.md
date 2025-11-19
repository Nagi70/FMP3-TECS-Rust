# FMP3+TECS/Rust
FMP3+TECS/Rust is a framework that combines [FMP+TECS](https://github.com/azu-lab/FMP3-TECS) — a componentized framework applying TECS to [TOPPERS/FMP3](https://www.toppers.jp/fmp3-kernel.html) — with [TECS/Rust](https://github.com/Nagi70/TECS-Rust), which adapts TECS for Rust.

### Sample Programs
The `evaluation` directory contains the samples used in the paper. Instructions for running each sample are provided in each respective directory.

### Development Environment
1. Vitis and Vivado

   - You can download Vitis and Vivado from [here](https://japan.xilinx.com/support/download.html).

      - The sample programs can run with a minimal configuration that does not use the PL part.

      - The project used was the contents extracted from `fmp3_workspace.zip`

2. Target Board

   - Zybo Z7_10 Zynq-7010 ARM development board
   
      - A dual-core development board equipped with an ARM Cortex-A9.

3. Toolchains

   - Rust
      - Rust is required to compile the code.

      - Install Rust from [here](https://rust-lang.org/tools/install/).

      - Verified to work with `rustc 1.89.0-nightly (bf64d66bd 2025-05-21)`.

   - bindgen
      - Installing bindgen is required for builds.

      - Install bindgen from [here](https://rust-lang.github.io/rust-bindgen/command-line-usage.html)

      - Verified to work with `bindgen-cli v0.69.0`.

   - Ruby

      - Ruby is required to use the TECS generator and the kernel configurator.

      - Verified to work with `ruby 2.7.8p225`; versions 3.0 or later may not work properly.


### Build and Run
All of the following operations are performed on XSDK.
1. Build

   - Move to a directory under the `evaluation` directory that contains a `Makefile`.
   
   - To enable optimization of mutual exclusion, set `ENABLE_TECS_RUST_OPT` to `1` in the `Makefile`.
   
   - Run the `make` command.
   
   - This builds the Rust code and links it with the FMP3 kernel to produce `fmp.elf`.

2. Run

   - Unzip `fmp3_workspace.zip`.

   - Open the extracted `fmp3_workspace` in Vitis.

   - In Vitis, write `fmp.elf` to core0 using `launch.json` to run it.


