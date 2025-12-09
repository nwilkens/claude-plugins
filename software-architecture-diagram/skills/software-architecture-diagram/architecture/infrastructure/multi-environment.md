# Multi-Environment High Availability Deployment

![Multi-Environment](./multi-environment.png)

[View source: multi-environment.drawio](./multi-environment.drawio)

## Overview

The Multi-Environment deployment provides high availability across two availability zones with full redundancy. This configuration is suitable for production workloads requiring 99.9%+ uptime and can handle significant traffic loads.

## Architecture Layout

### Global Load Balancing

| Component | Count | Purpose |
|-----------|-------|---------|
| Global Load Balancer | 1 | DNS-based routing, health checks |

### Availability Zone 1 (Primary)

| Component | Count | Specifications | Purpose |
|-----------|-------|----------------|---------|
| Load Balancer | 1 | 4 CPU / 8GB | Zone traffic distribution |
| App Servers | 4 | 4 CPU / 8GB each | API + Services |
| PostgreSQL Primary | 1 | 8 CPU / 32GB / 500GB | Write operations |
| Redis Primary | 1 | 4 CPU / 16GB | Cache + Sessions |
| Elasticsearch | 3 | Master + 2 Data | Search cluster |
| RabbitMQ | 3 | 2 CPU / 4GB each | Message queue cluster |
| Workers | 3 | 2 CPU / 4GB each | Background jobs |

### Availability Zone 2 (Secondary)

| Component | Count | Specifications | Purpose |
|-----------|-------|----------------|---------|
| Load Balancer | 1 | 4 CPU / 8GB | Zone traffic distribution |
| App Servers | 4 | 4 CPU / 8GB each | API + Services |
| PostgreSQL Replica | 1 | 8 CPU / 32GB / 500GB | Read operations, failover |
| Redis Replica | 1 | 4 CPU / 16GB | Cache replication |
| Elasticsearch | 3 | Master + 2 Data | Search cluster |
| Workers | 2 | 2 CPU / 4GB each | Background jobs |

### Shared Services

| Component | Count | Purpose |
|-----------|-------|---------|
| Object Storage (S3) | 1 | Multi-region file storage |
| Prometheus | 1 | Metrics aggregation |
| Grafana | 1 | Dashboards |
| Alertmanager | 1 | Alert routing |
| Loki | 1 | Log aggregation |
| Jaeger | 1 | Distributed tracing |

## Resource Requirements

### Total Resources

| Resource | Amount |
|----------|--------|
| Servers | ~30 |
| Total CPUs | ~120 cores |
| Total Memory | ~300 GB |
| Total Storage | ~3 TB |

### Estimated Cost

| Environment | Monthly Cost |
|-------------|--------------|
| Cloud (AWS/GCP/Azure) | $3,000 - $6,000 |
| Reserved Instances | $2,000 - $4,000 |

## High Availability

### Database HA

- **Primary**: All writes, some reads
- **Replica**: Read operations, automatic failover target
- **RPO**: Near-zero (synchronous replication)
- **RTO**: < 1 minute (automated failover)

### Application HA

- 8 application servers across 2 zones
- Active-active configuration
- Zero-downtime deployments

### Cache HA

- Redis replication across zones
- Sentinel for automatic failover
- Cache-aside pattern for resilience

### Search HA

- 6-node Elasticsearch cluster
- 2 master-eligible nodes (1 per zone)
- 4 data nodes (2 per zone)
- Cross-cluster replication

### Message Queue HA

- 3-node RabbitMQ cluster (quorum queues)
- Mirrored queues across nodes
- Automatic failover

## Failover Scenarios

### Zone 1 Failure

1. Global LB detects health check failure
2. Traffic routed to Zone 2
3. PostgreSQL replica promoted to primary
4. Redis replica becomes primary
5. **RTO**: < 2 minutes

### Single Component Failure

| Component | Failover Mechanism | RTO |
|-----------|-------------------|-----|
| App Server | LB removes from pool | Instant |
| Database | Patroni promotes replica | < 30s |
| Redis | Sentinel promotes replica | < 30s |
| Elasticsearch | Cluster rebalances | Minutes |
| RabbitMQ | Quorum queue leadership | Seconds |

## Capacity Planning

### Current Capacity

| Metric | Capacity |
|--------|----------|
| Concurrent users | 10,000+ |
| Requests per second | 5,000+ |
| Database connections | 800 |
| Background jobs | 1,000/minute |

### Scaling Triggers

| Metric | Threshold | Action |
|--------|-----------|--------|
| CPU utilization | > 70% | Add app servers |
| Database connections | > 80% | Add read replicas |
| Response time p99 | > 500ms | Scale horizontally |
| Queue depth | > 10,000 | Add workers |

## Network Design

```
                    Internet
                        |
                   [Global LB]
                   /         \
             [Zone 1]      [Zone 2]
                |              |
           [Zone LB]      [Zone LB]
           /   |   \      /   |   \
        [Apps]  [Apps]  [Apps]  [Apps]
           \     /          \     /
         [Database]------[Database]
           Primary        Replica
              |              |
          [Redis]---------[Redis]
           Primary        Replica
```

## Configuration

### Database Configuration

```yaml
patroni:
  scope: production
  name: pg-primary
  postgresql:
    listen: 0.0.0.0:5432
    connect_address: pg-primary:5432
  bootstrap:
    dcs:
      synchronous_mode: true
      synchronous_node_count: 1
```

### Redis Sentinel Configuration

```yaml
sentinel:
  monitor: mymaster 10.0.1.10 6379 2
  down-after-milliseconds: 5000
  failover-timeout: 10000
  parallel-syncs: 1
```

## Monitoring and Alerts

### Critical Alerts

| Alert | Condition | Action |
|-------|-----------|--------|
| Zone Down | All health checks fail | Page on-call |
| Database Failover | Primary unavailable | Notify team |
| High Error Rate | > 1% 5xx responses | Investigate |
| Replication Lag | > 30 seconds | Check network |

### Dashboard Panels

- Request rate and latency by zone
- Database replication lag
- Cache hit rate
- Queue depth and processing rate
- Error rate by service

## Related Diagrams

- [Logical Architecture](../logical-architecture.png) - Component relationships
- [Single Environment](./single-environment.png) - Simpler deployment
- [Physical Architecture](../physical-architecture.png) - Full deployment view
