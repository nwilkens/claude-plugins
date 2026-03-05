# Manta Admin Plugin

Claude Code plugin for Manta object storage administration and troubleshooting.

## Features

- Diagnose health check failures (500/503 errors)
- Investigate service health across all Manta tiers
- Tune storage node (mako/nginx) configuration via SAPI
- Trace requests through the Manta stack using request IDs
- Understand haproxy routing between webapi and buckets-api

## Skills

### manta-admin
Auto-triggered when working with Manta infrastructure. Provides:
- Architecture overview (directory API vs buckets API, shared storage nodes)
- Key diagnostic commands for every service tier
- Reference docs for SAPI tunables, haproxy routing, service inventory
- Step-by-step workflows for common issues

## Commands

### /manta-admin
Invoke directly to start diagnosing a Manta issue.

## Installation

Add to your Claude Code plugins directory or symlink:
```bash
ln -s /path/to/manta-admin ~/.claude/plugins/manta-admin
```
