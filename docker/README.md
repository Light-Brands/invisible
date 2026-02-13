# Docker Development Environment

Local development environment for Invisible messenger using Docker Compose.

## Quick Start

### Start Development Environment

```bash
# Start all services
docker-compose up -d

# Start only dev environment
docker-compose up dev

# Start relay network (3 nodes)
docker-compose up relay-1 relay-2 relay-3
```

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f relay-1

# Development container
docker-compose logs -f dev
```

### Run Tests

```bash
# Inside dev container
docker-compose exec dev cargo test

# All tests with coverage
docker-compose exec dev cargo tarpaulin --all-features
```

### Stop Services

```bash
# Stop all services
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

## Services

### Development Container (`dev`)
- Rust development environment
- Auto-reload with `cargo watch`
- Mounted source code for live editing
- Port 8080: Development relay
- Port 8443: Development relay (HTTPS)

### Relay Nodes (`relay-1`, `relay-2`, `relay-3`)
- Production relay node builds
- Simulates 3-layer mixnet
- Health checks enabled
- Ports 9001-9003: Relay HTTP endpoints

### PostgreSQL (`postgres`)
- Development database
- Port 5432
- User: invisible
- Password: invisible_dev_only
- Database: invisible

### Redis (`redis`)
- Relay coordination
- Port 6379

### Prometheus (`prometheus`)
- Metrics collection
- Port 9090
- Scrapes relay nodes every 15s

### Grafana (`grafana`)
- Metrics visualization
- Port 3000
- User: admin
- Password: invisible_dev_only

## Development Workflow

### 1. Start Environment

```bash
docker-compose up -d dev postgres redis
```

### 2. Enter Container

```bash
docker-compose exec dev bash
```

### 3. Run Commands

```bash
# Build
cargo build

# Test
cargo test

# Lint
cargo clippy

# Format
cargo fmt

# Security audit
cargo audit
```

### 4. Test Relay Network

```bash
# Start 3-node relay network
docker-compose up -d relay-1 relay-2 relay-3

# Check health
curl http://localhost:9001/health
curl http://localhost:9002/health
curl http://localhost:9003/health
```

## Monitoring

### Prometheus Metrics

Access Prometheus at http://localhost:9090

Query examples:
- `relay_messages_total` - Total messages processed
- `relay_latency_seconds` - Message latency
- `relay_connections_active` - Active connections

### Grafana Dashboards

Access Grafana at http://localhost:3000

Default credentials:
- Username: admin
- Password: invisible_dev_only

## Volumes

- `cargo-cache` - Cargo registry cache (speeds up builds)
- `target-cache` - Build artifacts cache
- `postgres-data` - Database persistence
- `prometheus-data` - Metrics storage
- `grafana-data` - Dashboard configurations

## Troubleshooting

### Clean Build

```bash
# Remove all containers and volumes
docker-compose down -v

# Remove build cache
docker-compose build --no-cache
```

### Permission Issues

```bash
# Fix ownership (run on host)
sudo chown -R $USER:$USER .
```

### Network Issues

```bash
# Recreate network
docker-compose down
docker network prune
docker-compose up -d
```

## Security Notes

⚠️ **This is a development environment only!**

- Default passwords are insecure
- Services expose ports on localhost
- No encryption between containers
- Volume data is not encrypted

**Never use in production!**
