# Load Balancer Routing (HAProxy)

Manta uses haproxy with muppet for load balancing. Config managed by muppet service which watches for zone changes via ZooKeeper.

## Backends

| Backend | Service | Health Check | Port |
|---|---|---|---|
| `secure_api` | webapi (muskie) | `GET /ping` | 80 |
| `insecure_api` | webapi (muskie) | `GET /ping` | 81 |
| `buckets_api` | buckets-api | `check inter 30s` | 8081-8096 |

## Frontend Routing Rules (HTTPS)

```
1. If Authorization header starts with "AWS4-HMAC-SHA256" → buckets_api
2. If query contains "X-Amz-Algorithm" (presigned URL) → buckets_api
3. If OPTIONS + access-control-request-method + path matches → buckets_api (CORS)
4. If path matches /account/buckets → buckets_api
5. Default → secure_api (webapi/muskie)
```

The health check URL (`/cloudops/public/health-check.txt`) uses the **default path** → `secure_api` → webapi/muskie.

## Inspecting HAProxy

```bash
# View frontend/backend sections
manta-oneach -s loadbalancer \
  "cat /opt/smartdc/muppet/etc/haproxy.cfg | grep -E '^(frontend|backend)'"

# View webapi backends and health check config
manta-oneach -s loadbalancer \
  "cat /opt/smartdc/muppet/etc/haproxy.cfg | grep -A20 'backend secure_api'"

# View buckets-api backends
manta-oneach -s loadbalancer \
  "cat /opt/smartdc/muppet/etc/haproxy.cfg | grep -A5 'backend buckets_api'"

# Count backend servers per section
manta-oneach -s loadbalancer \
  "cat /opt/smartdc/muppet/etc/haproxy.cfg | grep -c 'server '"
```

## Key Settings

- `timeout server 240000` — 240 second backend timeout
- `check inter 30s` — health check every 30 seconds
- `slowstart 10s` — ramp up traffic to new backends over 10 seconds
- Config path: `/opt/smartdc/muppet/etc/haproxy.cfg`
- TLS cert: `/opt/smartdc/muppet/etc/ssl.pem`
