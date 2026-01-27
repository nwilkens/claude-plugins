# Load Balancing with Triton-Moirai

Triton-Moirai is a metadata-driven HAProxy-based load balancer for Triton. Configure load balancing entirely through instance metadata - no manual HAProxy configuration needed.

## Overview

Triton-Moirai:
- Runs as a tenant-managed instance using the `cloud-load-balancer` image
- Configured entirely via instance metadata (`cloud.tritoncompute:*` keys)
- Integrates with CNS for automatic backend discovery
- Supports HTTP, HTTPS (with Let's Encrypt), TCP, and PROXY protocol
- Auto-reconfigures every minute when metadata changes

## Quick Start

### Basic HTTP Load Balancer
```bash
triton instance create \
  -n my-lb \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=http://80:web.svc.ACCOUNT_UUID.DATACENTER.cns.mnx.io:8080" \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

Replace:
- `ACCOUNT_UUID` with your account UUID (`triton account get`)
- `DATACENTER` with your datacenter name (`triton datacenters`)

## Metadata Configuration Keys

| Key | Required | Description |
|-----|----------|-------------|
| `cloud.tritoncompute:loadbalancer` | Yes | Set to `true` to enable |
| `cloud.tritoncompute:portmap` | Yes | Service definitions (see syntax below) |
| `cloud.tritoncompute:certificate_name` | No | Domain names for TLS certificates |
| `cloud.tritoncompute:max_rs` | No | Max backends per service (default: 32, max: 1024) |
| `cloud.tritoncompute:metrics_acl` | No | IP prefixes allowed to access metrics |
| `cloud.tritoncompute:metrics_port` | No | Metrics port (default: 8405) |
| `cloud.tritoncompute:syslog` | No | Remote syslog destination (HOST:PORT) |

## Portmap Syntax

```
<type>://<listen_port>:<backend_name>:<backend_port>{health_check}
```

### Service Types

| Type | Description |
|------|-------------|
| `http` | Layer-7 HTTP (adds X-Forwarded-For) |
| `https` | Layer-7 HTTPS with backend cert verification |
| `https+insecure` | Layer-7 HTTPS without backend cert verification |
| `https-http` | HTTPS frontend, HTTP backend (TLS termination) |
| `tcp` | Layer-4 TCP passthrough |
| `tcp-proxy-v2` | Layer-4 TCP with PROXY protocol v2 |

### Health Check Parameters

```
{check:/endpoint,port:9000,rise:2,fall:1}
```

| Parameter | Description |
|-----------|-------------|
| `check` | HTTP health check endpoint path |
| `port` | Override health check port |
| `rise` | Consecutive successes to mark healthy (default: 2) |
| `fall` | Consecutive failures to mark unhealthy (default: 1) |

## Configuration Examples

### HTTP with Health Check
```bash
triton instance create \
  -n lb-01 \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=http://80:web.svc.ACCOUNT.DC.cns.mnx.io:8080{check:/healthz}" \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

### HTTPS with TLS Termination (Let's Encrypt)
```bash
triton instance create \
  -n lb-01 \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=https-http://443:web.svc.ACCOUNT.DC.cns.mnx.io:8080{check:/healthz}" \
  -m cloud.tritoncompute:certificate_name=example.com,www.example.com \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

**Note:** For Let's Encrypt, ensure DNS points to the load balancer before deployment.

### Multiple Services
```bash
triton instance create \
  -n lb-01 \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=https-http://443:web.svc.ACCOUNT.DC.cns.mnx.io:8080{check:/healthz},http://80:web.svc.ACCOUNT.DC.cns.mnx.io:8080" \
  -m cloud.tritoncompute:certificate_name=example.com \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

### TCP Load Balancing (Database)
```bash
triton instance create \
  -n db-lb-01 \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=tcp://5432:postgres.svc.ACCOUNT.DC.cns.mnx.io:5432" \
  -N private-network \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

### With Metrics and Logging
```bash
triton instance create \
  -n lb-01 \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=http://80:web.svc.ACCOUNT.DC.cns.mnx.io:8080{check:/healthz}" \
  -m "cloud.tritoncompute:metrics_acl=10.0.0.0/8 172.16.0.0/12" \
  -m cloud.tritoncompute:metrics_port=9090 \
  -m cloud.tritoncompute:syslog=logs.internal.company.net:514 \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

## CNS Integration

Triton-Moirai discovers backends via CNS DNS queries:

1. **Backend instances** register with CNS using `triton.cns.services` tag:
   ```bash
   triton instance tag set web-01 triton.cns.services=web:8080
   triton instance tag set web-02 triton.cns.services=web:8080
   ```

2. **Load balancer** portmap references the CNS service FQDN:
   ```
   http://80:web.svc.ACCOUNT.DC.cns.mnx.io:8080
   ```

3. **Auto-discovery**: When you add/remove backend instances, CNS updates DNS and Moirai automatically picks up changes.

### DNS Resolution Modes

**A Record (with explicit port):**
```
http://80:web.svc.ACCOUNT.DC.cns.mnx.io:8080
```
- DNS A lookup for IP addresses
- Uses port 8080 for all backends

**SRV Record (auto port):**
```
http://80:web.svc.ACCOUNT.DC.cns.mnx.io
```
- DNS SRV lookup includes port information
- Backends can have different ports

## Scaling Backends

### Add a Backend
```bash
# Create new instance with same CNS service tag
triton instance create \
  -n web-03 \
  -t triton.cns.services=web:8080 \
  --firewall \
  -w \
  base-64-lts g4-highcpu-1G

# Wait for DNS propagation (30-60 seconds)
# Moirai auto-discovers new backend
```

### Remove a Backend (Graceful)
```bash
# 1. Remove from CNS
triton instance metadata set web-03 triton.cns.status=down

# 2. Wait for DNS propagation and connection draining
sleep 60

# 3. Delete instance
triton instance delete -w web-03
```

### Increase Backend Limit
Default is 32 backends per service. To increase:

```bash
triton instance metadata set lb-01 cloud.tritoncompute:max_rs=100
```

## High Availability

For load balancer redundancy, deploy multiple LB instances:

```bash
# LB 1
triton instance create \
  -n lb-01 \
  -t triton.cns.services=lb:443 \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=https-http://443:web.svc.ACCOUNT.DC.cns.mnx.io:8080" \
  -m cloud.tritoncompute:certificate_name=example.com \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G

# LB 2
triton instance create \
  -n lb-02 \
  -t triton.cns.services=lb:443 \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=https-http://443:web.svc.ACCOUNT.DC.cns.mnx.io:8080" \
  -m cloud.tritoncompute:certificate_name=example.com \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

Then configure external DNS or use a floating IP for failover.

## Updating Configuration

Metadata changes are picked up automatically within ~1 minute:

```bash
# Update portmap
triton instance metadata set lb-01 \
  "cloud.tritoncompute:portmap=http://80:web.svc.ACCOUNT.DC.cns.mnx.io:8080{check:/healthz,rise:3,fall:2}"

# Add certificate
triton instance metadata set lb-01 \
  cloud.tritoncompute:certificate_name=newdomain.com
```

## Monitoring

### Access Metrics
If `metrics_acl` is configured, access Prometheus metrics:

```bash
curl http://LB_IP:METRICS_PORT/metrics
```

### Check HAProxy Stats
SSH to the load balancer and check HAProxy status:

```bash
triton ssh lb-01
svcs -x haproxy
cat /var/svc/log/haproxy:default.log
```

## Troubleshooting

### LB Not Forwarding Traffic
1. Verify `cloud.tritoncompute:loadbalancer=true` is set
2. Check portmap syntax is correct
3. Verify CNS service name resolves: `dig SERVICE.svc.ACCOUNT.DC.cns.mnx.io`
4. Check firewall rules allow traffic
5. **HAProxy may not be running** - SSH to LB and run `svcadm restart haproxy`

### Backends Not Discovered
1. Verify backend instances have `triton.cns.services` tag
2. Check backends are running
3. Wait 60+ seconds for DNS propagation
4. Test DNS resolution from LB instance

### TLS Certificate Issues
1. Ensure DNS points to LB before requesting Let's Encrypt cert
2. Check certificate_name matches your domain exactly
3. SSH to LB and check `/opt/triton/dehydrated/` for logs

### Let's Encrypt Certificate Not Being Used (Self-Signed Instead)

**Symptom:** HTTPS works but browser shows self-signed certificate warning, even though Let's Encrypt certificate was issued.

**Cause:** The HAProxy config uses `/opt/triton/tls/default/fullchain.pem`, but the `default` symlink points to the self-signed certificate directory instead of the Let's Encrypt certificate.

**Fix:**
```bash
LB_IP=<your-load-balancer-ip>
DOMAIN=<your-certificate-domain>  # e.g., www.svc.account.dc.parlercloud.net

ssh root@$LB_IP "
  # Verify Let's Encrypt cert exists
  ls -la /opt/triton/tls/\$DOMAIN/

  # Update symlink to use Let's Encrypt cert
  rm -f /opt/triton/tls/default
  ln -s /opt/triton/tls/${DOMAIN} /opt/triton/tls/default

  # Restart HAProxy to load new cert
  svcadm restart haproxy
"

# Verify certificate is now correct
curl -v https://$DOMAIN/ 2>&1 | grep -E "subject:|issuer:"
# Should show: issuer: C=US; O=Let's Encrypt; CN=...
```

**Prevention:** After creating a load balancer with `certificate_name`, wait 1-2 minutes for the certificate to be issued, then run the symlink fix.

### HAProxy Not Listening After Metadata Update

After updating metadata (portmap, certificate_name), HAProxy may not automatically reload:

```bash
ssh root@$LB_IP "
  # Trigger reconfiguration
  /opt/triton/clb/reconfigure

  # If that doesn't work, restart HAProxy
  svcadm restart haproxy

  # Verify listening
  netstat -an | grep LISTEN | grep -E ':80|:443'
"
```

## Best Practices

1. **Always use health checks** - Ensures only healthy backends receive traffic
2. **Use `https-http` for TLS termination** - Simplifies backend configuration
3. **Configure metrics_acl** - Monitor LB health via Prometheus
4. **Set appropriate rise/fall** - Balance between quick failure detection and stability
5. **Use CNS status for graceful removal** - Never delete backends without draining
6. **Deploy multiple LBs for HA** - Single LB is a single point of failure
