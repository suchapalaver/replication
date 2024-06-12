# replication

replication is a simple app for generating images from text using natural language processing.

## Purpose

The goal of this project is to explore the deployment of multiple AI agents capable of interacting
with each other, with a design suitable for container clusters. Currently, replication supports
two models: one that generates an image from a text prompt, and another that reads an image via URL
to provide text feedback. The aim is to investigate image generation filtered by feedback from one
language model, which comments on and critiques the output of another.

## Design

The project consists of two primary components:

- `replication`: A [gRPC](https://grpc.io/) messaging service and [Replicate](https://replicate.com/)
  client implemented in Python.
- `replication-ctl`: A Rust client designed for loading Intents into `replication` over gRPC.

![`replication` design diagram](design.svg)

Equipped with built-in Docker support and a Makefile for streamlined development
and deployment, `replication` serves as a proof of concept for deploying AI agents on
orchestration infrastructure, such as Kubernetes.

## Installation

### Clone the Repository:

```terminal
git clone https://github.com/suchapalaver/replication.git
cd replication
```

### Set Up Replicate API Key:

- Obtain an API key from [Replicate](https://replicate.com/).
- Create a `.env` file in the `replication/` directory:

```env
REPLICATE_API_TOKEN=<paste-your-token-here>
```

Find your API token in [your account settings](https://replicate.com/account/api-tokens).

## Getting Started

Follow these instructions to get a copy of the project up and running on your
local machine for development and testing purposes.

### Prerequisites

- Docker
- Make (optional, but recommended for using the Makefile)
- [Rust and Cargo](https://www.rust-lang.org/tools/install) (for running the `replication-ctl` Rust client)

### Building and Running the Docker Container (`replication` App)

#### Build the Docker image:

```terminal
make build
```

This command builds the Docker image for the replication app.

#### Run the Docker container:

```terminal
make run
```

This command starts the Docker container.

By default, `replication-ctl` listens for the `replication` gRPC service on port
`50051`. Your replication app's gRPC service should be accessible at [http://localhost:50051](http://localhost:50051).

If things work, and you're using default settings, you should see this in your terminal:

```terminal
Starting 'replication' server. Listening on port 50051.
```

#### Clean up:

```terminal
make clean
```

Use this command to stop and remove running containers, and remove the
Docker image.

## Running the Rust Client (`replication-ctl`)

### Navigate to replication-ctl directory:

```bash
cd replication-ctl
```

### See the CLI Help Menu

```bash
cargo run -- --help
```

### Build and Run the Rust Client:

```bash
cargo run -- --models sdxl.yaml llava.yaml
```

This command compiles and runs the `replication-ctl` Rust client, sending a
request to the `replication` app's gRPC service.
