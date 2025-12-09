# [DEPLOYMENT_TYPE] Deployment

![DEPLOYMENT Deployment](./FILENAME.png)

[View source: FILENAME.drawio](./FILENAME.drawio)

## Overview

[Description of this deployment type - when to use it, key characteristics, and target use cases. Explain what makes this deployment appropriate for certain scenarios.]

## Architecture Layout

### [Section 1] - [Name, e.g., "Application Servers"]

| Component | Count | Specifications | Purpose |
|-----------|-------|----------------|---------|
| [Component 1] | [N] | [CPU/Memory/Storage] | [What it does] |
| [Component 2] | [N] | [CPU/Memory/Storage] | [What it does] |
| [Component 3] | [N] | [CPU/Memory/Storage] | [What it does] |

### [Section 2] - [Name, e.g., "Database Cluster"]

| Component | Count | Specifications | Purpose |
|-----------|-------|----------------|---------|
| [Component 1] | [N] | [CPU/Memory/Storage] | [What it does] |
| [Component 2] | [N] | [CPU/Memory/Storage] | [What it does] |

### [Section 3] - [Name, e.g., "Supporting Services"]

| Component | Count | Specifications | Purpose |
|-----------|-------|----------------|---------|
| [Component 1] | [N] | [CPU/Memory/Storage] | [What it does] |

## Resource Requirements

### Minimum Specifications

| Resource | Requirement | Notes |
|----------|-------------|-------|
| Total Servers | [N] | [Any notes] |
| Total CPUs | [N cores] | [Distribution notes] |
| Total Memory | [N GB] | [Distribution notes] |
| Total Storage | [N GB/TB] | [SSD/HDD requirements] |
| Network | [Bandwidth] | [Latency requirements] |

### Recommended Specifications

| Resource | Requirement | Notes |
|----------|-------------|-------|
| Total Servers | [N] | [Recommendation rationale] |
| Total CPUs | [N cores] | [Recommendation rationale] |
| Total Memory | [N GB] | [Recommendation rationale] |
| Total Storage | [N GB/TB] | [Recommendation rationale] |

## High Availability

### [HA Component 1, e.g., "Load Balancer HA"]

[How this component achieves high availability - active-passive, active-active, clustering, etc.]

- **Failover Time**: [Expected failover duration]
- **Data Loss**: [RPO - Recovery Point Objective]
- **Recovery**: [RTO - Recovery Time Objective]

### [HA Component 2, e.g., "Database HA"]

[How this component achieves high availability]

- **Failover Time**: [Expected duration]
- **Data Loss**: [RPO]
- **Recovery**: [RTO]

### [HA Component 3, e.g., "Application HA"]

[How this component achieves high availability]

## Limitations

| Limitation | Impact | Mitigation |
|------------|--------|------------|
| [Limitation 1] | [What this means for users/operations] | [How to work around it] |
| [Limitation 2] | [What this means] | [Workaround] |
| [Limitation 3] | [What this means] | [Workaround] |

## Recommended Use Cases

This deployment is recommended for:
- [Use case 1 - e.g., "Development and testing environments"]
- [Use case 2 - e.g., "Small production deployments (<1000 users)"]
- [Use case 3 - e.g., "Single-region deployments"]

This deployment is NOT recommended for:
- [Anti-use case 1 - e.g., "High-availability production requirements"]
- [Anti-use case 2 - e.g., "Multi-region deployments"]

## Scaling Path

### Vertical Scaling

[How to scale up individual components]

1. [Step 1 - e.g., "Increase database server memory"]
2. [Step 2 - e.g., "Upgrade CPU cores"]
3. [Limitations - when vertical scaling is no longer effective]

### Horizontal Scaling

[How to add more instances]

1. [Step 1 - e.g., "Add application server instances"]
2. [Step 2 - e.g., "Configure load balancer"]
3. [Step 3 - e.g., "Update monitoring"]

## Migration Path

### To [Larger Deployment Type]

When this deployment is no longer sufficient:

1. [Step 1 - e.g., "Provision additional infrastructure"]
2. [Step 2 - e.g., "Configure replication"]
3. [Step 3 - e.g., "Migrate data"]
4. [Step 4 - e.g., "Switch traffic"]
5. [Step 5 - e.g., "Verify and decommission old infrastructure"]

**Estimated Downtime**: [Expected downtime for migration]

## Network Topology

```
                    Internet
                        |
                   [Firewall]
                        |
                  [Load Balancer]
                   /    |    \
            [App1]  [App2]  [App3]
                   \    |    /
                  [Internal LB]
                        |
                   [Database]
```

## Related Diagrams

- [Logical Architecture](../logical-architecture.png) - Component relationships
- [Networking](./networking.png) - Network topology details
- [Other Deployment](./other-deployment.png) - Alternative deployment option

---

*Template usage: Copy this file, replace all `[BRACKETED]` placeholders with actual content, and rename to match your diagram file (e.g., `single-environment.md`).*
