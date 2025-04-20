# gRPC Examples in Rust

This repository contains examples of using gRPC with Rust.  Or at least it might... Initially it starts off with trying out `clap` for command-line parsing.

## Prerequisites

-   Rust and Cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
-   Protocol Buffers compiler (`protoc`): [https://grpc.io/docs/protoc-installation/](https://grpc.io/docs/protoc-installation/)

## Examples

### Greeter

A simple "Hello, World!" example using gRPC.

-   **`greeter/proto/greeter.proto`**: Defines the gRPC service and messages.
-   **`greeter/src/client.rs`**: The gRPC client implementation.
-   **`greeter/src/server.rs`**: The gRPC server implementation.

### Running the Greeter Example

1.  Navigate to the `greeter` directory:

    ```bash
    cd greeter
    ```

2.  Run the server:

    ```bash
    cargo run --bin server
    ```

3.  In a new terminal, run the client:

    ```bash
    cargo run --bin client
    ```

### Building the project

1. Navigate to the root directory:
    ```bash
    cd ..
    ```
2. Build the project
    ```bash
    cargo build
    ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
