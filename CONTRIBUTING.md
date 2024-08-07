# Contributing guide

## Local development

### Prerequisites

- Install [homebrew](https://brew.sh/) if you haven't already
- Install `brew install just`. just is a command line tool used to run commands defined inside the project `justfile`. You run commands by passing the command name, for example `just build-openapi`.

### Clone the repository

This repository relies on a git submodule to get access to our openapi spec. To clone the repository, use the command:

```
git clone --recurse-submodules git@github.com:pinecone-io/pinecone-rust-client.git
```

Or, if you have already cloned the repository, you can update the submodules with

```
git submodule update --init --recursive
```

### Generating code from OpenAPI and Proto specs

The generated code is already checked into the repository and normally should not need to be modified.

To regenerate OpenAPI or Protobuf code, you will require access to the private `apis` repository.
- Follow setup instructions for the `apis` repository: [apis setup](https://github.com/pinecone-io/apis)

OpenAPI

- You need [Docker Desktop](https://www.docker.com/products/docker-desktop/) in order to generate code using openapi. Look at `codegen/build-oas.sh` to see how this is used.
    - Make sure Docker is running.
- `just build-openapi`
    - References the spec files from the `codegen/apis` submodule
    - Outputs the generated code to `src/openapi`

Protobuf
- `brew install protobuf`
- `cargo install protobuf-codegen` and add it to path: `PATH="$HOME/.cargo/bin:$PATH"`
- `just build-proto`
    - References the spec files from the `codegen/apis` submodule
    - Outputs the generated code to `src/protos`

Alternatively, you can run `just build-client` to regenerate both OpenAPI and Protobuf code.

What the build process looks like in all cases:
- Build the `apis` submodule to produce consolidated .yaml files in `codegen/apis/_build`
- Create a `version.rs` file containing API version info based on the defined value in the justfile
- Run build scripts for OpenAPI and/or Protobuf, propagating the API version

### Build and run

Build and run the project:
- `cargo build` builds the project
- `cargo test` builds the project and runs tests
