# Create / Write Flow

![Create Flow](./create-flow.png)

[View source: create-flow.drawio](./create-flow.drawio)

## Overview

The Create flow describes how data travels through the system when a client creates or writes a new resource. This flow involves all three tiers: API Tier for request handling and validation, Business Tier for logic and transactions, and Data Tier for persistence and replication.

## Flow Steps

### API Tier

#### 1. HTTPS Request

| Attribute | Value |
|-----------|-------|
| Component | Load Balancer / API Gateway |
| Action | Receive incoming request |
| Input | POST /api/resource with JSON body |
| Output | Parsed request with auth token |

#### 2. Load Balancer

| Attribute | Value |
|-----------|-------|
| Component | HAProxy / Nginx |
| Action | SSL termination, route to gateway |
| Input | HTTPS request |
| Output | HTTP request to backend |

#### 3. Auth Check

| Attribute | Value |
|-----------|-------|
| Component | Auth Service |
| Action | Validate JWT token, check RBAC permissions |
| Input | Authorization header |
| Output | User context with permissions |

#### 4. Rate Limit

| Attribute | Value |
|-----------|-------|
| Component | Rate Limiter |
| Action | Check and increment rate limit counter |
| Input | User ID / IP address |
| Output | Pass / 429 Too Many Requests |

#### 5. Validation

| Attribute | Value |
|-----------|-------|
| Component | API Gateway |
| Action | Schema validation, input sanitization |
| Input | Request body |
| Output | Validated and sanitized data |

### Business Tier

#### 6. Service Router

| Attribute | Value |
|-----------|-------|
| Component | Service Discovery |
| Action | Find target service, load balance |
| Input | Request path |
| Output | Target service instance |

#### 7. Business Logic

| Attribute | Value |
|-----------|-------|
| Component | Domain Service |
| Action | Apply business rules, transform data |
| Input | Validated request |
| Output | Domain object ready for persistence |

#### 8. Transaction

| Attribute | Value |
|-----------|-------|
| Component | Transaction Manager |
| Action | Begin transaction, prepare writes |
| Input | Domain object |
| Output | Transaction context |

#### 9. Publish Event (Async)

| Attribute | Value |
|-----------|-------|
| Component | Message Broker |
| Action | Publish domain event |
| Input | Event payload |
| Output | Message acknowledgment |

#### 10. Audit Log

| Attribute | Value |
|-----------|-------|
| Component | Audit Service |
| Action | Record who/what/when |
| Input | Action details |
| Output | Audit entry |

### Data Tier

#### 11. Connection Pool

| Attribute | Value |
|-----------|-------|
| Component | PgBouncer |
| Action | Acquire database connection |
| Input | Connection request |
| Output | Database connection |

#### 12. Write Primary

| Attribute | Value |
|-----------|-------|
| Component | PostgreSQL Primary |
| Action | INSERT/UPDATE statement |
| Input | SQL with parameters |
| Output | Affected rows, generated ID |

#### 13. Replicate

| Attribute | Value |
|-----------|-------|
| Component | PostgreSQL Replicas |
| Action | Streaming replication |
| Input | WAL records |
| Output | Replicated data |

#### 14. Invalidate Cache

| Attribute | Value |
|-----------|-------|
| Component | Redis |
| Action | Delete or update cached keys |
| Input | Cache keys |
| Output | Cache updated |

#### 15. Index Update (Async)

| Attribute | Value |
|-----------|-------|
| Component | Elasticsearch |
| Action | Update search index |
| Input | Document data |
| Output | Index updated |

#### 16. Response

| Attribute | Value |
|-----------|-------|
| Component | API Gateway |
| Action | Return success response |
| Input | Created resource |
| Output | HTTP 201 Created |

## Request/Response Format

### Request

```http
POST /api/resources HTTP/1.1
Host: api.example.com
Authorization: Bearer <jwt-token>
Content-Type: application/json

{
  "name": "Example Resource",
  "description": "A new resource",
  "metadata": {
    "category": "example"
  }
}
```

### Response

```http
HTTP/1.1 201 Created
Content-Type: application/json
Location: /api/resources/12345

{
  "id": "12345",
  "name": "Example Resource",
  "description": "A new resource",
  "metadata": {
    "category": "example"
  },
  "created_at": "2024-01-01T00:00:00Z",
  "created_by": "user-uuid"
}
```

## Error Handling

| Scenario | Behavior | Response Code |
|----------|----------|---------------|
| Invalid token | Return unauthorized | 401 |
| Permission denied | Return forbidden | 403 |
| Rate limit exceeded | Return rate limited | 429 |
| Validation failed | Return bad request | 400 |
| Database error | Rollback transaction | 500 |
| Service unavailable | Circuit breaker open | 503 |

## Performance Characteristics

- **Latency**: ~50-100ms (dominated by database write)
- **Throughput**: ~500-1000 writes/second per instance
- **Scalability**: Horizontal scaling of Business Tier services

## Sequence Diagram

```
Client    LB      Gateway    Service    Database   Cache
  |        |         |          |           |        |
  |--Req-->|         |          |           |        |
  |        |--Auth-->|          |           |        |
  |        |         |--Logic-->|           |        |
  |        |         |          |--Write--->|        |
  |        |         |          |--Invalidate------->|
  |        |         |          |<--Ack-----|        |
  |        |         |<--Res----|           |        |
  |        |<--Res---|          |           |        |
  |<--201--|         |          |           |        |
```

## Related Diagrams

- [Logical Architecture](../logical-architecture.png) - Overall system view
- [API Tier](../layers/api-tier.md) - Entry point details
- [Data Tier](../layers/data-tier.md) - Storage details
- [Read Flow](./read-flow.png) - Read operation path
