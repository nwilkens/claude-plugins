# SAPI Configuration Tunables

Manta service configuration is managed through SAPI (Services API). Config-agent in each zone polls SAPI, re-renders config templates, and runs a `post_cmd` to apply changes.

## How SAPI Config Works

1. Each service has a **template** (e.g., `/opt/smartdc/mako/sapi_manifests/mako/template`)
2. Templates use mustache syntax: `{{VARIABLE_NAME}}` with defaults via `{{^VARIABLE_NAME}}`
3. A **manifest.json** specifies the output path and post_cmd
4. Variables can be set at **service level** (all instances) or **instance level** (one zone)

## Managing SAPI Metadata

```bash
# Find the service UUID
sdc-sapi /services?name=storage | json -Ha uuid metadata

# View current metadata
sdc-sapi /services/<service_uuid> | json metadata

# Set a service-level variable (applies to all zones)
sapiadm update <service_uuid> metadata.<KEY>=<value>

# Set an instance-level variable (one zone only)
sapiadm update <instance_uuid> metadata.<KEY>=<value>

# After updating, force config-agent to pick up changes immediately:
ssh <CN_IP> "zlogin <zone_uuid> svcadm restart config-agent"
# Or wait for config-agent polling interval (~30s-60s)
```

## Storage (Mako) Tunables

Template: `/opt/smartdc/mako/sapi_manifests/mako/template`
Output: `/opt/smartdc/mako/nginx/conf/nginx.conf`
Post-cmd: `svcadm refresh mako` (graceful nginx reload)

| Variable | Default | Description |
|---|---|---|
| `MAKO_WORKER_PROCESSES` | 8 | nginx worker processes |
| `MAKO_WORKER_CONNECTIONS` | 1024 | Max connections per worker. Total max = workers × connections |
| `MAKO_HTTP_KEEPALIVE_TIMEOUT` | 0 (disabled) | Keepalive timeout in seconds. Set to enable keepalive. See MANTA-3084 |
| `MAKO_THREAD_POOL_SIZE` | 8 | Thread pool for multipart upload commit |

### Keepalive Configuration (MANTA-3084)

The template default for keepalive is **0 (disabled)**. When `MAKO_HTTP_KEEPALIVE_TIMEOUT` is set:
- `keepalive_timeout` = the value provided
- `keepalive_requests` = 1,000,000
- Socket-level keepalive: `so_keepalive=10s:30s:10`

When unset (default):
- `keepalive_timeout` = 0
- `keepalive_requests` = 0

**Capacity planning:** Total max connections = `MAKO_WORKER_PROCESSES` × `MAKO_WORKER_CONNECTIONS`. With defaults (8 × 1024 = 8,192). If keepalive is enabled with a long timeout, idle connections consume these slots.

### Common Tuning Scenarios

**Storage nodes connection-starved (high idle keepalive count):**
```bash
# Increase connection capacity
sapiadm update <uuid> metadata.MAKO_WORKER_CONNECTIONS=4096

# Reduce keepalive timeout to free idle connections faster
sapiadm update <uuid> metadata.MAKO_HTTP_KEEPALIVE_TIMEOUT=300
```

**After changes, restart config-agent on each storage CN:**
```bash
ssh <CN_IP> "zlogin <zone_uuid> svcadm restart config-agent"
```

**Verify the change took effect:**
```bash
ssh <CN_IP> "zlogin <zone_uuid> grep worker_connections /opt/smartdc/mako/nginx/conf/nginx.conf"
```

## Webapi (Muskie) Tunables

Template: muskie config template
Output: muskie configuration

### Request Throttle

Muskie includes a request throttle to prevent overload. When enabled, requests beyond the concurrency limit are queued, and requests that wait too long are rejected with 503.

| Variable | Default | Description |
|---|---|---|
| `MUSKIE_THROTTLE_ENABLED` | false | Enable request throttling |
| `MUSKIE_THROTTLE_CONCURRENCY` | 50 | Max concurrent requests per muskie worker before queueing |
| `MUSKIE_THROTTLE_QUEUE_TOLERANCE` | 25 | Max queued requests per worker before rejecting with 503 |

When throttle is triggered, muskie returns `503 Service Unavailable` with `x-server-name` header identifying the zone. Check muskie logs for `ThrottledError` entries.

```bash
# Enable throttle on webapi service
sdc-sapi /services?name=webapi | json -Ha uuid
sapiadm update <webapi_uuid> metadata.MUSKIE_THROTTLE_ENABLED=true
sapiadm update <webapi_uuid> metadata.MUSKIE_THROTTLE_CONCURRENCY=50
sapiadm update <webapi_uuid> metadata.MUSKIE_THROTTLE_QUEUE_TOLERANCE=25
```

### Other Webapi Tunables

| Variable | Default | Description |
|---|---|---|
| `WEBAPI_USE_PICKER` | (varies) | When true, muskie uses local picker instead of storinfo service for storage node selection |
| `MUSKIE_MPU_PREFIX_DIR_LEN` | 1 | Prefix directory length for multipart upload directories |
| `MUSKIE_DEFAULT_MAX_STREAMING_SIZE_MB` | 5120 | Max streaming object size in MB |
| `MUSKIE_MAX_PERCENT_UTIL` | 90 | Max storage node utilization percentage before excluding from writes |
| `MUSKIE_MAX_OPERATOR_PERCENT_UTIL` | 92 | Max utilization for operator account writes |

## Storinfo Tunables

| Variable | Default | Description |
|---|---|---|
| `STORINFO_MORAY_POLLING_INTERVAL` | 30000 | Interval (ms) for refreshing storage node data from moray |

## Service UUIDs

Service UUIDs are installation-specific. Always look them up:
```bash
# List all Manta service UUIDs
sdc-sapi '/services?application_uuid=*' | json -Ha name uuid | grep -v '^$' | sort
```

## Further Reading

See the [Manta Operator Guide](https://github.com/TritonDataCenter/manta/tree/master/docs/operator-guide) for complete tunable documentation.
