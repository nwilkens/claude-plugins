# Tune Storage Node (Mako) Configuration

Adjust mako/nginx settings via SAPI to resolve storage node performance issues.

## When to Use

- nginx `Active connections` near max capacity (`worker_processes × worker_connections`)
- High `Waiting` count in nginx_status (idle keepalive connections)
- Storage node nginx error log shows `client timed out` or `client prematurely closed connection`
- Health check or API requests timing out at storage tier

## Workflow

```
Tuning Progress:
- [ ] Step 1: Assess current state
- [ ] Step 2: Identify the storage service UUID in SAPI
- [ ] Step 3: Apply configuration change via SAPI
- [ ] Step 4: Restart config-agent on each storage CN
- [ ] Step 5: Verify config was applied
- [ ] Step 6: Verify improvement
```

### Step 1: Assess Current State

```bash
# Check current nginx stats on all storage nodes
manta-oneach -s storage "curl -s http://localhost/nginx_status"

# Check current nginx config
# First get zone-to-CN mapping:
manta-adm show | grep storage
# Then read config:
ssh <CN_IP> "zlogin <zone_uuid> cat /opt/smartdc/mako/nginx/conf/nginx.conf"
```

Key metrics:
- `Active connections` vs max capacity (`worker_processes × worker_connections`)
- `Waiting` count = idle keepalive connections consuming slots

### Step 2: Find Storage Service UUID

```bash
sdc-sapi /services?name=storage | json -Ha uuid metadata
```

Note the UUID and current metadata values.

### Step 3: Apply Change via SAPI

```bash
# Increase max connections per worker (default: 1024)
sapiadm update <service_uuid> metadata.MAKO_WORKER_CONNECTIONS=4096

# Reduce keepalive timeout (default: 0/disabled, commonly set to 86400)
sapiadm update <service_uuid> metadata.MAKO_HTTP_KEEPALIVE_TIMEOUT=300

# Verify the update
sdc-sapi /services/<service_uuid> | json metadata
```

Setting at **service level** applies to all storage zones. Use instance-level only if you need different values per zone.

### Step 4: Restart Config-Agent

Config-agent polls SAPI periodically (~30-60s), but restart for immediate pickup:

```bash
# Get storage zone-to-CN mapping
manta-adm show | grep storage

# Restart config-agent on each storage zone
ssh <CN_IP> "zlogin <zone_uuid> svcadm restart config-agent"
```

The manifest `post_cmd` is `svcadm refresh mako`, which triggers a **graceful nginx reload** — existing connections finish normally, new connections use the new config.

### Step 5: Verify Config Applied

```bash
# Check the rendered nginx.conf shows new values
ssh <CN_IP> "zlogin <zone_uuid> grep worker_connections /opt/smartdc/mako/nginx/conf/nginx.conf"
ssh <CN_IP> "zlogin <zone_uuid> grep keepalive_timeout /opt/smartdc/mako/nginx/conf/nginx.conf"
```

### Step 6: Verify Improvement

```bash
# Check nginx stats — connections should drop after reload
manta-oneach -s storage "curl -s http://localhost/nginx_status"

# Test health check
for i in $(seq 1 20); do
  curl -s -o /dev/null -w "%{http_code} %{time_total}s\n" \
    <MANTA_HEALTH_CHECK_URL>
done
```

## Rollback

To revert a SAPI change:

```bash
# Remove the metadata key (reverts to template default)
echo '{"metadata": {"MAKO_WORKER_CONNECTIONS": null}}' | \
  sdc-sapi /services/<service_uuid> -X PUT -d@-

# Or set it back to the previous value
sapiadm update <service_uuid> metadata.MAKO_WORKER_CONNECTIONS=1024

# Restart config-agent on each storage node to apply
```
