# replication

replication is a simple solution for extracting tabular data from PDFs and
converting it to JSON using natural language processing. 

The project consists of two primary components:

- `replication`: A [gRPC](https://grpc.io/) messaging service and [Replicate](https://replicate.com/)
  client implemented in Python.
- `replication-ctl`: A Rust client designed for loading data into `replication` over gRPC.

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
- Create a `.env` file in the project root, the same directory as this README:

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

This command starts the Docker container, and your replication app's gRPC service
will be accessible at [http://localhost:50051](http://localhost:50051).

If things work, you should see this in your terminal:

```terminal
Starting 'replication' server. Listening on port 50051.
```

#### Clean up:

```terminal
make clean
```

Use this command to stop and remove running containers, and remove the
Docker image.

### Manual Docker Commands (without Makefile)

If you prefer not to use the Makefile, you can use the following manual
commands:

#### Build the Docker image:

```terminal
docker build -t replication-image .
```

#### Run the Docker container:

```terminal
docker run -it -p 50051:50051 --env-file .env replication-image
```

#### Clean up:

```terminal
docker stop $$(docker ps -aq --filter ancestor=replication-image) || true
docker rm $$(docker ps -aq --filter ancestor=replication-image) || true
docker rmi replication-image || true
```

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
cargo run
```

This command compiles and runs the `replication-ctl` Rust client, sending a
request to the `replication` app's gRPC service.

By default, `replication-ctl` listens for the `replication` gRPC service on port
`50051`, and uses data from the `table.pdf` test file.

Using the default `table.pdf` file, you should output like this:

```terminal
Received from 'replication' via OpenAI:

{
  "table": [
    {
      "Ballots Completed": 1,
      "Ballots Incomplete/Terminated": 4,
      "Category": "Completed",
      "Disability": "Blind",
      "Participants": 5,
      "Results Accuracy": "34.5%, n=1",
      "Time to complete": "1199 sec, n=1"
    },
    {
      "Ballots Completed": 2,
      "Ballots Incomplete/Terminated": 3,
      "Category": "Completed",
      "Disability": "Low Vision",
      "Participants": 5,
      "Results Accuracy": "98.3% n=2 (97.7%, n=3)",
      "Time to complete": "1716 sec, n=3 (1934 sec, n=2)"
    },
    {
      "Ballots Completed": 4,
      "Ballots Incomplete/Terminated": 1,
      "Category": "Completed",
      "Disability": "Dexterity",
      "Participants": 5,
      "Results Accuracy": "98.3%, n=4",
      "Time to complete": "1672.1 sec, n=4"
    },
    {
      "Ballots Completed": 3,
      "Ballots Incomplete/Terminated": 0,
      "Category": "Completed",
      "Disability": "Mobility",
      "Participants": 3,
      "Results Accuracy": "95.4%, n=3",
      "Time to complete": "1416 sec, n=3"
    }
  ]
}
```
