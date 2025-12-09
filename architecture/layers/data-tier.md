# Data Tier

![Data Tier Architecture](./data-tier.png)

[View source: data-tier.drawio](./data-tier.drawio)

## Overview

The Data Tier handles all persistent storage, caching, and data management. It includes relational databases, caches, search indexes, object storage, and backup systems. This tier is responsible for data durability, consistency, and high availability.

## Components

### Relational Database Cluster

PostgreSQL cluster with streaming replication.

| Component | Role | Specifications |
|-----------|------|----------------|
| Primary | Read + Write | 8 CPU / 32GB / 500GB SSD |
| Replica 1 | Read Only | 8 CPU / 32GB / 500GB SSD |
| Replica 2 | Read Only | 8 CPU / 32GB / 500GB SSD |
| Replica 3 | Read Only (DR) | 8 CPU / 32GB / 500GB SSD |

**Features:**
- Synchronous replication to Replica 1
- Asynchronous replication to Replicas 2-3
- Automatic failover via Patroni
- Point-in-time recovery (PITR)
- Connection pooling via PgBouncer

### Caching Layer

Redis cluster for high-performance caching.

| Component | Purpose | Specifications |
|-----------|---------|----------------|
| Redis Primary | Cache + Session | 4 CPU / 16GB |
| Redis Replica | Hot Standby | 4 CPU / 16GB |
| Memcached | Session Store | 2 CPU / 8GB |

**Features:**
- Redis Cluster mode for horizontal scaling
- Sentinel for automatic failover
- TTL-based eviction
- Pub/Sub for real-time updates

### Search & Analytics

Elasticsearch cluster for full-text search.

| Component | Role | Purpose |
|-----------|------|---------|
| Master Node | Cluster coordination | Index management |
| Data Node 1 (Hot) | Recent data | Active queries |
| Data Node 2 (Hot) | Recent data | Active queries |
| Data Node 3 (Warm) | Older data | Less frequent access |

**Features:**
- Index lifecycle management (ILM)
- Cross-cluster replication
- Snapshot and restore
- Hot-Warm-Cold architecture

### Object Storage

S3-compatible storage for files and media.

| Component | Purpose | Retention |
|-----------|---------|-----------|
| S3 / MinIO | Primary storage | Indefinite |
| CDN (CloudFront) | Edge caching | Cache-based |

**Features:**
- Unlimited scalability
- Multi-region replication
- Versioning support
- Lifecycle policies

### Time Series / Metrics

Specialized storage for metrics and monitoring data.

| Component | Purpose | Retention |
|-----------|---------|-----------|
| InfluxDB | Application metrics | 30 days |
| Prometheus | Short-term metrics | 15 days |

**Features:**
- High write throughput
- Efficient compression
- Built-in aggregation
- Downsampling

### Backup & Archive

Data protection and long-term storage.

| Component | Purpose | Retention |
|-----------|---------|-----------|
| DB Backups | Point-in-time recovery | 30 days |
| Long-term Archive | Compliance storage | 7 years |

**Features:**
- Daily full backups
- Continuous WAL archiving
- Encrypted at rest
- Cross-region replication

## Data Flow

```
1. Request from Business Tier
       |
2. Connection Pool (PgBouncer)
       |
3. Read/Write Router (HAProxy)
       |
   +---+---+
   |       |
Write    Read
   |       |
Primary  Replicas
```

## Scaling

| Component | Strategy |
|-----------|----------|
| PostgreSQL | Vertical + Read replicas |
| Redis | Cluster mode (horizontal) |
| Elasticsearch | Add data nodes |
| Object Storage | Unlimited (managed) |

## Configuration

### Database Configuration

| Parameter | Description | Recommended |
|-----------|-------------|-------------|
| `max_connections` | Max DB connections | 200 (use pooler) |
| `shared_buffers` | Memory for caching | 8GB (25% of RAM) |
| `work_mem` | Per-query memory | 256MB |
| `wal_level` | WAL detail level | replica |

### Cache Configuration

| Parameter | Description | Recommended |
|-----------|-------------|-------------|
| `maxmemory` | Max cache size | 12GB |
| `maxmemory-policy` | Eviction policy | allkeys-lru |
| `timeout` | Client timeout | 300s |

## Monitoring

| Metric | Description | Alert Threshold |
|--------|-------------|-----------------|
| Replication lag | Replica behind primary | > 10 seconds |
| Connection count | Active connections | > 80% of max |
| Cache hit rate | Cache effectiveness | < 90% |
| Disk usage | Storage utilization | > 80% |
| Query latency (p99) | Slow queries | > 100ms |

## Related Diagrams

- [Logical Architecture](../logical-architecture.png) - Overall system view
- [Business Tier](./business-tier.md) - Upstream tier
- [Create Flow](../diagrams/create-flow.png) - Write path
- [Read Flow](../diagrams/read-flow.png) - Read path with caching
