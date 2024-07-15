# Pinecone Rust SDK

- [Asana backlog](https://app.asana.com/0/1207449888227387/1207449824366220)

# Prerequisites

- You need [Docker Desktop](https://www.docker.com/products/docker-desktop/) in order to generate code using openapi. Look at `codegen/build-oas.sh` to see how this is used.
- Install [homebrew](https://brew.sh/) if you haven't already
- Install `brew install just`. just is a command line tool used to run commands defined inside the project `justfile`. You run commands by passing the command name, for example `just build-openapi`.

Dependencies for generating code from OpenAPI and protobuf specifications:
- Follow setup instructions for the `apis` repository
- `brew install protobuf`
- `cargo install protobuf-codegen` and add it to path: `PATH="$HOME/.cargo/bin:$PATH"`

# Clone the repository

This repository relies on a git submodule to get access to our openapi spec. To clone the repository, use the command:

```
git clone --recurse-submodules git@github.com:pinecone-io/pinecone-rust-client.git
```

Or, if you have already cloned the repository, you can update the submodules with

```
git submodule update --init --recursive
```

# Build and run

OpenAPI
- The generated code is comitted to the repository, but to regenerate run `just build-openapi`
- References the spec files from the `codegen/apis` submodule
- Outputs the generated code to `openapi`

Protobuf
- Code is generated from protobuf during the project build process (`cargo build`) using the build script `build.rs`
- The generated code is outputted to `/target/debug/build/pinecone_sdk-{hash}/out`

Build and run the project:
- `cargo build` builds the project
- `cargo test` builds the project and runs tests
