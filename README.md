# Cpp vs Rust

This repository aims to demonstrate and compare key differences between C++ and Rust programming languages through various examples highlighting memory management, safety, and concurrency issues. By examining these examples, developers can gain insights into the strengths and weaknesses of each language in handling common programming pitfalls.

## Requirements

- [Visual Studio Code](https://code.visualstudio.com/) \
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Extensions:
    - WSL (When on windows)
    - Dev Containers
- [Docker](https://docs.docker.com/get-docker/)

## How to Build and Try:

The Dev containers feature provide a consistent and reproducible environment including all dependencies when developing within Visual Studio Code (VS Code). In order to build and use a VS Code Dev container execute the following command:

```
Ctrl + Shift + p and then select -> Dev Containers: Rebuild Container
``` 

## How to build project:

When using the Dev Container, make sure that the repo is copied into the Dev Container volume:

```
git clone https://gitlab.com/djalberts/cpp_vs_rust.git
```

Create a build directory in **each cpp** directory of the examples.

```
mkdir build
```

Call CMake to generate the build files:

```
cmake -B build -S .
```

Build the example:

```
cmake --build build
```

## How to run the targets:

This section outlines how to run the different built targets.

Run the cpp application when being in each of the cpp directories:

```
./build/test-project-cpp
```

Run the rust application by calling the following in **each rust** directory:

```
cargo run
```