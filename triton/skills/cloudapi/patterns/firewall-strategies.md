# Firewall Rule Strategies

Triton Cloud Firewall provides tag-based security groups for dynamic, scalable network access control. Rules target instances by **tags** rather than IP addresses, so policies follow machines as they scale up or down.

## Key Concepts

- **Tag-based targeting**: Rules use `tag <key> = <value>` to match instances dynamically
- **Default policy**: All traffic is **allowed** unless firewall is enabled on the instance (`firewall_enabled: true`)
- **Stateful rules**: Allowed connections are stateful -- return traffic is automatically permitted
- **No port ranges**: Individual port rules only. Use `PORT all` for all ports on a protocol. You cannot write `PORT 8000-9000`.
- **Rule evaluation**: BLOCK rules take precedence; ALLOW rules then selectively permit traffic

## Tag-Based Security Groups

Assign role and environment tags when creating instances to group them into logical security tiers.

### Role Tags

| Tag | Purpose | Example Instances |
|-----|---------|-------------------|
| `role=lb` | Load balancers (internet-facing) | HAProxy, nginx LB |
| `role=web` | Web/application servers | nginx, Node.js |
| `role=api` | API/backend servers | REST services |
| `role=db` | Database servers | PostgreSQL, MySQL |
| `role=bastion` | SSH jump host | Bastion/jumpbox |
| `role=monitor` | Monitoring infrastructure | Prometheus, Grafana |

### Environment Tags

| Tag | Purpose |
|-----|---------|
| `env=production` | Production workloads |
| `env=staging` | Staging/pre-production |
| `env=development` | Development instances |

### Tagging Instances at Creation

```
POST /:login/machines
{
  "name": "web-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "web",
  "tag.env": "production",
  "tag.triton.cns.services": "webapp:8080"
}
```

### Adding Tags to Existing Instances

```
POST /:login/machines/:id/tags
{
  "role": "web",
  "env": "production"
}
```

## Multi-Tier Firewall Architecture

A standard 3-tier application with a load balancer, web tier, API tier, and database tier.

```
Internet
    |
    | :443
    v
[ LB (role=lb) ]
    |
    | :8080
    v
[ Web (role=web) ]
    |
    | :3000
    v
[ API (role=api) ]
    |
    | :5432
    v
[ DB (role=db) ]

[ Bastion (role=bastion) ] --SSH:22--> all tiers
```

### Rule Set

#### 1. Allow HTTPS from internet to load balancers

```
POST /:login/fwrules
{
  "rule": "FROM any TO tag \"role\" = \"lb\" ALLOW tcp PORT 443",
  "enabled": true,
  "description": "Allow HTTPS from internet to load balancers"
}
```

#### 2. Allow HTTP from internet to load balancers (redirect to HTTPS)

```
POST /:login/fwrules
{
  "rule": "FROM any TO tag \"role\" = \"lb\" ALLOW tcp PORT 80",
  "enabled": true,
  "description": "Allow HTTP from internet to LB (for HTTPS redirect)"
}
```

#### 3. Allow LB to Web on internal port 8080

```
POST /:login/fwrules
{
  "rule": "FROM tag \"role\" = \"lb\" TO tag \"role\" = \"web\" ALLOW tcp PORT 8080",
  "enabled": true,
  "description": "Allow LB to forward traffic to web tier"
}
```

#### 4. Allow Web to API on internal port 3000

```
POST /:login/fwrules
{
  "rule": "FROM tag \"role\" = \"web\" TO tag \"role\" = \"api\" ALLOW tcp PORT 3000",
  "enabled": true,
  "description": "Allow web tier to reach API tier"
}
```

#### 5. Allow API to DB on internal port 5432

```
POST /:login/fwrules
{
  "rule": "FROM tag \"role\" = \"api\" TO tag \"role\" = \"db\" ALLOW tcp PORT 5432",
  "enabled": true,
  "description": "Allow API tier to reach database"
}
```

#### 6. Allow SSH only from bastion

```
POST /:login/fwrules
{
  "rule": "FROM tag \"role\" = \"bastion\" TO tag \"role\" = \"web\" ALLOW tcp PORT 22",
  "enabled": true,
  "description": "Allow SSH from bastion to web tier"
}
```

```
POST /:login/fwrules
{
  "rule": "FROM tag \"role\" = \"bastion\" TO tag \"role\" = \"api\" ALLOW tcp PORT 22",
  "enabled": true,
  "description": "Allow SSH from bastion to API tier"
}
```

```
POST /:login/fwrules
{
  "rule": "FROM tag \"role\" = \"bastion\" TO tag \"role\" = \"db\" ALLOW tcp PORT 22",
  "enabled": true,
  "description": "Allow SSH from bastion to DB tier"
}
```

```
POST /:login/fwrules
{
  "rule": "FROM tag \"role\" = \"bastion\" TO tag \"role\" = \"lb\" ALLOW tcp PORT 22",
  "enabled": true,
  "description": "Allow SSH from bastion to LB tier"
}
```

#### 7. Allow SSH from a trusted IP to the bastion

```
POST /:login/fwrules
{
  "rule": "FROM ip 203.0.113.50 TO tag \"role\" = \"bastion\" ALLOW tcp PORT 22",
  "enabled": true,
  "description": "Allow SSH to bastion from office IP"
}
```

#### 8. Allow monitoring server to scrape metrics from all tiers

```
POST /:login/fwrules
{
  "rule": "FROM tag \"role\" = \"monitor\" TO tag \"env\" = \"production\" ALLOW tcp PORT 9100",
  "enabled": true,
  "description": "Allow Prometheus node_exporter scraping across production"
}
```

## Default-Deny Approach

Triton firewall starts with **allow-all** when firewall is disabled. Enabling `firewall_enabled: true` on an instance activates rule evaluation. To implement default-deny:

1. **Enable the firewall** on every instance at creation time
2. **Create only the ALLOW rules you need** -- traffic not matching any ALLOW rule is implicitly denied when the firewall is enabled
3. **Use BLOCK rules** for explicit denials that override ALLOW rules

### BLOCK + Selective ALLOW Pattern

Block all traffic to a tag group, then selectively allow specific sources.

#### Block all inbound TCP to the database tier

```
POST /:login/fwrules
{
  "rule": "FROM any TO tag \"role\" = \"db\" BLOCK tcp PORT all",
  "enabled": true,
  "description": "Block all TCP to database tier by default"
}
```

#### Then allow only the API tier to reach the database

```
POST /:login/fwrules
{
  "rule": "FROM tag \"role\" = \"api\" TO tag \"role\" = \"db\" ALLOW tcp PORT 5432",
  "enabled": true,
  "description": "Allow only API tier to reach database on 5432"
}
```

BLOCK rules have higher priority, but a more specific ALLOW rule between two tagged groups will still permit that traffic. The evaluation order is:

1. If a BLOCK rule matches the traffic, check for a more specific ALLOW rule
2. If a matching ALLOW rule exists for the specific source/destination pair, traffic is **allowed**
3. Otherwise, traffic is **blocked**

## Port Rules -- No Ranges

Triton firewall does **not** support port ranges. You cannot write `PORT 8000:9000` or `PORT 8000-9000`.

### What works

```
FROM any TO tag "role" = "web" ALLOW tcp PORT 443
FROM any TO tag "role" = "web" ALLOW tcp PORT 80
FROM any TO tag "role" = "web" ALLOW tcp PORT all
```

### What does NOT work

```
FROM any TO tag "role" = "web" ALLOW tcp PORT 80-443     # INVALID
FROM any TO tag "role" = "web" ALLOW tcp PORT 8000:9000  # INVALID
```

### If you need multiple ports

Create one rule per port:

```
POST /:login/fwrules
{
  "rule": "FROM any TO tag \"role\" = \"web\" ALLOW tcp PORT 80",
  "enabled": true,
  "description": "Allow HTTP"
}
```

```
POST /:login/fwrules
{
  "rule": "FROM any TO tag \"role\" = \"web\" ALLOW tcp PORT 443",
  "enabled": true,
  "description": "Allow HTTPS"
}
```

Or use `PORT all` to allow all ports on a protocol (when appropriate):

```
POST /:login/fwrules
{
  "rule": "FROM tag \"role\" = \"bastion\" TO tag \"role\" = \"web\" ALLOW tcp PORT all",
  "enabled": true,
  "description": "Allow bastion full TCP access to web tier"
}
```

## Environment Isolation

Prevent staging instances from communicating with production.

```
POST /:login/fwrules
{
  "rule": "FROM tag \"env\" = \"staging\" TO tag \"env\" = \"production\" BLOCK tcp PORT all",
  "enabled": true,
  "description": "Block staging from reaching production"
}
```

```
POST /:login/fwrules
{
  "rule": "FROM tag \"env\" = \"staging\" TO tag \"env\" = \"production\" BLOCK udp PORT all",
  "enabled": true,
  "description": "Block staging UDP from reaching production"
}
```

## Managing Rules at Scale

### List all firewall rules

```
GET /:login/fwrules
```

### Get a specific rule

```
GET /:login/fwrules/:id
```

### Update a rule

```
POST /:login/fwrules/:id
{
  "rule": "FROM any TO tag \"role\" = \"lb\" ALLOW tcp PORT 443",
  "enabled": true,
  "description": "Updated: Allow HTTPS to LB"
}
```

### Disable a rule (without deleting)

```
POST /:login/fwrules/:id
{
  "enabled": false
}
```

### Delete a rule

```
DELETE /:login/fwrules/:id
```

### List machines affected by a rule

```
GET /:login/fwrules/:id/machines
```

Returns all instances that currently match the rule's tag selectors. Useful for auditing which machines are impacted before modifying or deleting a rule.

### List firewall rules affecting a specific machine

```
GET /:login/machines/:id/fwrules
```

Returns all active firewall rules that apply to a specific instance. Useful for debugging connectivity issues.

## Rule Syntax Reference

```
FROM <source> TO <target> <action> <protocol> PORT <port>
```

| Component | Options |
|-----------|---------|
| Source/Target | `any`, `ip <address>`, `subnet <cidr>`, `tag "<key>" = "<value>"`, `vm <uuid>` |
| Action | `ALLOW`, `BLOCK` |
| Protocol | `tcp`, `udp`, `icmp` |
| Port | Single port number, or `all` |

### Examples

```
FROM any TO tag "role" = "web" ALLOW tcp PORT 443
FROM ip 10.0.0.0/8 TO tag "role" = "api" ALLOW tcp PORT 3000
FROM subnet 192.168.1.0/24 TO vm <uuid> ALLOW tcp PORT 22
FROM tag "role" = "monitor" TO all vms ALLOW icmp TYPE all
FROM any TO tag "env" = "production" BLOCK tcp PORT 22
```

## Best Practices

1. **Always enable `firewall_enabled: true`** on every instance at creation time
2. **Use tags, not IPs** -- tag-based rules automatically apply to new instances with matching tags
3. **Minimize `FROM any` rules** -- only load balancers and bastions should accept traffic from `any`
4. **One rule per port** -- since port ranges are not supported, create individual rules for each port
5. **Use descriptive names** in the `description` field for auditability
6. **Audit before deleting** -- use `GET /:login/fwrules/:id/machines` to see affected instances
7. **Disable before delete** -- set `enabled: false` first, verify nothing breaks, then delete
8. **Separate environments** -- use BLOCK rules between `env=staging` and `env=production` tags
9. **Bastion pattern** -- all SSH access should route through a bastion host, never directly from the internet
10. **Review regularly** -- list all rules with `GET /:login/fwrules` and remove stale entries
