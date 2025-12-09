# [TIER_NAME] Tier

![TIER_NAME Architecture](./FILENAME.png)

[View source: FILENAME.drawio](./FILENAME.drawio)

## Overview

The [TIER_NAME] tier [brief description of responsibility and role in the overall architecture. Explain what this tier handles and why it exists as a separate layer].

## Components

### [Component 1 Name]

[Brief description of what this component does and why it's needed]

| Attribute | Value |
|-----------|-------|
| Technology | [Technology stack, e.g., Node.js, PostgreSQL] |
| Purpose | [Primary purpose in 1-2 sentences] |
| Scales | [Horizontal / Vertical / Both / N/A] |

**Features:**
- [Feature 1]
- [Feature 2]
- [Feature 3]

### [Component 2 Name]

[Brief description of what this component does]

| Attribute | Value |
|-----------|-------|
| Technology | [Technology stack] |
| Purpose | [Primary purpose] |
| Scales | [Scaling approach] |

**Features:**
- [Feature 1]
- [Feature 2]
- [Feature 3]

### [Component 3 Name]

[Repeat structure for each component in this tier]

## Request Flow

```
1. [First step - what happens when a request enters this tier]
       |
2. [Second step - processing or routing]
       |
3. [Third step - interaction with other components]
       |
4. [Final step - response or handoff to next tier]
```

## Scaling

[Description of how this tier scales, including:]
- **Horizontal scaling**: [How to add more instances]
- **Vertical scaling**: [Limits and considerations]
- **Key metrics**: [What to monitor for scaling decisions]

## Configuration

### [Component 1] Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| [PARAM_1] | [What this controls] | [Default value] |
| [PARAM_2] | [What this controls] | [Default value] |

### [Component 2] Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| [PARAM_1] | [What this controls] | [Default value] |

## Monitoring

Key metrics to monitor for this tier:

| Metric | Description | Alert Threshold |
|--------|-------------|-----------------|
| [Metric 1] | [What it measures] | [When to alert] |
| [Metric 2] | [What it measures] | [When to alert] |
| [Metric 3] | [What it measures] | [When to alert] |

## Related Diagrams

- [Logical Architecture](../logical-architecture.png) - Overall system view
- [Related Flow](../diagrams/FLOW.png) - [How this tier participates in flows]
- [Infrastructure](../infrastructure/DEPLOYMENT.png) - [How this tier is deployed]

---

*Template usage: Copy this file, replace all `[BRACKETED]` placeholders with actual content, and rename to match your diagram file (e.g., `api-tier.md`).*
