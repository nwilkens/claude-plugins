# Load Balancing with Triton-Moirai

Triton-Moirai is a metadata-driven HAProxy-based load balancer for Triton. Configuration is done entirely through instance metadata keys -- no manual HAProxy configuration is needed. All provisioning and management is performed through the CloudAPI REST interface.

## Overview

Triton-Moirai:
- Runs as a tenant-managed instance using the `cloud-load-balancer` image
- Configured entirely via instance metadata (`cloud.tritoncompute:*` keys)
- Integrates with CNS for automatic backend discovery via DNS
- Supports HTTP, HTTPS (with Let's Encrypt), TCP, and PROXY protocol
- Auto-reconfigures approximately every 1 minute when metadata or DNS changes

## Quick Start: Basic HTTP Load Balancer

```
POST /:login/machines
```

```json
{
  "name": "my-lb",
  "image": "<cloud-load-balancer-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<external-network-uuid>", "<app-network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "lb",
  "metadata.cloud.tritoncompute:loadbalancer": "true",
  "metadata.cloud.tritoncompute:portmap": "http://80:web.svc.ACCOUNT_UUID.DATACENTER.cns.PROVIDER.zone:8080"
}
```

Replace `ACCOUNT_UUID`, `DATACENTER`, and `PROVIDER` with your actual values. Discover these from `GET /:login` (account UUID) and `GET /:login/datacenters` (datacenter name). The provider zone suffix can be discovered from instance `dns_names`.

## Metadata Configuration Keys

All configuration is passed as instance metadata. Set these in the `metadata.*` fields when creating the instance or update them later via the metadata API.

| Key | Required | Description |
|-----|----------|-------------|
| `cloud.tritoncompute:loadbalancer` | Yes | Set to `true` to enable load balancing |
| `cloud.tritoncompute:portmap` | Yes | Service definitions (see portmap syntax below) |
| `cloud.tritoncompute:certificate_name` | No | Comma-separated domain names for TLS certificates (Let's Encrypt) |
| `cloud.tritoncompute:max_rs` | No | Max backends per service (default: 32, max: 1024) |
| `cloud.tritoncompute:metrics_acl` | No | Space-separated IP prefixes allowed to access Prometheus metrics |
| `cloud.tritoncompute:metrics_port` | No | Port for Prometheus metrics endpoint (default: 8405) |
| `cloud.tritoncompute:syslog` | No | Remote syslog destination (`HOST:PORT`) |

## Portmap Syntax

The portmap defines how the load balancer listens and where it forwards traffic.

```
<type>://<listen_port>:<backend_dns>:<backend_port>{health_check_params}
```

Multiple services are separated by commas:

```
https-http://443:web.svc.ACCT.DC.cns.PROVIDER.zone:8080{check:/healthz},http://80:web.svc.ACCT.DC.cns.PROVIDER.zone:8080
```

### Service Types

| Type | Description |
|------|-------------|
| `http` | Layer-7 HTTP (adds X-Forwarded-For header) |
| `https` | Layer-7 HTTPS with backend certificate verification |
| `https+insecure` | Layer-7 HTTPS without backend certificate verification |
| `https-http` | HTTPS frontend, HTTP backend (TLS termination at LB) |
| `tcp` | Layer-4 TCP passthrough |
| `tcp-proxy-v2` | Layer-4 TCP with PROXY protocol v2 |

### Health Check Parameters

Append health check configuration inside curly braces:

```
{check:/healthz,port:9000,rise:2,fall:1}
```

| Parameter | Description |
|-----------|-------------|
| `check` | HTTP health check endpoint path |
| `port` | Override health check port (defaults to backend port) |
| `rise` | Consecutive successes required to mark a backend healthy (default: 2) |
| `fall` | Consecutive failures required to mark a backend unhealthy (default: 1) |

### DNS Resolution Modes

**A record with explicit port** (recommended):
```
http://80:web.svc.ACCT.DC.cns.PROVIDER.zone:8080
```
DNS A lookup returns IP addresses. Port 8080 is used for all backends.

**SRV record (auto port)**:
```
http://80:web.svc.ACCT.DC.cns.PROVIDER.zone
```
DNS SRV lookup returns both IP and port information. Backends can use different ports.

## Configuration Examples

### HTTP with Health Check

```
POST /:login/machines
```

```json
{
  "name": "lb-01",
  "image": "<cloud-load-balancer-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<external-network-uuid>", "<app-network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "lb",
  "metadata.cloud.tritoncompute:loadbalancer": "true",
  "metadata.cloud.tritoncompute:portmap": "http://80:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080{check:/healthz}"
}
```

### HTTPS with TLS Termination (Let's Encrypt)

```
POST /:login/machines
```

```json
{
  "name": "lb-01",
  "image": "<cloud-load-balancer-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<external-network-uuid>", "<app-network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "lb",
  "metadata.cloud.tritoncompute:loadbalancer": "true",
  "metadata.cloud.tritoncompute:portmap": "https-http://443:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080{check:/healthz}",
  "metadata.cloud.tritoncompute:certificate_name": "example.com,www.example.com"
}
```

**IMPORTANT**: The `certificate_name` domain must resolve to the load balancer's public IP before creation so Let's Encrypt can validate the domain. Point your external DNS to the LB first.

### HTTP + HTTPS (Redirect Pattern)

Serve both HTTP and HTTPS. Commonly used when the HTTP listener redirects to HTTPS or serves the Let's Encrypt ACME challenge.

```json
{
  "metadata.cloud.tritoncompute:portmap": "https-http://443:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080{check:/healthz},http://80:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080"
}
```

### TCP Load Balancing (Database)

For non-HTTP protocols, use TCP passthrough:

```
POST /:login/machines
```

```json
{
  "name": "db-lb-01",
  "image": "<cloud-load-balancer-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<db-network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "lb",
  "metadata.cloud.tritoncompute:loadbalancer": "true",
  "metadata.cloud.tritoncompute:portmap": "tcp://5432:postgres.svc.ACCOUNT.DC.cns.PROVIDER.zone:5432"
}
```

Note: This LB is on the private network only (no external network) since the database tier should not be publicly accessible.

### With Metrics and Logging

```
POST /:login/machines
```

```json
{
  "name": "lb-01",
  "image": "<cloud-load-balancer-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<external-network-uuid>", "<app-network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "lb",
  "metadata.cloud.tritoncompute:loadbalancer": "true",
  "metadata.cloud.tritoncompute:portmap": "https-http://443:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080{check:/healthz}",
  "metadata.cloud.tritoncompute:certificate_name": "example.com",
  "metadata.cloud.tritoncompute:metrics_acl": "10.0.0.0/8 172.16.0.0/12",
  "metadata.cloud.tritoncompute:metrics_port": "9090",
  "metadata.cloud.tritoncompute:syslog": "logs.internal.company.net:514"
}
```

## CNS Integration

Triton-Moirai discovers backends automatically through CNS DNS queries. This is the core mechanism for dynamic backend management.

### How It Works

1. **Backend instances** register with CNS via the `triton.cns.services` tag:

   ```
   POST /:login/machines
   ```
   ```json
   {
     "name": "web-01",
     "image": "<image-uuid>",
     "package": "<package-uuid>",
     "networks": ["<app-network-uuid>"],
     "tag.triton.cns.services": "web:8080",
     "tag.role": "web"
   }
   ```

2. **Load balancer** portmap references the CNS service FQDN (using the **private zone**):
   ```
   http://80:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080
   ```

3. **Auto-discovery**: When you add or remove backend instances, CNS updates DNS. Moirai resolves the DNS periodically (~1 minute) and reconfigures HAProxy automatically.

### Use Private Zone for Backends

Always use the **private zone** (`.cns.<provider>.zone`) for the backend DNS name in the portmap. This ensures:
- Traffic between the LB and backends stays on the fabric network
- No public IP is required on backend instances
- Lower latency and no bandwidth charges

## Scaling Backends

### Adding a Backend

Create a new instance with the same `triton.cns.services` tag:

```
POST /:login/machines
```

```json
{
  "name": "web-03",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<app-network-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "web:8080",
  "tag.role": "web"
}
```

After the instance reaches `running` state, wait 30-60 seconds for DNS propagation. Moirai will pick up the new backend within its next reconfiguration cycle (~1 minute).

### Removing a Backend (Graceful)

**Step 1: Remove from CNS**

```
PUT /:login/machines/:machine_id/metadata/triton.cns.status
```

```json
"down"
```

**Step 2: Wait for DNS propagation and connection draining (60 seconds minimum)**

**Step 3: Delete the instance**

```
DELETE /:login/machines/:machine_id
```

### Increasing the Backend Limit

The default maximum is 32 backends per service. To increase:

```
PUT /:login/machines/:lb_machine_id/metadata/cloud.tritoncompute:max_rs
```

```json
"100"
```

## High Availability

For load balancer redundancy, deploy multiple LB instances with identical configuration:

**LB 1:**

```
POST /:login/machines
```

```json
{
  "name": "lb-01",
  "image": "<cloud-load-balancer-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<external-network-uuid>", "<app-network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "lb",
  "tag.triton.cns.services": "lb:443",
  "metadata.cloud.tritoncompute:loadbalancer": "true",
  "metadata.cloud.tritoncompute:portmap": "https-http://443:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080{check:/healthz}",
  "metadata.cloud.tritoncompute:certificate_name": "example.com"
}
```

**LB 2:**

```
POST /:login/machines
```

```json
{
  "name": "lb-02",
  "image": "<cloud-load-balancer-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<external-network-uuid>", "<app-network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "lb",
  "tag.triton.cns.services": "lb:443",
  "metadata.cloud.tritoncompute:loadbalancer": "true",
  "metadata.cloud.tritoncompute:portmap": "https-http://443:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080{check:/healthz}",
  "metadata.cloud.tritoncompute:certificate_name": "example.com"
}
```

Then configure external DNS with round-robin A records pointing to both LB public IPs, or use a floating IP for automatic failover.

## Updating Configuration

Metadata changes are picked up automatically within approximately 1 minute.

### Update Portmap

```
PUT /:login/machines/:lb_machine_id/metadata/cloud.tritoncompute:portmap
```

```json
"https-http://443:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080{check:/healthz,rise:3,fall:2}"
```

### Add Certificate

```
PUT /:login/machines/:lb_machine_id/metadata/cloud.tritoncompute:certificate_name
```

```json
"newdomain.com"
```

### Replace All Metadata

To update multiple metadata keys at once:

```
POST /:login/machines/:lb_machine_id/metadata
```

```json
{
  "cloud.tritoncompute:portmap": "https-http://443:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080{check:/healthz}",
  "cloud.tritoncompute:certificate_name": "example.com,www.example.com",
  "cloud.tritoncompute:metrics_acl": "10.0.0.0/8"
}
```

## Monitoring

### Prometheus Metrics

If `metrics_acl` is configured, access Prometheus-format metrics:

```
GET http://<lb-ip>:<metrics-port>/metrics
```

Default metrics port is 8405. Change it with the `cloud.tritoncompute:metrics_port` metadata key.

### HAProxy Stats (via SSH)

SSH to the load balancer to inspect HAProxy directly:

```bash
ssh root@<lb-ip>
svcs -x haproxy
cat /var/svc/log/haproxy:default.log
```

## Known Gotchas

### 1. Let's Encrypt Certificate Defaults to Self-Signed

**Symptom**: HTTPS works but the browser shows a self-signed certificate warning even though Let's Encrypt issued a certificate.

**Cause**: HAProxy uses `/opt/triton/tls/default/fullchain.pem`, but the `default` symlink points to the self-signed certificate directory instead of the Let's Encrypt certificate.

**Fix** (SSH to the LB):
```bash
DOMAIN=<your-certificate-domain>  # e.g., example.com

# Verify Let's Encrypt cert exists
ls -la /opt/triton/tls/$DOMAIN/

# Update symlink to use Let's Encrypt cert
rm -f /opt/triton/tls/default
ln -s /opt/triton/tls/$DOMAIN /opt/triton/tls/default

# Restart HAProxy to load the new cert
svcadm restart haproxy
```

Verify the fix:
```bash
curl -v https://$DOMAIN/ 2>&1 | grep -E "subject:|issuer:"
# Should show: issuer: C=US; O=Let's Encrypt; CN=...
```

**Prevention**: After creating a load balancer with `certificate_name`, wait 1-2 minutes for the certificate to be issued, then run the symlink fix.

### 2. HAProxy May Need Manual Restart After Metadata Changes

**Symptom**: You updated portmap or certificate_name via the metadata API but the load balancer is not reflecting the changes.

**Cause**: The auto-reconfiguration runs approximately every 1 minute. In some cases, HAProxy does not automatically reload.

**Fix** (SSH to the LB):
```bash
# Trigger reconfiguration manually
/opt/triton/clb/reconfigure

# If that does not work, restart HAProxy
svcadm restart haproxy

# Verify it is listening on the expected ports
netstat -an | grep LISTEN | grep -E ':80|:443'
```

### 3. Auto-Reconfiguration Cycle Is ~1 Minute

Backend changes (via CNS DNS) and metadata changes are not instant. The Moirai agent polls every ~1 minute. During this window, new backends will not receive traffic and removed backends may still receive requests.

Plan for this delay when scripting deployments. After adding or removing backends, wait at least 90 seconds (60s DNS TTL + potential reconfiguration delay) before verifying.

## Complete Example: Web Application with TLS

This example creates a full stack: two web backends behind a load balancer with Let's Encrypt TLS.

**Step 1: Create the app network** (see [networking.md](networking.md))

```
POST /:login/fabrics/default/vlans/100/networks
```

```json
{
  "name": "app-net",
  "subnet": "10.100.0.0/24",
  "provision_start_ip": "10.100.0.10",
  "provision_end_ip": "10.100.0.250",
  "gateway": "10.100.0.1",
  "internet_nat": true
}
```

**Step 2: Create backend instances with CNS tags**

```
POST /:login/machines
```

```json
{
  "name": "web-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<app-net-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "web:8080",
  "tag.role": "web"
}
```

```
POST /:login/machines
```

```json
{
  "name": "web-02",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<app-net-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "web:8080",
  "tag.role": "web"
}
```

**Step 3: Point DNS to the LB's public IP** (external DNS configuration)

**Step 4: Create the load balancer**

```
POST /:login/machines
```

```json
{
  "name": "lb-01",
  "image": "<cloud-load-balancer-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<external-network-uuid>", "<app-net-uuid>"],
  "firewall_enabled": true,
  "tag.role": "lb",
  "metadata.cloud.tritoncompute:loadbalancer": "true",
  "metadata.cloud.tritoncompute:portmap": "https-http://443:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080{check:/healthz},http://80:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080",
  "metadata.cloud.tritoncompute:certificate_name": "example.com"
}
```

**Step 5: Wait and fix certificate symlink** (see Gotcha #1 above)

**Step 6: Create firewall rules**

```
POST /:login/fwrules
```

```json
{
  "rule": "FROM any TO tag role = lb ALLOW tcp (PORT 80 AND PORT 443)",
  "enabled": true,
  "description": "Allow HTTP/HTTPS to load balancer"
}
```

```
POST /:login/fwrules
```

```json
{
  "rule": "FROM tag role = lb TO tag role = web ALLOW tcp PORT 8080",
  "enabled": true,
  "description": "Allow LB to reach web backends"
}
```

## Best Practices

1. **Always use health checks** -- Ensures only healthy backends receive traffic. Use `{check:/healthz}` or your application's health endpoint.

2. **Use `https-http` for TLS termination** -- Terminate TLS at the load balancer and communicate with backends over plain HTTP on the fabric network. This simplifies backend configuration.

3. **Use the private zone for backend DNS** -- The portmap `backend_dns` should always use the private zone (`.cns.<provider>.zone`) so traffic stays on the fabric network.

4. **Configure `metrics_acl`** -- Enable Prometheus metrics for monitoring. Restrict to your monitoring network's CIDR.

5. **Set appropriate `rise`/`fall` thresholds** -- Balance between quick failure detection (`fall:1`) and stability during transient issues (`fall:3`). Start with `rise:2,fall:1`.

6. **Use CNS status for graceful removal** -- Always set `triton.cns.status=down` and wait before deleting backend instances.

7. **Deploy multiple LBs for HA** -- A single load balancer is a single point of failure. Deploy at least two with identical configuration.

8. **Plan for the reconfiguration delay** -- Changes take up to ~90 seconds to take effect (DNS TTL + Moirai poll interval). Account for this in deployment scripts.

9. **Check the certificate symlink** -- After deploying a new LB with Let's Encrypt, verify the TLS certificate is correct and fix the `default` symlink if needed.

10. **Put the LB on both public and app networks** -- The LB needs a public network for incoming traffic and the app fabric network to reach backends via the private DNS zone.
