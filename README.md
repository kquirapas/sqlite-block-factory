# sqlite block factory

![Block Factory Architecture Diagram](.github/img/block-factory-diagram.png)

- [x] Concurrent (with tokio)
- [ ] Observable (with tracing)

Generating consensus-less blocks from incoming transactions

# Objects

- `Transaction` - Basic unit of interaction

- `Block` - Data structure for storing transactions

- `Chain` - Data structure for managing blocks

- `Node` - Data structure for managing the node (request handling, persistence)

- `Persistence` - The gateway to database I/O

# Configuration Flags

> Tip: Run `block-factory --help` for a full list of configuration flags.

- `p` and `--port` (Usage: `block-factory --port 8080`)

- `b` and `--block-time` (Usage: `block-factory --block-time 1`)

- `m` and `--mode` (Usage: `block-factory --mode full`, Options: `full`, `factory-only`, `query-only`

# API Groups

`/api` - For all API / Data interactions

`/` - For all UI rendering
