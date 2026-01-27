# Triton CLI Plugin

A Claude Code plugin for managing Triton DataCenter infrastructure using the `triton` CLI.

## Features

- Instance management (create, list, delete, start/stop)
- Network configuration (VLANs, fabric networks, NICs)
- CNS service discovery with instance tags
- Firewall rule management
- Triton-Moirai load balancer deployment
- Volume management for persistent storage

## Installation

Add to your Claude Code settings:

```json
{
  "plugins": [
    "github:nwilkens/claude-plugins/triton"
  ]
}
```

## Usage

### Slash Command

Use `/triton` to invoke the infrastructure management workflow.

### Automatic Skill Loading

The triton skill loads automatically when you ask about:
- Creating or managing Triton instances
- Configuring networks or VLANs
- Setting up CNS service discovery
- Managing firewall rules
- Deploying load balancers

## Documentation

- [SKILL.md](skills/triton/SKILL.md) - Main skill documentation
- [commands/REFERENCE.md](skills/triton/commands/REFERENCE.md) - Complete CLI reference
- [patterns/](skills/triton/patterns/) - Common patterns and best practices
- [workflows/](skills/triton/workflows/) - Step-by-step deployment workflows

## Prerequisites

1. Install the triton CLI
2. Configure a profile: `triton profile create`
3. Enable CNS: `triton account update triton_cns_enabled=true`
