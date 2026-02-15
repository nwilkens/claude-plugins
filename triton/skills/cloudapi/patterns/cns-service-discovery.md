# CNS Service Discovery Patterns

Triton CNS (Container Name Service) provides DNS-based service discovery for your infrastructure. Instances register themselves via tags, and CNS automatically generates DNS records. All registration is done through the CloudAPI REST interface.

## How CNS Works

1. You set a `triton.cns.services` tag on an instance (at creation time or via tag update)
2. CNS watches for tag changes and generates DNS records within 30-60 seconds
3. Other instances (and load balancers) resolve the service DNS name to discover backends
4. When instances are added or removed, DNS records update automatically

No manual DNS configuration is required. Tags drive everything.

## Prerequisites

### Enable CNS on Your Account

```
POST /:login
```

```json
{
  "triton_cns_enabled": true
}
```

### Get Your Account UUID

```
GET /:login
```

Note the `id` field in the response -- this UUID appears in all CNS DNS names.

## Service Registration

### Register at Instance Creation

Set the `triton.cns.services` tag when creating an instance. Use the `tag.` prefix in the `CreateMachine` request body.

```
POST /:login/machines
```

```json
{
  "name": "web-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "web:8080",
  "tag.role": "web",
  "tag.env": "production"
}
```

### Register via Tag Update

Add or change service registration on an existing instance.

```
PUT /:login/machines/:machine_id/tags/triton.cns.services
```

```json
"web:8080"
```

Or set multiple tags at once:

```
POST /:login/machines/:machine_id/tags
```

```json
{
  "triton.cns.services": "web:8080",
  "role": "web"
}
```

### Multiple Services

An instance can belong to multiple services. Separate entries with commas.

```
POST /:login/machines
```

```json
{
  "name": "app-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "tag.triton.cns.services": "web:443,api:8080,metrics:9090"
}
```

This creates DNS records for three services, each with its own port.

### Priority and Weight (SRV Records)

Control traffic distribution across instances using SRV record priority and weight:

```json
{
  "tag.triton.cns.services": "web:8080:priority=10:weight=50"
}
```

- **priority**: Lower values are preferred (default: 0)
- **weight**: Higher values receive more traffic among same-priority instances (default: 0)

## Two DNS Zones: Public vs Private

CNS provides **two separate DNS zones** that resolve to different IP addresses. This is a critical concept for correct configuration.

| Zone Type | Domain Pattern | Resolves To | Use Case |
|-----------|----------------|-------------|----------|
| **Private** | `<svc>.svc.<account-uuid>.<dc>.cns.<provider>.zone` | Fabric/private IPs | Internal service communication |
| **Public** | `<svc>.svc.<account-uuid>.<dc>.<provider>.net` | Public IPs | External access, Let's Encrypt |

### Example

For a `web` service in the `us-central-1a` datacenter:

```
# Private zone -- returns fabric IP (e.g., 192.168.128.50)
dig web.svc.ACCOUNT_UUID.us-central-1a.cns.parlercloud.zone A +short

# Public zone -- returns public IP (e.g., 142.147.4.50)
dig web.svc.ACCOUNT_UUID.us-central-1a.parlercloud.net A +short
```

### When to Use Each Zone

**Private zone (`.cns.<provider>.zone`)**:
- Load balancer backend discovery (portmap `backend_dns` value)
- Internal service-to-service communication
- Database connection strings
- Any traffic that should stay on fabric networks

**Public zone (`.<provider>.net`)**:
- Let's Encrypt certificate domain validation
- External DNS CNAME targets
- Public-facing service endpoints

**KEY INSIGHT**: Use the private zone for load balancer backends so traffic stays on the fabric network. Use the public zone for `certificate_name` on load balancers so Let's Encrypt can reach the LB for domain validation.

> **WARNING**: Using the private zone for Let's Encrypt `certificate_name` will fail because Let's Encrypt cannot reach private IPs for domain validation.

## DNS Name Formats

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
web-01.inst.a1b2c3d4-e5f6-7890-abcd-ef1234567890.us-central-1a.cns.parlercloud.zone

# Public (public IP)
web-01.inst.a1b2c3d4-e5f6-7890-abcd-ef1234567890.us-central-1a.parlercloud.net
```

### Service DNS Name

Service names aggregate all instances with the matching `triton.cns.services` tag:

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
# Private (fabric IPs of all instances in the "web" service)
web.svc.a1b2c3d4-e5f6-7890-abcd-ef1234567890.us-central-1a.cns.parlercloud.zone

# Public (public IPs of all instances in the "web" service)
web.svc.a1b2c3d4-e5f6-7890-abcd-ef1234567890.us-central-1a.parlercloud.net
```

### SRV Records

For services registered with ports (e.g., `web:8080`), CNS creates SRV records:

```
_<service>._tcp.svc.<account-uuid>.<datacenter>.cns.<provider>.zone
```

SRV records include priority, weight, port, and target hostname. Query them with:

```bash
dig _web._tcp.svc.ACCOUNT_UUID.us-central-1a.cns.parlercloud.zone SRV +short
```

## Discovering DNS Suffixes Programmatically

Rather than hardcoding zone suffixes, discover them from instance or network data.

### From Instance DNS Names

Each instance exposes its DNS names in the response:

```
GET /:login/machines/:machine_id
```

**Response excerpt:**
```json
{
  "id": "b6c73d6a-...",
  "name": "web-01",
  "dns_names": [
    "web-01.inst.ACCOUNT.us-central-1a.parlercloud.net",
    "web-01.inst.ACCOUNT.us-central-1a.cns.parlercloud.zone",
    "web.svc.ACCOUNT.us-central-1a.parlercloud.net",
    "web.svc.ACCOUNT.us-central-1a.cns.parlercloud.zone"
  ]
}
```

### From Network Configuration

Networks include DNS suffix information:

```
GET /:login/networks/:network_id
```

Check the `public` field to determine which zone type applies:
- `"public": true` -- DNS resolves to routable public IPs
- `"public": false` -- DNS resolves to fabric/private IPs

## Service Status Control

### Maintenance Mode: Remove from DNS

Temporarily remove an instance from service discovery without stopping it. Set the `triton.cns.status` metadata key to `down`.

**Via metadata update on existing instance:**

```
PUT /:login/machines/:machine_id/metadata/triton.cns.status
```

```json
"down"
```

The instance remains running but is removed from service DNS records within 30-60 seconds.

### Restore to DNS

Set the status back to `up`:

```
PUT /:login/machines/:machine_id/metadata/triton.cns.status
```

```json
"up"
```

Or delete the metadata key entirely:

```
DELETE /:login/machines/:machine_id/metadata/triton.cns.status
```

### Completely Disable CNS for an Instance

Permanently exclude an instance from all CNS records:

```
POST /:login/machines/:machine_id/tags
```

```json
{
  "triton.cns.disable": "true"
}
```

## Graceful Scaling Patterns

### Scaling Up (Adding Instances)

New instances with the same service tag are automatically added to DNS. Create the instance and wait for propagation.

```
POST /:login/machines
```

```json
{
  "name": "web-03",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "web:8080",
  "tag.role": "web",
  "tag.env": "production"
}
```

After the instance is running, wait 30-60 seconds for DNS propagation. Verify by querying the service DNS name.

### Scaling Down (Removing Instances)

Always remove from DNS before deleting to allow connection draining.

**Step 1: Set maintenance mode (removes from DNS)**

```
PUT /:login/machines/:machine_id/metadata/triton.cns.status
```

```json
"down"
```

**Step 2: Wait for DNS propagation and connection draining (30-60 seconds)**

The DNS TTL is 30-60 seconds. Wait at least 60 seconds after setting `triton.cns.status=down` before proceeding.

**Step 3: Delete the instance**

```
DELETE /:login/machines/:machine_id
```

> **IMPORTANT**: Never skip step 1. Deleting an instance without first removing it from DNS causes a window where clients are routed to a dead backend.

## Multi-Tier Application Example with CNS

This example shows a full three-tier application using CNS for internal service discovery.

### Web Tier

```
POST /:login/machines
```

```json
{
  "name": "web-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<web-net-uuid>", "<api-net-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "myapp-web:8080",
  "tag.app": "myapp",
  "tag.role": "web",
  "tag.env": "production"
}
```

### API Tier

```
POST /:login/machines
```

```json
{
  "name": "api-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<api-net-uuid>", "<db-net-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "myapp-api:3000",
  "tag.app": "myapp",
  "tag.role": "api",
  "tag.env": "production"
}
```

### Database Tier (Private Network Only)

```
POST /:login/machines
```

```json
{
  "name": "db-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<db-net-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "myapp-db:5432",
  "tag.app": "myapp",
  "tag.role": "db",
  "tag.env": "production"
}
```

### Application Configuration Using CNS

In your web tier, configure the API endpoint using the private zone CNS name:

```
API_URL=http://myapp-api.svc.ACCOUNT_UUID.DATACENTER.cns.PROVIDER.zone:3000
```

In your API tier, configure the database connection:

```
DATABASE_URL=postgres://user:pass@myapp-db.svc.ACCOUNT_UUID.DATACENTER.cns.PROVIDER.zone:5432/mydb
```

Both use the **private zone** (`.cns.<provider>.zone`) so traffic stays on fabric networks.

## Best Practices

1. **Consistent naming**: Use `<app>-<tier>` pattern for service names
   - `myapp-web`, `myapp-api`, `myapp-db`

2. **Always include ports**: Use port syntax for SRV record generation
   - `triton.cns.services=web:8080` not `triton.cns.services=web`

3. **Use maintenance mode before removal**: Set `triton.cns.status=down` and wait for TTL before deleting instances

4. **Private zone for internal services**: Database connections and inter-service communication should always use the private zone (`.cns.<provider>.zone`)

5. **Public zone for external services**: Let's Encrypt certificates and public-facing endpoints use the public zone (`.<provider>.net`)

6. **Multiple tags for organization**: Use additional tags for firewall rules and grouping
   - `app=myapp`, `env=production`, `role=web`

7. **TTL awareness**: DNS TTL is 30-60 seconds. Wait at least 60 seconds after CNS changes before assuming propagation is complete.

8. **Discover suffixes from instance data**: Use the `dns_names` array from `GET /:login/machines/:id` instead of hardcoding zone suffixes

## Troubleshooting

### Instance Not Appearing in DNS

1. Verify CNS is enabled on the account:
   ```
   GET /:login
   ```
   Check that `triton_cns_enabled` is `true`.

2. Verify the instance has the correct tag:
   ```
   GET /:login/machines/:machine_id/tags
   ```
   Check that `triton.cns.services` is set.

3. Verify the instance is running:
   ```
   GET /:login/machines/:machine_id
   ```
   Check that `state` is `running`.

4. Verify the instance does not have CNS disabled:
   ```
   GET /:login/machines/:machine_id/tags
   ```
   Check that `triton.cns.disable` is NOT `true`.

5. Check if maintenance mode is active:
   ```
   GET /:login/machines/:machine_id/metadata/triton.cns.status
   ```
   If it returns `down`, the instance is intentionally excluded from DNS.

6. Wait at least 60 seconds -- CNS updates are not instant.

### DNS Resolves to Wrong IP

- Check which zone you are querying. The private zone returns fabric IPs; the public zone returns public IPs.
- If the instance only has a fabric NIC, it will not have a public zone record.
- If the instance only has a public NIC, it will not have a private zone record.
