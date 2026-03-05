---
name: manta-admin
description: Administer and troubleshoot Manta object storage. Diagnose health check failures, investigate service health across all tiers (loadbalancer, webapi, buckets-api, moray, postgres, storage), tune mako/nginx configuration via SAPI, and check storage node performance. Use when working with Manta infrastructure, mako, muskie, haproxy, or SAPI service configuration.
---

# Manta Administration

Skill for diagnosing and resolving Manta object storage issues. Covers the full Manta stack from load balancers through metadata tiers to storage nodes.

## Architecture Overview

Manta has two API paths sharing the same storage nodes:

- **Directory API** (webapi/muskie) — traditional Manta path-based API
- **Buckets API** (buckets-api) — S3-compatible API

```
                    ┌─────────────┐
                    │ Load Balancer│ (haproxy/muppet)
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │ SigV4/bucket→           │ default→
     ┌────────▼────────┐      ┌────────▼────────┐
     │   Buckets API   │      │  Webapi (Muskie) │
     └────────┬────────┘      └────────┬────────┘
              │                        │
     ┌────────▼────────┐      ┌────────▼────────┐
     │  buckets-mdapi  │      │  electric-moray  │
     │  buckets-postgres│      │  moray → postgres│
     └────────┬────────┘      └────────┬────────┘
              │                        │
              └────────────┬───────────┘
                    ┌──────▼──────┐
                    │ Storage Nodes│ (mako/nginx)
                    └─────────────┘
```

## Headnode Access

All commands run from the Manta headnode. After SSHing to the headnode, set the PATH:

```bash
export PATH=/opt/smartdc/bin:/opt/local/bin:/opt/local/sbin:/opt/smartdc/agents/bin:$PATH
```

Always set the PATH before running `manta-oneach`, `manta-adm`, `sdc-sapi`, or `sapiadm`.

## Diagnostic Approach

When investigating Manta issues, work top-down through the stack:

1. **Reproduce the issue** — confirm the symptom
2. **Check service health** — `svcs -x` across all tiers
3. **Check loadbalancer routing** — haproxy config and backend health
4. **Check metadata tier** — moray, electric-moray, postgres
5. **Check storage nodes** — nginx connections, disk I/O, capacity
6. **Correlate logs** — use `x-request-id` to trace through muskie logs

See [workflows/diagnose-health-check.md](workflows/diagnose-health-check.md) for the full diagnostic workflow.

## Key Commands

### Service Health
```bash
# Check all instances of a service for degraded state
manta-oneach -s <service> "svcs -x"

# Services: loadbalancer, webapi, storage, buckets-api,
#   buckets-mdapi, buckets-mdplacement, buckets-postgres,
#   electric-moray, moray, authcache, garbage-collector

# List all Manta service instances with CN mapping
manta-adm show
```

### Storage Node Diagnostics
```bash
# nginx connection count and status
manta-oneach -s storage "curl -s http://localhost/nginx_status"

# Established connection count
manta-oneach -s storage "netstat -an | grep -c ESTABLISHED"

# Top source IPs connecting to storage
manta-oneach -s storage "netstat -an -f inet | grep ESTABLISHED | \
  awk '{print \$2}' | rev | cut -d. -f2- | rev | sort | uniq -c | sort -rn | head -15"

# Disk usage
manta-oneach -s storage "df -h /manta"

# nginx error log (from CN)
ssh <CN_IP> "zlogin <storage_zone_uuid> tail -50 /var/log/mako-error.log"

# nginx config
ssh <CN_IP> "zlogin <storage_zone_uuid> cat /opt/smartdc/mako/nginx/conf/nginx.conf"
```

### SAPI Configuration
```bash
# View storage service metadata
sdc-sapi /services?name=storage | json -Ha uuid metadata

# Update storage service tunable (applies to all mako zones)
sapiadm update <service_uuid> metadata.<KEY>=<value>

# Force config-agent to pick up changes
ssh <CN_IP> "zlogin <zone_uuid> svcadm restart config-agent"
```

### Loadbalancer
```bash
# Check haproxy backend sections
manta-oneach -s loadbalancer "cat /opt/smartdc/muppet/etc/haproxy.cfg | grep -E '^(frontend|backend)'"

# Check webapi backends
manta-oneach -s loadbalancer "cat /opt/smartdc/muppet/etc/haproxy.cfg | grep -A50 'backend secure_api'"
```

### Log Tracing
```bash
# Muskie logs are in /var/log/manta/upload/ inside webapi zones
# Files named: muskie_<zone>_<timestamp>_<port>.log (bunyan JSON format)
# Ports 8081-8096 (one log per worker)

# Search by request ID
manta-oneach -z <zone_uuid> "cd /var/log/manta/upload && \
  grep -rh <request_id> muskie_*.log"

# Search for errors
manta-oneach -s webapi "cd /var/log/manta/upload && \
  grep -l InternalError \$(ls -t muskie_*.log | head -1)"
```

## Documentation Structure

- **[reference/services.md](reference/services.md)** — Service inventory, zone-to-CN mapping, network topology
- **[reference/sapi-tunables.md](reference/sapi-tunables.md)** — SAPI configuration variables for mako, muskie, and other services
- **[reference/haproxy-routing.md](reference/haproxy-routing.md)** — Load balancer routing rules and backend configuration
- **[reference/debugging.md](reference/debugging.md)** — Log locations, request tracing, debugging API failures
- **[workflows/diagnose-health-check.md](workflows/diagnose-health-check.md)** — Step-by-step health check failure diagnosis
- **[workflows/tune-storage-nodes.md](workflows/tune-storage-nodes.md)** — Storage node nginx tuning via SAPI

For comprehensive documentation, see the [Manta Operator Guide](https://github.com/TritonDataCenter/manta/tree/master/docs/operator-guide).
