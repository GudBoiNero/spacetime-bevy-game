This repository is entirely experimental. I have no plans for it besides learning the tools. I'm simply jumping straight in to test and demonstrate the connection between a SpacetimeDB server and a BevyEngine client. With absolutely zero experience in either, including the Rust language. I'm enjoying myself believe it or not.

# Setup

-   Install [SpacetimeDB CLI](https://spacetimedb.com/install), [wasm-opt](https://github.com/WebAssembly/binaryen/releases), [Rust](https://www.rust-lang.org/tools/install), and [CMake](https://cmake.org/download).
-   Clone this repository and open it within VSCode.
-   Run the VSCode task `Generate Server Bindings`.
-   Next open a new terminal and run `spacetime run`.
-   Next run the VSCode task `Build`.

### Operating System

This project was made on a Windows 10 machine.

### Versions

The versions used to develop this project were-

> -   **SpacetimeDB CLI:** [`BETA v0.8`](https://spacetimedb.com/install)
> -   **wasm-opt:** [`version_116`](https://github.com/WebAssembly/binaryen/releases/tag/version_116)
> -   **Rust:** [`1.75.0`](https://www.rust-lang.org/tools/install)
> -   **CMake:** [`3.28.1`](https://cmake.org/download/)
