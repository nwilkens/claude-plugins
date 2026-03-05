# Diagnose Health Check Failures

Step-by-step workflow for diagnosing Manta health check 500/503 errors.

## Prerequisites

SSH to the Manta headnode and set the PATH:

```bash
export PATH=/opt/smartdc/bin:/opt/local/bin:/opt/local/sbin:/opt/smartdc/agents/bin:$PATH
```

## Workflow

```
Diagnosis Progress:
- [ ] Step 1: Reproduce and characterize the failure
- [ ] Step 2: Check service health across all tiers
- [ ] Step 3: Identify which webapi zones are failing
- [ ] Step 4: Capture error details (request ID, response time, headers)
- [ ] Step 5: Check storage node health (nginx connections, I/O, disk)
- [ ] Step 6: Check metadata tier health (moray, postgres)
- [ ] Step 7: Root cause and remediate
- [ ] Step 8: Verify fix
```

### Step 1: Reproduce the Failure

```bash
# Quick status check
curl -s -o /dev/null -w "%{http_code}" \
  <MANTA_HEALTH_CHECK_URL>

# Rapid-fire test (20 requests) to measure failure rate
for i in $(seq 1 20); do
  curl -s -o /dev/null -w "%{http_code} %{time_total}s\n" \
    <MANTA_HEALTH_CHECK_URL>
done
```

Note the failure rate and response times. Slow failures (6-7s) suggest upstream timeouts. Fast failures suggest service-level issues.

### Step 2: Check Service Health

```bash
# Check all tiers for degraded services (empty output = healthy)
for svc in loadbalancer webapi storage buckets-api buckets-mdapi \
           buckets-postgres electric-moray moray authcache; do
  echo "=== $svc ==="
  manta-oneach -s $svc "svcs -x"
done
```

If any service shows output, it has degraded/maintenance SMF services.

### Step 3: Identify Failing Webapi Zones

```bash
# Capture x-server-name header to see which backends return errors
for i in $(seq 1 20); do
  result=$(curl -s -D- -o /dev/null -w "%{http_code}" \
    <MANTA_HEALTH_CHECK_URL> 2>&1)
  code=$(echo "$result" | tail -1)
  server=$(echo "$result" | grep -i x-server-name | tr -d '\r')
  echo "$code $server"
done
```

If errors come from **multiple zones** → shared dependency issue (storage or metadata).
If errors come from **one zone** → that specific webapi instance is unhealthy.

### Step 4: Capture Error Details

```bash
# Loop until we catch a non-200 with full headers
for i in $(seq 1 30); do
  code=$(curl -s -D /tmp/manta_headers -o /tmp/manta_body \
    -w "%{http_code}" \
    <MANTA_HEALTH_CHECK_URL>)
  if [ "$code" != "200" ]; then
    echo "STATUS: $code"
    cat /tmp/manta_headers
    echo "---BODY---"
    cat /tmp/manta_body
    break
  fi
done
```

Key indicators from the response:
- **`etag`/`last-modified` present + 500** → metadata resolved, storage fetch failed
- **`x-response-time` > 5000ms** → upstream timeout (likely storage node)
- **`x-request-id`** → use to trace in muskie logs

### Step 5: Check Storage Nodes

```bash
# nginx active connections and idle keepalive count
manta-oneach -s storage "curl -s http://localhost/nginx_status"

# Connection count by source IP
manta-oneach -s storage "netstat -an -f inet | grep ESTABLISHED | \
  awk '{print \$2}' | rev | cut -d. -f2- | rev | sort | uniq -c | sort -rn | head -15"

# Map source IPs to services
manta-oneach -s buckets-api "ipadm show-addr -p -o ADDR | head -3"

# Disk usage
manta-oneach -s storage "df -h /manta"

# nginx error log (via CN)
# Get zone-to-CN mapping first:
manta-adm show | grep storage
# Then:
ssh <CN_IP> "zlogin <zone_uuid> tail -50 /var/log/mako-error.log"

# nginx config
ssh <CN_IP> "zlogin <zone_uuid> cat /opt/smartdc/mako/nginx/conf/nginx.conf"

# zpool I/O from CN
ssh <CN_IP> "zpool iostat zones 5 1"
```

**Common finding:** High idle keepalive connections from buckets-api zones saturating nginx `worker_connections` limit.

### Step 6: Check Metadata Tier

```bash
# Electric-moray health
manta-oneach -s electric-moray "svcs -x"

# Moray health
manta-oneach -s moray "svcs -x"

# Check for long-running postgres queries (from postgres zones)
# Get postgres zone UUIDs:
manta-adm show | grep postgres
```

### Step 7: Remediate

Based on findings, see:
- [tune-storage-nodes.md](tune-storage-nodes.md) — for nginx connection/keepalive issues
- Restart unhealthy services: `manta-oneach -z <zone> "svcadm restart <service>"`
- For webapi issues: `manta-oneach -z <zone> "svcadm restart muskie"`

### Step 8: Verify Fix

```bash
# Run 20 health checks — all should return 200
for i in $(seq 1 20); do
  curl -s -o /dev/null -w "%{http_code} %{time_total}s\n" \
    <MANTA_HEALTH_CHECK_URL>
done

# Check nginx stats improved
manta-oneach -s storage "curl -s http://localhost/nginx_status"
```
