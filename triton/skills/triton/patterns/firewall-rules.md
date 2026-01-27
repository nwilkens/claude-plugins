# Firewall Rule Patterns

Triton Cloud Firewall provides stateful packet filtering for instances. Rules are defined at the account level and applied dynamically based on tags.

## Rule Syntax

```
FROM <source> TO <target> <action> <protocol> PORT <port>
```

### Source and Target Types

| Type | Syntax | Description |
|------|--------|-------------|
| Any | `any` | Any IP address |
| All VMs | `all vms` | All VMs in your account |
| IP | `ip 192.168.1.1` | Specific IP address |
| CIDR | `ip 10.0.0.0/8` | IP range |
| Subnet | `subnet 192.168.0.0/24` | Network subnet |
| Tag | `tag role` | VMs with tag key |
| Tag Value | `tag role=web` | VMs with specific tag value |
| VM | `vm UUID` | Specific VM by UUID |

### Actions

| Action | Description |
|--------|-------------|
| `ALLOW` | Allow matching traffic |
| `BLOCK` | Block matching traffic |

### Protocols

| Protocol | Description |
|----------|-------------|
| `tcp` | TCP traffic |
| `udp` | UDP traffic |
| `icmp` | ICMP (ping) traffic |

### Port Specifications

| Syntax | Description |
|--------|-------------|
| `PORT 80` | Single port |
| `PORT all` | All ports |
| `(PORT 80 AND PORT 443)` | Multiple specific ports |

> **WARNING: Port Ranges Are NOT Supported**
>
> Despite what some documentation suggests, Triton firewall does **not** support port range syntax like `PORT 1:1024` or `PORT 30000-32767`. These will either fail or be interpreted incorrectly.
>
> **What works:**
> - `PORT 80` - Single port
> - `PORT all` - All ports
> - `(PORT 80 AND PORT 443)` - Specific multiple ports (limited)
>
> **What does NOT work:**
> - `PORT 1:1024` - Range syntax
> - `PORT 30000-32767` - Dash range syntax
> - `(PORT >= 30000 AND PORT <= 32767)` - Comparison operators
>
> **Workaround:** Create individual rules for each port you need, or use `PORT all` for internal cluster communication where appropriate.

## Creating Rules

```bash
triton fwrule create [OPTIONS] "RULE"
```

**Options:**
| Option | Description |
|--------|-------------|
| `-D, --description` | Rule description |
| `-d, --disabled` | Create disabled |
| `--log` | Enable logging |

## Common Patterns

### SSH Access

**Allow SSH from anywhere:**
```bash
triton fwrule create -D "ssh-any" \
  "FROM any TO all vms ALLOW tcp PORT 22"
```

**Restrict SSH to internal network:**
```bash
triton fwrule create -D "ssh-internal" \
  "FROM ip 10.0.0.0/8 TO all vms ALLOW tcp PORT 22"
```

**SSH from specific IP:**
```bash
triton fwrule create -D "ssh-office" \
  "FROM ip 203.0.113.50 TO all vms ALLOW tcp PORT 22"
```

### Web Traffic

**Allow HTTP/HTTPS to web tier:**
```bash
triton fwrule create -D "http" \
  "FROM any TO tag role=web ALLOW tcp PORT 80"

triton fwrule create -D "https" \
  "FROM any TO tag role=web ALLOW tcp PORT 443"
```

**Combined HTTP and HTTPS:**
```bash
triton fwrule create -D "web-traffic" \
  "FROM any TO tag role=web ALLOW tcp (PORT 80 AND PORT 443)"
```

### Load Balancer Rules

**Allow public traffic to LB:**
```bash
triton fwrule create -D "lb-public" \
  "FROM any TO tag role=lb ALLOW tcp (PORT 80 AND PORT 443)"
```

**LB to backend communication:**
```bash
triton fwrule create -D "lb-to-web" \
  "FROM tag role=lb TO tag role=web ALLOW tcp PORT 8080"
```

### Internal Service Communication

**Web to API tier:**
```bash
triton fwrule create -D "web-to-api" \
  "FROM tag role=web TO tag role=api ALLOW tcp PORT 3000"
```

**API to database:**
```bash
triton fwrule create -D "api-to-db" \
  "FROM tag role=api TO tag role=db ALLOW tcp PORT 5432"
```

**API to cache (Redis):**
```bash
triton fwrule create -D "api-to-cache" \
  "FROM tag role=api TO tag role=cache ALLOW tcp PORT 6379"
```

### Monitoring and Metrics

**Allow Prometheus scraping:**
```bash
triton fwrule create -D "prometheus" \
  "FROM tag role=monitoring TO all vms ALLOW tcp PORT 9090"
```

**Allow metrics collection from specific instances:**
```bash
triton fwrule create -D "metrics-web" \
  "FROM tag role=monitoring TO tag role=web ALLOW tcp PORT 9100"
```

### ICMP (Ping)

**Allow ping from internal network:**
```bash
triton fwrule create -D "ping-internal" \
  "FROM ip 10.0.0.0/8 TO all vms ALLOW icmp TYPE 8 CODE 0"
```

## Three-Tier Application Example

```bash
# 1. SSH from bastion only
triton fwrule create -D "ssh-bastion" \
  "FROM tag role=bastion TO all vms ALLOW tcp PORT 22"

# 2. Public HTTPS to load balancer
triton fwrule create -D "https-public" \
  "FROM any TO tag role=lb ALLOW tcp PORT 443"

# 3. LB to web tier
triton fwrule create -D "lb-to-web" \
  "FROM tag role=lb TO tag role=web ALLOW tcp PORT 8080"

# 4. Web to API tier
triton fwrule create -D "web-to-api" \
  "FROM tag role=web TO tag role=api ALLOW tcp PORT 3000"

# 5. API to database
triton fwrule create -D "api-to-db" \
  "FROM tag role=api TO tag role=db ALLOW tcp PORT 5432"

# 6. API to cache
triton fwrule create -D "api-to-cache" \
  "FROM tag role=api TO tag role=cache ALLOW tcp PORT 6379"
```

## Managing Rules

### List Rules
```bash
triton fwrule list
```

### Get Rule Details
```bash
triton fwrule get RULE_ID
```

### Enable/Disable Rules
```bash
triton fwrule enable RULE_ID
triton fwrule disable RULE_ID
```

### Delete Rules
```bash
triton fwrule delete RULE_ID
```

### Update Rules
```bash
triton fwrule update RULE_ID "NEW_RULE_TEXT"
```

### List Instances Affected by Rule
```bash
triton fwrule instances RULE_ID
```

## Instance Firewall Management

### Enable Firewall on Instance
```bash
triton instance enable-firewall INSTANCE
```

### Disable Firewall on Instance
```bash
triton instance disable-firewall INSTANCE
```

### Check Instance Firewall Status
```bash
triton instance get INSTANCE | grep firewall_enabled
```

### List Rules Applied to Instance
```bash
triton instance fwrules INSTANCE
```

### Enable Firewall During Instance Creation
```bash
triton instance create \
  -n my-instance \
  --firewall \
  -w \
  base-64-lts g4-highcpu-1G
```

## Tag Strategy for Firewall Rules

### Application Tags
```bash
triton instance tag set myinstance app=myapp
```

### Role Tags
```bash
triton instance tag set myinstance role=web
triton instance tag set myinstance role=api
triton instance tag set myinstance role=db
triton instance tag set myinstance role=cache
triton instance tag set myinstance role=lb
```

### Environment Tags
```bash
triton instance tag set myinstance env=production
triton instance tag set myinstance env=staging
```

### Combined Tag Rules
```bash
# Only allow production API to access production DB
triton fwrule create -D "prod-api-to-db" \
  "FROM (tag role=api AND tag env=production) TO (tag role=db AND tag env=production) ALLOW tcp PORT 5432"
```

## Best Practices

1. **Always enable firewall on production instances**
   ```bash
   triton instance create --firewall ...
   ```

2. **Use tag-based rules** - Rules automatically apply to new instances with matching tags

3. **Principle of least privilege** - Only allow necessary traffic

4. **Separate by tier** - Web, API, database should have separate rules

5. **Document rules** - Always use `-D` to add descriptions

6. **Restrict SSH** - Never allow SSH from `any` in production; use bastion hosts

7. **Block by default** - Enable firewall, then explicitly allow needed traffic

8. **Use environment separation** - Different rules for production vs staging

## Troubleshooting

### Instance Not Receiving Traffic
1. Check firewall is enabled: `triton instance get INST | grep firewall`
2. List applied rules: `triton instance fwrules INST`
3. Verify instance has correct tags: `triton instance tag list INST`
4. Check rule is enabled: `triton fwrule get RULE_ID`

### Rule Not Applying
1. Verify instance firewall is enabled
2. Check tag matches exactly (case-sensitive)
3. Ensure rule is enabled (not disabled)
4. Wait a few seconds for rule propagation

### Debug with Logging
Enable logging on a rule to see matched traffic:
```bash
triton fwrule update RULE_ID --log
```

Then check instance logs for firewall entries.
