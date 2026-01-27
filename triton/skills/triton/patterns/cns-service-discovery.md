# CNS Service Discovery Patterns

Triton CNS (Container Name Service) provides DNS-based service discovery for your infrastructure. Instances register themselves via tags, and CNS automatically generates DNS records.

## Prerequisites

### Enable CNS on Your Account
```bash
triton account update triton_cns_enabled=true
```

### Get Your Account UUID
```bash
triton account get
# Note the "id" field - you'll need this for DNS names
```

## Service Registration

### Basic Service Tag
Register an instance as part of a service:

```bash
triton instance tag set myinstance triton.cns.services=web
```

### Service with Port (SRV Records)
Include port for SRV record generation:

```bash
triton instance tag set myinstance triton.cns.services=web:8080
```

### Multiple Services
An instance can belong to multiple services:

```bash
triton instance tag set myinstance triton.cns.services=web:8080,api:3000,metrics:9090
```

### Priority and Weight (for Load Balancing)
Control SRV record priority and weight:

```bash
triton instance tag set myinstance triton.cns.services=web:8080:priority=10:weight=50
```

- **priority**: Lower values = higher priority (default: 0)
- **weight**: Higher values = more traffic (default: 0)

## DNS Naming Patterns

### Important: Two DNS Zones (Public vs Private)

Triton CNS provides **two different DNS zones** that resolve to different IPs:

| Zone Type | Domain Pattern | Resolves To | Use Case |
|-----------|----------------|-------------|----------|
| **Private** | `*.cns.<provider>.zone` | Fabric/Private IPs | Internal service communication |
| **Public** | `*.<provider>.net` | Public IPs | External access, Let's Encrypt |

**Example (Parler Cloud):**
```bash
# Private zone - returns fabric IP (192.168.x.x)
dig web.svc.ACCOUNT.us-central-1a.cns.parlercloud.zone
# Returns: 192.168.128.50

# Public zone - returns public IP (142.x.x.x)
dig web.svc.ACCOUNT.us-central-1a.parlercloud.net
# Returns: 142.147.4.50
```

**When to use each:**
- **Private zone (`.cns.*.zone`)**: Load balancer backends, internal services, database connections
- **Public zone (`.*.net`)**: Let's Encrypt certificates, external DNS, public-facing services

> **WARNING:** Using the private zone for Let's Encrypt `certificate_name` will fail because Let's Encrypt cannot reach private IPs for domain validation.

### Instance DNS Name
Each instance gets DNS names in both zones:

**Private zone:**
```
<instance-name>.inst.<account-uuid>.<datacenter>.cns.<provider>.zone
```

**Public zone:**
```
<instance-name>.inst.<account-uuid>.<datacenter>.<provider>.net
```

Example:
```
# Private (fabric IP)
web-01.inst.a1b2c3d4-e5f6-7890-abcd-ef1234567890.us-central-1.cns.parlercloud.zone

# Public (public IP)
web-01.inst.a1b2c3d4-e5f6-7890-abcd-ef1234567890.us-central-1.parlercloud.net
```

### Service DNS Name
Services are accessible via both zones:

**Private zone:**
```
<service-name>.svc.<account-uuid>.<datacenter>.cns.<provider>.zone
```

**Public zone:**
```
<service-name>.svc.<account-uuid>.<datacenter>.<provider>.net
```

Example:
```
# Private (fabric IPs of all instances in service)
web.svc.a1b2c3d4-e5f6-7890-abcd-ef1234567890.us-central-1.cns.parlercloud.zone

# Public (public IPs of all instances in service)
web.svc.a1b2c3d4-e5f6-7890-abcd-ef1234567890.us-central-1.parlercloud.net
```

### SRV Records
For services with ports, SRV records are created:

```
_<service>._tcp.svc.<account-uuid>.<datacenter>.cns.mnx.io
```

Example:
```
_web._tcp.svc.a1b2c3d4-e5f6-7890-abcd-ef1234567890.us-central-1.cns.mnx.io
```

## DNS Queries

### Query A Record (IP addresses)
```bash
dig web.svc.ACCOUNT_UUID.us-central-1.cns.mnx.io A +short
```

Returns all IP addresses for instances in the "web" service.

### Query SRV Record (with ports)
```bash
dig _web._tcp.svc.ACCOUNT_UUID.us-central-1.cns.mnx.io SRV +short
```

Returns priority, weight, port, and hostname for each instance.

## Service Status Control

### Temporarily Remove from DNS
For maintenance, remove an instance from service discovery without stopping it:

```bash
triton instance metadata set myinstance triton.cns.status=down
```

The instance remains running but is removed from service DNS records (within 30-60 seconds).

### Restore to DNS
```bash
triton instance metadata set myinstance triton.cns.status=up
```

Or delete the metadata key:
```bash
triton instance metadata delete myinstance triton.cns.status
```

### Completely Disable CNS for an Instance
To permanently exclude an instance from all CNS records:

```bash
triton instance tag set myinstance triton.cns.disable=true
```

## Graceful Scaling Patterns

### Adding Instances
New instances with the same service tag are automatically added to DNS:

```bash
# Create new instance with existing service tag
triton instance create \
  -n web-03 \
  -t triton.cns.services=web:8080 \
  -t env=production \
  -w \
  base-64-lts g4-highcpu-1G

# Wait for DNS propagation (30-60 seconds)
sleep 60

# Verify DNS includes new instance
dig web.svc.ACCOUNT_UUID.DATACENTER.cns.mnx.io A +short
```

### Removing Instances (Graceful)
1. Mark instance as down in CNS:
   ```bash
   triton instance metadata set web-03 triton.cns.status=down
   ```

2. Wait for DNS propagation and connection draining:
   ```bash
   sleep 60
   ```

3. Delete the instance:
   ```bash
   triton instance delete -w web-03
   ```

## Multi-Tier Application Example

### Web Tier
```bash
triton instance create \
  -n web-01 \
  -t triton.cns.services=myapp-web:8080 \
  -t app=myapp \
  -t role=web \
  -t env=production \
  --firewall \
  -w \
  base-64-lts g4-highcpu-1G
```

### API Tier
```bash
triton instance create \
  -n api-01 \
  -t triton.cns.services=myapp-api:3000 \
  -t app=myapp \
  -t role=api \
  -t env=production \
  --firewall \
  -w \
  base-64-lts g4-highcpu-2G
```

### Database Tier (Private Network)
```bash
triton instance create \
  -n db-01 \
  -t triton.cns.services=myapp-db:5432 \
  -t app=myapp \
  -t role=db \
  -t env=production \
  -N private-network \
  --firewall \
  -w \
  base-64-lts g4-highmem-4G
```

### Application Configuration
In your web tier, configure API endpoint:
```
API_URL=http://myapp-api.svc.ACCOUNT_UUID.DATACENTER.cns.mnx.io:3000
```

In your API tier, configure database:
```
DATABASE_URL=postgres://user:pass@myapp-db.svc.ACCOUNT_UUID.DATACENTER.cns.mnx.io:5432/mydb
```

## Best Practices

1. **Consistent Naming**: Use `<app>-<tier>` pattern for service names
   - `myapp-web`, `myapp-api`, `myapp-db`

2. **Always Include Ports**: Use port syntax for SRV record generation
   - `triton.cns.services=web:8080` not `triton.cns.services=web`

3. **Use Status for Maintenance**: Set `triton.cns.status=down` before maintenance
   - Never just delete instances without graceful removal

4. **Private Services**: Put internal services (databases, caches) on private networks
   - They'll still get CNS names resolvable within your network

5. **Multiple Tags**: Use additional tags for organization and firewall rules
   - `app=myapp`, `env=production`, `role=web`

6. **TTL Awareness**: DNS TTL is 30-60 seconds
   - Wait at least 60 seconds after CNS changes before assuming propagation

## Troubleshooting

### Check if CNS is Enabled
```bash
triton account get | grep triton_cns_enabled
```

### Verify Instance Tags
```bash
triton instance tag list myinstance
```

### Test DNS Resolution
```bash
# From within Triton network
dig myservice.svc.ACCOUNT_UUID.DATACENTER.cns.mnx.io

# Check what IPs are registered
dig myservice.svc.ACCOUNT_UUID.DATACENTER.cns.mnx.io A +short
```

### Check Instance in CNS
An instance appears in CNS if:
- Account has `triton_cns_enabled=true`
- Instance is running
- Instance has `triton.cns.services` tag
- Instance does NOT have `triton.cns.disable=true`
- Instance does NOT have `triton.cns.status=down`
