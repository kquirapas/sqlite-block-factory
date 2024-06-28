# sqlite block factory

- [x] Concurrent (with tokio)
- [ ] Observable (with tracing)

Generating consensus-less blocks from incoming transactions

# Configuration Flags

> Tip: Run `block-factory --help` for a full list of configuration flags.

- `p` and `--port` (Usage: `block-factory --port 8080`)

- `b` and `--block-time` (Usage: `block-factory --block-time 1`)

- `m` and `--mode` (Usage: `block-factory --mode full`, Options: `full`, `factory-only`, `query-only`
