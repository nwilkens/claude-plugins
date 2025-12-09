# Business Tier

![Business Tier Architecture](./business-tier.png)

[View source: business-tier.drawio](./business-tier.drawio)

## Overview

The Business Tier implements all application logic, processing, and orchestration. It contains domain services, background workers, scheduled jobs, and message processing components. This tier is responsible for business rules, data transformation, and coordinating operations across the system.

## Components

### Core Services

#### User Service

| Attribute | Value |
|-----------|-------|
| Technology | Node.js / Express |
| Purpose | Account management, profile operations |
| Scales | Horizontal |

**Features:**
- User registration and authentication
- Profile management
- Preferences and settings
- Account linking

#### Product Service

| Attribute | Value |
|-----------|-------|
| Technology | Go |
| Purpose | Catalog management, inventory |
| Scales | Horizontal |

**Features:**
- Product CRUD operations
- Category management
- Inventory tracking
- Price management

#### Order Service

| Attribute | Value |
|-----------|-------|
| Technology | Java / Spring Boot |
| Purpose | Order processing and lifecycle |
| Scales | Horizontal |

**Features:**
- Order creation and validation
- Status tracking
- Order history
- Fulfillment coordination

#### Payment Service

| Attribute | Value |
|-----------|-------|
| Technology | Rust |
| Purpose | Payment processing |
| Scales | Horizontal |

**Features:**
- Payment gateway integration
- Transaction processing
- Refund handling
- PCI compliance

### Message Processing

#### Message Broker

| Attribute | Value |
|-----------|-------|
| Technology | RabbitMQ or Apache Kafka |
| Purpose | Event-driven communication |
| Scales | Cluster (3+ nodes) |

**Features:**
- Publish/subscribe messaging
- Message persistence
- Dead letter queues
- Message routing

#### Event Processor

| Attribute | Value |
|-----------|-------|
| Technology | Consumer groups |
| Purpose | Process async events |
| Scales | Horizontal |

**Features:**
- Event consumption
- At-least-once delivery
- Parallel processing
- Retry logic

### Background Workers

Workers process long-running tasks asynchronously.

| Worker | Purpose | Schedule |
|--------|---------|----------|
| Import Worker | Bulk data processing | On-demand |
| Export Worker | Report generation | On-demand |
| Cleanup Worker | Data maintenance | Scheduled |

### Scheduled Jobs

| Job | Purpose | Schedule |
|-----|---------|----------|
| Daily Reports | Generate business reports | 00:00 UTC |
| Cache Refresh | Update cached data | Every 5 min |
| Health Check | Service health verification | Every 1 min |

### Service Infrastructure

#### Service Discovery

| Attribute | Value |
|-----------|-------|
| Technology | Consul or etcd |
| Purpose | Dynamic service registration |
| Scales | Cluster (3+ nodes) |

#### Service Mesh

| Attribute | Value |
|-----------|-------|
| Technology | Istio / Envoy |
| Purpose | Service-to-service communication |
| Scales | Per-service sidecar |

## Request Flow

```
1. Request from API Tier
       |
2. Service Discovery (find target service)
       |
3. Load Balance (select instance)
       |
4. Circuit Breaker Check
       |
5. Service Processing
       |
6. Data Tier Operations (if needed)
       |
7. Response / Event Publication
```

## Scaling

- **Core Services**: Horizontal scaling based on CPU/memory metrics
- **Workers**: Scale based on queue depth
- **Message Broker**: Cluster for high availability
- **Service Mesh**: Sidecar per service instance

## Configuration

### Service Configuration

| Parameter | Description | Recommended |
|-----------|-------------|-------------|
| `REPLICAS` | Number of service instances | 3+ for production |
| `CPU_REQUEST` | CPU allocation | 500m - 2000m |
| `MEMORY_REQUEST` | Memory allocation | 512Mi - 2Gi |
| `CIRCUIT_BREAKER_THRESHOLD` | Failure threshold | 5 failures |

### Message Queue Configuration

| Parameter | Description | Recommended |
|-----------|-------------|-------------|
| `PREFETCH_COUNT` | Messages per consumer | 10 |
| `RETRY_COUNT` | Max retry attempts | 3 |
| `RETRY_DELAY` | Delay between retries | Exponential backoff |

## Monitoring

| Metric | Description | Alert Threshold |
|--------|-------------|-----------------|
| Service latency | Response time per service | > 200ms |
| Queue depth | Messages waiting | > 1000 |
| Error rate | Failures / total | > 0.5% |
| Circuit breaker state | Open/closed/half-open | Open state |
| Worker job duration | Time per job | > 5 minutes |

## Related Diagrams

- [Logical Architecture](../logical-architecture.png) - Overall system view
- [API Tier](./api-tier.md) - Upstream tier
- [Data Tier](./data-tier.md) - Downstream tier
- [Create Flow](../diagrams/create-flow.png) - Write operations
