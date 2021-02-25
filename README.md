# Filecoin scheduling 
![stability-wip](https://img.shields.io/badge/stability-work_in_progress-lightgrey.svg)                                                                        
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

![Build status Actions Status](https://github.com/Zondax/filecoin-scheduling/workflows/rust/badge.svg)

**This project is still work in progress !**

## How to build this project
### Linux
This project is written in Rust, for a complete guide on how to install Rust on your computer  follow the instructions [here](https://www.rust-lang.org/tools/install). Other dependencies that must to be installed are:

- CBC: _"The COIN  Branch and Cut solver"_ [1 ](https://www.coin-or.org/Cbc/cbcuserguide.html#ftn.id3342326)] 
- OpenCL

You can install these dependencies on Ubuntu by running the following commands on your terminal: 

```shell
sudo apt-get update
sudo apt-get install ocl-icd-opencl-dev
sudo apt-get install coinor-libcbc-dev
```

The next step is cloning this repository

```shell
git clone https://github.com/Zondax/filecoin-scheduling.git
```

go to the repository and run:

```shell
cargo build --release #for building the project in release mode
cargo test # For executing all the unit tests and integration tests
```

### MacOS

Although the project supports _macOS_, there are not  instruction yet. Feel free to contribute on this regard.

# 