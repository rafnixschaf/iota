# IOTA Private Network

## Requirements

- [Docker Compose](https://docs.docker.com/engine/install/)

## Steps

### 1. Build Docker Images

Run the following commands to build the necessary Docker images:

#### iota-node

```bash
../iota-node/build.sh -t iota-node --no-cache
```

#### iota-indexer

```bash
../iota-indexer/build.sh -t iota-indexer --no-cache
```

#### iota-tools

```bash
../iota-tools/build.sh -t iota-tools --no-cache
```

#### GraphQL

```bash
../iota-graphql-rpc/build.sh -t iota-graphql-rpc --no-cache
```

### 2. Bootstrap the Network

Generate the keystores, configuration files and genesis files:

```bash
./bootstrap.sh
```

### 3. Start the Network

This will bring up 4 validators, 2 fullnodes, 1 indexer writer, 1 indexer reader and faucet.

```bash
docker compose up
```

### Endpoints

- fullnode-0: http://127.0.0.1:9000

- fullnode-2: http://127.0.0.1:9001

- Indexer: http://127.0.0.1:9002

- Faucet: http://127.0.0.1:5003

- GraphQL: http://127.0.0.1:8000

- Dashboard: http://127.0.0.1:3000

