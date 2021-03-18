# Filecoin scheduling 
![stability-wip](https://img.shields.io/badge/stability-work_in_progress-lightgrey.svg)                                                                        
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)



[![Actions Status](https://github.com/Zondax/filecoin-scheduling/workflows/Rust/badge.svg)](https://github.com/Zondax/filecoin-scheduling/actions)



**This project is still work in progress !**

## Prerequisites

This project is written in Rust, for a complete guide on how to install Rust on your computer  follow the instructions [here](https://www.rust-lang.org/tools/install).

### Prerequisites - Linux

You can install these dependencies on Ubuntu by running the following commands on your terminal:

```shell
sudo apt-get update
sudo apt-get install ocl-icd-opencl-dev
```

### Prerequisites - MacOS

- None 

## Building

Clone this repository

```shell
git clone https://github.com/Zondax/filecoin-scheduling.git
cd filecoin-scheduling
```

Then compile:

```shell
cargo build --release #for building the project in release mode
cargo test # For executing all the unit tests and integration tests
```
