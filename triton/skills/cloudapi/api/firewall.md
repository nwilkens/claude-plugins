# Firewall Rules API

Manage firewall rules that control network traffic to and from instances. Rules use a domain-specific language (DSL) to define traffic policies based on IPs, subnets, tags, or specific VMs.

Firewall rules only apply to instances that have `firewall_enabled: true` set. Instances without this flag are unaffected by any rules.

## Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/:login/fwrules` | List all firewall rules |
| GET | `/:login/fwrules/:id` | Get a single firewall rule |
| POST | `/:login/fwrules` | Create a new firewall rule |
| POST | `/:login/fwrules/:id` | Update an existing firewall rule |
| POST | `/:login/fwrules/:id/enable` | Enable a rule |
| POST | `/:login/fwrules/:id/disable` | Disable a rule |
| DELETE | `/:login/fwrules/:id` | Delete a rule |
| GET | `/:login/fwrules/:id/machines` | List instances affected by a rule |

---

## List Firewall Rules

```
GET /:login/fwrules
```

Returns an array of all firewall rules for the account.

### Response

```json
[
  {
    "id": "38de17c4-39e8-48c7-a168-0f58083de860",
    "rule": "FROM any TO tag role = web ALLOW tcp PORT 443",
    "enabled": true,
    "description": "Allow HTTPS to web tier"
  }
]
```

---

## Get Firewall Rule

```
GET /:login/fwrules/:id
```

Returns a single firewall rule by UUID.

### Response

```json
{
  "id": "38de17c4-39e8-48c7-a168-0f58083de860",
  "rule": "FROM any TO tag role = web ALLOW tcp PORT 443",
  "enabled": true,
  "description": "Allow HTTPS to web tier"
}
```

---

## Create Firewall Rule

```
POST /:login/fwrules
```

### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule` | string | yes | Rule text in the firewall DSL syntax |
| `enabled` | boolean | no | Whether the rule is active (default: `false`) |
| `description` | string | no | Human-readable description of the rule's purpose |

### Example

```json
{
  "rule": "FROM any TO tag role = web ALLOW tcp PORT 443",
  "enabled": true,
  "description": "Allow HTTPS to web tier"
}
```

### Response

Returns the created rule object with its assigned `id`.

```json
{
  "id": "38de17c4-39e8-48c7-a168-0f58083de860",
  "rule": "FROM any TO tag role = web ALLOW tcp PORT 443",
  "enabled": true,
  "description": "Allow HTTPS to web tier"
}
```

---

## Update Firewall Rule

```
POST /:login/fwrules/:id
```

Updates an existing rule. Any fields not included in the request body remain unchanged.

### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule` | string | no | Updated rule text |
| `enabled` | boolean | no | Enable or disable the rule |
| `description` | string | no | Updated description |

### Example

```json
{
  "rule": "FROM any TO tag role = web ALLOW tcp PORT 8443",
  "description": "Allow HTTPS on alternate port to web tier"
}
```

---

## Enable / Disable Rule

```
POST /:login/fwrules/:id/enable
POST /:login/fwrules/:id/disable
```

Convenience endpoints to toggle a rule on or off without modifying any other fields. No request body required.

---

## Delete Firewall Rule

```
DELETE /:login/fwrules/:id
```

Permanently removes a firewall rule. Returns `204 No Content` on success.

---

## List Affected Instances

```
GET /:login/fwrules/:id/machines
```

Returns an array of instance objects that are affected by the specified rule. This is useful for understanding the blast radius of a rule change before modifying or deleting it.

---

## Rule Syntax (DSL)

Firewall rules are written in a domain-specific language with the following structure:

```
FROM <source> TO <target> <action> <protocol> PORT <port>
```

### Source and Target Types

| Type | Syntax | Description |
|------|--------|-------------|
| Any IP | `any` | Matches all IP addresses (internet + private) |
| All VMs | `all vms` | All instances in the account |
| Specific IP | `ip X.X.X.X` | A single IP address |
| Subnet | `subnet X.X.X.X/CIDR` | A CIDR range (e.g., `subnet 10.0.1.0/24`) |
| Tag | `tag key = value` | All instances matching a specific tag key-value pair |
| Specific VM | `vm UUID` | A single instance identified by UUID |

### Actions

| Action | Description |
|--------|-------------|
| `ALLOW` | Permit traffic matching this rule |
| `BLOCK` | Deny traffic matching this rule |

### Protocols

| Protocol | Description | Port Syntax |
|----------|-------------|-------------|
| `tcp` | TCP traffic | `PORT <number>` or `PORT all` |
| `udp` | UDP traffic | `PORT <number>` or `PORT all` |
| `icmp` | ICMP traffic | `TYPE <type> CODE <code>` (e.g., `TYPE 8 CODE 0` for ping) |

### Priority

Rules can include an optional `PRIORITY` clause with a value from 0 to 100. Lower numbers indicate higher priority.

```
FROM all vms TO any ALLOW tcp PORT 443 PRIORITY 1
```

When no priority is specified, `ALLOW` rules take precedence over `BLOCK` rules at the same priority level. Use explicit priorities when you need `BLOCK` rules to override `ALLOW` rules or to establish a specific evaluation order.

---

## Port Range Limitation

> **WARNING: Port ranges are NOT supported in the firewall rule DSL.**
>
> The following syntaxes **do not work** and will be rejected:
>
> - `PORT 1:1024` -- range syntax is invalid
> - `PORT >= 30000 AND PORT <= 32767` -- comparison operators are invalid
> - `PORT 80,443` -- comma-separated lists are invalid
>
> **You must create one rule per port**, or use `PORT all` to match all ports on a protocol.
>
> For example, to allow both HTTP and HTTPS, create two separate rules:
> ```
> FROM any TO tag role = web ALLOW tcp PORT 80
> FROM any TO tag role = web ALLOW tcp PORT 443
> ```

This is the single most common source of errors when working with Triton firewall rules programmatically. Always generate individual rules for each port.

---

## The `firewall_enabled` Flag

Firewall rules are only enforced on instances that have `firewall_enabled: true`. This flag is set at instance creation time or updated afterward.

### Enabling at Creation

```json
POST /:login/machines
{
  "name": "web-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "firewall_enabled": true
}
```

### Enabling on an Existing Instance

```json
POST /:login/machines/:id?action=enable_firewall
```

### Disabling on an Existing Instance

```json
POST /:login/machines/:id?action=disable_firewall
```

### Default Behavior

- When `firewall_enabled` is `false` (the default), **all traffic is allowed** to and from the instance. No firewall rules apply.
- When `firewall_enabled` is `true`, the default policy is **deny all inbound** traffic. Only traffic explicitly permitted by `ALLOW` rules is accepted. Outbound traffic is allowed by default.

This means that enabling the firewall without any `ALLOW` rules will block all inbound connections to the instance.

---

## Common Rule Patterns

### Basic Web Server

Allow HTTP and HTTPS from the internet:

```
FROM any TO tag role = web ALLOW tcp PORT 80
FROM any TO tag role = web ALLOW tcp PORT 443
```

### SSH Access

Allow SSH from anywhere (use sparingly in production):

```
FROM any TO tag role = web ALLOW tcp PORT 22
```

Restrict SSH to a bastion/jump host subnet:

```
FROM subnet 10.0.0.0/24 TO all vms ALLOW tcp PORT 22
```

### Multi-Tier Application

Load balancer receives public traffic, forwards to application tier, which connects to the database tier:

```
# Public HTTPS to load balancers
FROM any TO tag role = lb ALLOW tcp PORT 443

# Load balancers to application backends
FROM tag role = lb TO tag role = app ALLOW tcp PORT 8080

# Application tier to database
FROM tag role = app TO tag role = db ALLOW tcp PORT 5432

# Application tier to cache (Redis)
FROM tag role = app TO tag role = cache ALLOW tcp PORT 6379
```

### Block-Then-Allow Pattern

Block all outbound traffic by default, then selectively allow specific ports. Use `PRIORITY` to ensure the allow rules override the block:

```
FROM all vms TO any BLOCK tcp PORT all
FROM all vms TO any ALLOW tcp PORT 443 PRIORITY 1
FROM all vms TO any ALLOW tcp PORT 80 PRIORITY 1
FROM all vms TO any ALLOW tcp PORT 53 PRIORITY 1
FROM all vms TO any ALLOW udp PORT 53 PRIORITY 1
```

### ICMP (Ping)

Allow ping to all instances:

```
FROM any TO all vms ALLOW icmp TYPE 8 CODE 0
```

### Environment Isolation

Prevent production and staging instances from communicating:

```
FROM tag env = staging TO tag env = production BLOCK tcp PORT all
FROM tag env = production TO tag env = staging BLOCK tcp PORT all
```

### Specific VM Communication

Allow a monitoring server to reach all instances on the metrics port:

```
FROM vm 8a4f3e2d-1b5c-4a6e-9d7f-0c2e3b4a5d6f TO all vms ALLOW tcp PORT 9100
```

---

## Tag Strategy

Tags are the primary mechanism for building dynamic, scalable firewall rules. Instead of referencing individual VM UUIDs or IP addresses, tag-based rules automatically apply to any instance that carries the matching tag.

### Recommended Tag Taxonomy

| Tag Key | Example Values | Purpose |
|---------|---------------|---------|
| `role` | `web`, `app`, `db`, `cache`, `lb`, `monitor` | Functional tier of the instance |
| `env` | `production`, `staging`, `development` | Deployment environment |
| `service` | `api`, `worker`, `scheduler`, `gateway` | Specific service running on the instance |
| `project` | `myapp`, `billing`, `auth` | Project or application grouping |

### How Tags Work with Rules

When a rule references `tag role = web`, it automatically applies to every instance in the account that has the tag `role=web`. Adding or removing the tag from an instance immediately changes which rules affect it -- no rule modifications needed.

This makes scaling straightforward:

1. Define rules once using tags (e.g., `FROM any TO tag role = web ALLOW tcp PORT 443`).
2. Provision new instances with the appropriate tags.
3. The new instances are automatically protected by the existing rules.

### Setting Tags on Instances

Tags are set at creation time or updated afterward:

```json
POST /:login/machines
{
  "name": "web-03",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "firewall_enabled": true,
  "tag.role": "web",
  "tag.env": "production",
  "tag.service": "api"
}
```

Or add tags to an existing instance:

```json
PUT /:login/machines/:id/tags
{
  "role": "web",
  "env": "production"
}
```

---

## Best Practices

1. **Always enable firewalls on production instances.** Set `firewall_enabled: true` at creation time. Instances without this flag accept all traffic.

2. **Use tag-based rules over VM-specific rules.** Tag-based rules scale automatically as you add or remove instances. VM UUID-based rules require manual updates.

3. **Follow the principle of least privilege.** Start with the default deny-all-inbound policy and explicitly allow only the ports and sources each tier needs.

4. **Create one rule per port.** Due to the port range limitation, structure your automation to emit individual rules for each required port.

5. **Use descriptions on every rule.** When managing dozens of rules, descriptions are essential for understanding the intent without parsing the DSL.

6. **Check affected instances before deleting rules.** Use `GET /:login/fwrules/:id/machines` to understand which instances will be impacted before removing a rule.

7. **Separate environments with tags.** Use `env` tags to ensure staging rules do not accidentally affect production instances and vice versa.

8. **Keep SSH access restricted.** Avoid `FROM any TO all vms ALLOW tcp PORT 22` in production. Prefer bastion hosts or subnet-scoped rules.

9. **Enable rules at creation.** Set `"enabled": true` when creating rules. Creating a disabled rule and forgetting to enable it is a common mistake.

10. **Use the block-then-allow pattern for outbound control.** If you need to restrict egress, block all outbound traffic at a lower priority and selectively allow at a higher priority.
