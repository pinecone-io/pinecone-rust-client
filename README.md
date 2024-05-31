# Pinecone Rust SDK

- [Asana backlog](https://app.asana.com/0/1207449888227387/1207449824366220)

# Prerequisites

- You need [Docker Desktop](https://www.docker.com/products/docker-desktop/) in order to generate code using openapi. Look at `codegen/build-oas.sh` to see how this is used.
- Install [homebrew](https://brew.sh/) if you haven't already
- Instasll `brew install just`. just is a command line tool used to run commands defined inside the project `justfile`. You run commands by passing the command name, for example `just build-openapi`.

# Clone the repository

This repository relies on a git submodule to get access to our openapi spec. To clone the repository, use the command:

```
git clone --recurse-submodules git@github.com:pinecone-io/pinecone-rust-client.git
```

Or, if you have already cloned the repository, you can update the submodules with

```
git submodule update --init --recursive
```