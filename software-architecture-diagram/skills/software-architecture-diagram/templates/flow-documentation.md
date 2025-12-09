# [OPERATION_NAME] Flow

![OPERATION Flow](./FILENAME.png)

[View source: FILENAME.drawio](./FILENAME.drawio)

## Overview

The [OPERATION] flow describes [what happens when this operation is triggered]. This flow involves [which tiers/components] and is responsible for [the outcome of this flow].

## Flow Steps

### 1. [Step Name]

[Detailed description of what happens in this step]

| Attribute | Value |
|-----------|-------|
| Component | [Which component handles this step] |
| Action | [What action is performed] |
| Input | [What data/request is received] |
| Output | [What is produced/returned] |

### 2. [Step Name]

[Detailed description of this step]

| Attribute | Value |
|-----------|-------|
| Component | [Component name] |
| Action | [Action performed] |
| Input | [Input data] |
| Output | [Output data] |

### 3. [Step Name]

[Continue for each step in the flow...]

| Attribute | Value |
|-----------|-------|
| Component | [Component name] |
| Action | [Action performed] |
| Input | [Input data] |
| Output | [Output data] |

### 4. [Final Step Name]

[Description of the final step and what it produces]

| Attribute | Value |
|-----------|-------|
| Component | [Component name] |
| Action | [Action performed] |
| Input | [Input data] |
| Output | [Final output/response] |

## Request/Response Format

### Request

```json
{
  "field1": "value1",
  "field2": "value2",
  "nested": {
    "field3": "value3"
  }
}
```

### Response

```json
{
  "status": "success",
  "data": {
    "id": "...",
    "created_at": "..."
  }
}
```

## Error Handling

| Scenario | Behavior | Response Code | Message |
|----------|----------|---------------|---------|
| [Error scenario 1] | [What the system does] | [HTTP code] | [Error message] |
| [Error scenario 2] | [What the system does] | [HTTP code] | [Error message] |
| [Error scenario 3] | [What the system does] | [HTTP code] | [Error message] |

## Performance Characteristics

- **Latency**: [Typical latency and what dominates it, e.g., "~50ms, dominated by database query"]
- **Throughput**: [Limiting factors, e.g., "Limited by database connections, ~1000 req/s per instance"]
- **Scalability**: [How performance scales with load]

## Sequence Diagram (ASCII)

```
Client          API Gateway       Service         Database
   |                 |               |               |
   |-- Request ----->|               |               |
   |                 |-- Auth ------>|               |
   |                 |<-- Token -----|               |
   |                 |-- Process --->|               |
   |                 |               |-- Query ----->|
   |                 |               |<-- Data ------|
   |                 |<-- Result ----|               |
   |<-- Response ----|               |               |
   |                 |               |               |
```

## Related Diagrams

- [Logical Architecture](../logical-architecture.png) - Overall system view
- [API Tier](../layers/api-tier.md) - Entry point components
- [Data Tier](../layers/data-tier.md) - Storage components involved
- [Other Flow](./other-flow.png) - Related operation flow

---

*Template usage: Copy this file, replace all `[BRACKETED]` placeholders with actual content, and rename to match your diagram file (e.g., `create-flow.md`).*
