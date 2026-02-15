# Instances (Machines) API

Compute instances are the primary resource in Triton. An instance is either a **container** (brands: `joyent`, `lx`) running directly on SmartOS, or a **virtual machine** (brands: `kvm`, `bhyve`) running a full guest OS. Both are managed through the same API.

## Instance Lifecycle

```
create (POST)
    |
    v
provisioning --> running <--> stopping --> stopped --> deleted (DELETE)
                   ^  |                      |
                   |  +--- reboot -----------+
                   +--- start ---------------+
```

Additional terminal states: `failed` (provisioning error), `incomplete` (partial setup).

---

## Endpoints

### Core Instance Operations

| Method | Path | Description |
|--------|------|-------------|
| GET | `/:login/machines` | List all instances |
| GET | `/:login/machines/:id` | Get a single instance |
| POST | `/:login/machines` | Create a new instance |
| POST | `/:login/machines/:id?action=start` | Start a stopped instance |
| POST | `/:login/machines/:id?action=stop` | Stop a running instance |
| POST | `/:login/machines/:id?action=reboot` | Reboot an instance |
| POST | `/:login/machines/:id?action=resize` | Resize to a different package |
| POST | `/:login/machines/:id?action=rename` | Rename an instance |
| POST | `/:login/machines/:id?action=enable-firewall` | Enable the firewall |
| POST | `/:login/machines/:id?action=disable-firewall` | Disable the firewall |
| DELETE | `/:login/machines/:id` | Delete an instance |
| GET | `/:login/machines/:id/audit` | Get the audit trail |

### Tags

Tags are key-value string pairs attached to an instance. They are used for filtering, CNS service discovery, and firewall rule targeting.

| Method | Path | Description |
|--------|------|-------------|
| POST | `/:login/machines/:id/tags` | Create or replace tags |
| GET | `/:login/machines/:id/tags` | List all tags |
| GET | `/:login/machines/:id/tags/:key` | Get a single tag value |
| DELETE | `/:login/machines/:id/tags/:key` | Delete a single tag |
| DELETE | `/:login/machines/:id/tags` | Delete all tags |

### Metadata

Metadata stores key-value pairs accessible from inside the instance via the metadata API (mdata-get). Used for provisioning scripts, configuration injection, and service coordination.

| Method | Path | Description |
|--------|------|-------------|
| POST | `/:login/machines/:id/metadata` | Update metadata keys |
| GET | `/:login/machines/:id/metadata` | List all metadata |
| GET | `/:login/machines/:id/metadata/:key` | Get a single metadata value |
| DELETE | `/:login/machines/:id/metadata/:key` | Delete a single metadata key |
| DELETE | `/:login/machines/:id/metadata` | Delete all metadata |

### Snapshots

Point-in-time snapshots of an instance's disk. Only supported for container brands (`joyent`, `lx`). For `bhyve`/`kvm` VMs, use image creation instead.

| Method | Path | Description |
|--------|------|-------------|
| POST | `/:login/machines/:id/snapshots` | Create a snapshot |
| GET | `/:login/machines/:id/snapshots` | List all snapshots |
| GET | `/:login/machines/:id/snapshots/:snap_id` | Get a single snapshot |
| DELETE | `/:login/machines/:id/snapshots/:snap_id` | Delete a snapshot |

---

## Create Instance

```
POST /:login/machines
```

### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | No | Instance name. Auto-generated if omitted. Must be unique within the account. |
| `package` | string | Yes | Package UUID or name. Determines CPU, memory, disk, and billing. |
| `image` | string | Yes | Image UUID. The OS or application image to provision from. |
| `networks` | array | No | Array of network UUIDs. Defaults to the account's default fabric network. |
| `firewall_enabled` | boolean | No | Enable cloud firewall. Default: `false`. |
| `tag.*` | string | No | Instance tags, specified as dot-prefixed keys (e.g., `tag.role`). |
| `metadata.*` | string | No | Instance metadata, specified as dot-prefixed keys (e.g., `metadata.user-script`). |
| `affinity` | array | No | Affinity rules for placement (e.g., `["instance!=webserver*"]`). |

### Example: Create a Container (LX Brand)

```json
POST /:login/machines
Content-Type: application/json

{
  "name": "web-01",
  "package": "g1.xsmall",
  "image": "2b683a82-a066-11e3-97ab-2faa44701c5a",
  "networks": ["a9c130da-e3ba-40e9-8b18-112aba886b5e"],
  "firewall_enabled": true,
  "tag.role": "web",
  "tag.env": "production",
  "tag.triton.cns.services": "webapp:443",
  "metadata.user-script": "#!/bin/bash\napt-get update && apt-get install -y nginx"
}
```

### Example: Create a Virtual Machine (Bhyve Brand)

```json
POST /:login/machines
Content-Type: application/json

{
  "name": "database-01",
  "package": "g1.medium",
  "image": "d42c37f4-2163-11e6-8b80-d32ef9490e01",
  "networks": ["b1963383-6b1e-4b97-8be2-65b1e36f2e56"],
  "firewall_enabled": true,
  "tag.role": "database",
  "tag.env": "production",
  "metadata.user-script": "#!/bin/bash\ncurl -fsSL https://example.com/setup-db.sh | bash"
}
```

### Response

Returns the full instance object (see Instance Object below) with `state: "provisioning"`.

**Status Code:** `201 Created`

---

## Instance Object

The response schema for a single instance (returned by GET, POST create, and as array elements in list):

```json
{
  "id": "b6979942-7d5d-4fe6-a2ec-b812e950625a",
  "name": "web-01",
  "type": "smartmachine",
  "brand": "lx",
  "state": "running",
  "image": "2b683a82-a066-11e3-97ab-2faa44701c5a",
  "memory": 1024,
  "disk": 25600,
  "metadata": {
    "user-script": "#!/bin/bash\napt-get update"
  },
  "tags": {
    "role": "web",
    "env": "production",
    "triton.cns.services": "webapp:443"
  },
  "created": "2024-01-15T08:24:00.000Z",
  "updated": "2024-01-15T08:25:12.000Z",
  "nics": [
    {
      "ip": "10.0.1.5",
      "mac": "90:b8:d0:c0:ff:ee",
      "primary": true,
      "network": "a9c130da-e3ba-40e9-8b18-112aba886b5e",
      "gateway": "10.0.1.1",
      "netmask": "255.255.255.0"
    }
  ],
  "ips": ["10.0.1.5"],
  "networks": ["a9c130da-e3ba-40e9-8b18-112aba886b5e"],
  "primaryIp": "10.0.1.5",
  "firewall_enabled": true,
  "compute_node": "44454c4c-5000-1048-8044-b3c04f585131",
  "package": "g1.xsmall",
  "dns_names": [
    "web-01.inst.b6979942.us-east-1.cns.example.com",
    "webapp.svc.b6979942.us-east-1.cns.example.com"
  ]
}
```

### Field Reference

| Field | Type | Description |
|-------|------|-------------|
| `id` | string (UUID) | Unique instance identifier. |
| `name` | string | Human-readable instance name. |
| `type` | string | `smartmachine` (container) or `virtualmachine` (VM). |
| `brand` | string | Instance brand: `joyent`, `lx`, `kvm`, or `bhyve`. |
| `state` | string | Current lifecycle state (see states below). |
| `image` | string (UUID) | The image used to provision this instance. |
| `memory` | integer | RAM in megabytes. |
| `disk` | integer | Disk quota in megabytes. |
| `metadata` | object | Key-value metadata accessible from inside the instance. |
| `tags` | object | Key-value tags for filtering, CNS, and firewall rules. |
| `created` | string (ISO 8601) | Creation timestamp. |
| `updated` | string (ISO 8601) | Last modification timestamp. |
| `nics` | array | Network interfaces with IP, MAC, network UUID, and gateway. |
| `ips` | array | Flat list of all IP addresses. |
| `networks` | array | Flat list of all network UUIDs. |
| `primaryIp` | string | The primary IP address. |
| `firewall_enabled` | boolean | Whether the cloud firewall is active. |
| `compute_node` | string (UUID) | Physical server hosting this instance (operator-only). |
| `package` | string | The package name or UUID. |
| `dns_names` | array | CNS DNS names (instance names and service names). |

### Instance States

| State | Description |
|-------|-------------|
| `provisioning` | Instance is being created. Not yet accessible. |
| `running` | Instance is active and accessible. |
| `stopping` | Instance is in the process of shutting down. |
| `stopped` | Instance is shut down. Not billed for compute (still billed for disk). |
| `failed` | Provisioning failed. Check audit log for details. |
| `incomplete` | Partial setup. May require manual intervention or deletion. |
| `deleted` | Instance has been destroyed. Only visible in audit logs. |

---

## List Instances

```
GET /:login/machines
```

### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `type` | string | Filter by type: `smartmachine` or `virtualmachine`. |
| `brand` | string | Filter by brand: `joyent`, `lx`, `kvm`, `bhyve`. |
| `state` | string | Filter by state: `provisioning`, `running`, `stopped`, etc. |
| `name` | string | Filter by exact instance name. |
| `image` | string | Filter by image UUID. |
| `memory` | integer | Filter by memory (MB). |
| `tag.*` | string | Filter by tag value (e.g., `tag.role=web`). |
| `docker` | boolean | Filter Docker instances. |
| `credentials` | boolean | Include generated credentials in the response. |
| `limit` | integer | Maximum number of results. Default: `1000`. Max: `1000`. |
| `offset` | integer | Offset for pagination. Default: `0`. |

### Pagination

Use `offset` and `limit` for pagination. The response includes an `x-resource-count` header with the total number of matching instances.

### Examples

**List all running instances:**
```
GET /:login/machines?state=running
```

**List instances with a specific tag:**
```
GET /:login/machines?tag.role=web
```

**List bhyve VMs, paginated:**
```
GET /:login/machines?brand=bhyve&limit=25&offset=0
```

**List by name:**
```
GET /:login/machines?name=web-01
```

---

## Instance Actions

All actions are performed via POST with a query parameter specifying the action.

### Start

```
POST /:login/machines/:id?action=start
```

Start a stopped instance. No request body required.

**Status Code:** `202 Accepted`

### Stop

```
POST /:login/machines/:id?action=stop
```

Gracefully stop a running instance. No request body required.

**Status Code:** `202 Accepted`

### Reboot

```
POST /:login/machines/:id?action=reboot
```

Reboot a running instance. No request body required.

**Status Code:** `202 Accepted`

### Resize

```
POST /:login/machines/:id?action=resize
```

Change the instance's package (CPU, memory, disk).

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `package` | string | Yes | New package UUID or name. |

```json
POST /:login/machines/:id?action=resize
Content-Type: application/json

{
  "package": "g1.medium"
}
```

**Note:** Resizing a running instance may require a reboot for changes to take effect. Not all package transitions are allowed -- disk can only increase, never decrease.

**Status Code:** `202 Accepted`

### Rename

```
POST /:login/machines/:id?action=rename
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | New instance name. |

```json
POST /:login/machines/:id?action=rename
Content-Type: application/json

{
  "name": "web-02"
}
```

**Status Code:** `202 Accepted`

### Enable Firewall

```
POST /:login/machines/:id?action=enable-firewall
```

Enable the cloud firewall on the instance. Firewall rules targeting this instance (by tag or ID) will begin to apply.

**Status Code:** `202 Accepted`

### Disable Firewall

```
POST /:login/machines/:id?action=disable-firewall
```

Disable the cloud firewall. All inbound traffic will be allowed.

**Status Code:** `202 Accepted`

---

## Delete Instance

```
DELETE /:login/machines/:id
```

Permanently destroy an instance. This is irreversible. The instance must be in `stopped` or `running` state.

**Status Code:** `204 No Content`

---

## Tags

Tags are key-value string pairs used for:

- **Filtering** instances in list queries (`?tag.role=web`)
- **CNS service discovery** (see CNS Tags below)
- **Firewall rules** (`FROM tag role = web TO ...`)

### Create or Update Tags

```
POST /:login/machines/:id/tags
Content-Type: application/json

{
  "role": "web",
  "env": "production",
  "triton.cns.services": "webapp:443"
}
```

This **merges** the provided tags with existing tags. To replace a tag, include it with the new value. To replace all tags, delete all first and then create.

**Status Code:** `200 OK`

### List Tags

```
GET /:login/machines/:id/tags
```

Returns a JSON object of all tags.

### Get a Single Tag

```
GET /:login/machines/:id/tags/:key
```

Returns the tag value as a plain string.

### Delete a Single Tag

```
DELETE /:login/machines/:id/tags/:key
```

**Status Code:** `204 No Content`

### Delete All Tags

```
DELETE /:login/machines/:id/tags
```

**Status Code:** `204 No Content`

### CNS Tags

These special tags control Container Name Service (CNS) behavior:

| Tag | Description | Example |
|-----|-------------|---------|
| `triton.cns.services` | Register the instance under one or more DNS service names. Format: `service:port` or `service:port,service2:port2`. | `webapp:443` |
| `triton.cns.status` | Override instance CNS status. Set to `down` to remove from DNS before stopping. Set to `up` to re-register. | `down` |
| `triton.cns.disable` | Set to `true` to completely disable CNS for this instance. No DNS records will be created. | `true` |
| `triton.cns.reverse_ptr` | Custom reverse DNS (PTR) record for the instance's public IP. | `mail.example.com` |

**CNS DNS name format:**

- Instance name: `<instance-name>.inst.<account-uuid>.<dc>.cns.<zone>`
- Service name: `<service>.svc.<account-uuid>.<dc>.cns.<zone>`

When multiple instances share the same `triton.cns.services` tag, CNS creates a round-robin DNS entry across all of them -- this is the foundation for service discovery and load balancing in Triton.

---

## Metadata

Metadata stores configuration data accessible from inside the instance via the SmartOS metadata protocol (`mdata-get`, `mdata-list`).

### Update Metadata

```
POST /:login/machines/:id/metadata
Content-Type: application/json

{
  "app-config": "{\"db_host\":\"10.0.1.10\",\"db_port\":5432}",
  "deploy-version": "v2.1.0"
}
```

Merges with existing metadata. Keys prefixed with `user-` and the special `user-script` key are user-writable. Other keys may be read-only (set during provisioning).

**Status Code:** `200 OK`

### List Metadata

```
GET /:login/machines/:id/metadata
```

Returns a JSON object of all metadata key-value pairs. Pass `?credentials=true` to include generated credentials (if any).

### Special Metadata Keys

| Key | Description |
|-----|-------------|
| `user-script` | Shell script executed on first boot. Runs as root. Used for automated provisioning. |
| `user-data` | Arbitrary data blob. Often used for cloud-init configuration or application config. |
| `operator-script` | Operator-level boot script (operator-only). |

---

## Snapshots

Snapshots capture the full state of a container's ZFS dataset at a point in time.

### Create Snapshot

```
POST /:login/machines/:id/snapshots
Content-Type: application/json

{
  "name": "pre-upgrade-2024-01-15"
}
```

**Status Code:** `201 Created`

### List Snapshots

```
GET /:login/machines/:id/snapshots
```

### Get Snapshot

```
GET /:login/machines/:id/snapshots/:snap_id
```

### Snapshot Object

```json
{
  "name": "pre-upgrade-2024-01-15",
  "state": "created",
  "created": "2024-01-15T10:00:00.000Z",
  "updated": "2024-01-15T10:00:05.000Z"
}
```

### Delete Snapshot

```
DELETE /:login/machines/:id/snapshots/:snap_id
```

**Status Code:** `204 No Content`

**Note:** Snapshots are only available for container brands (`joyent`, `lx`). They are not supported for `kvm` or `bhyve` virtual machines.

---

## Audit Trail

```
GET /:login/machines/:id/audit
```

Returns a chronological list of all actions performed on the instance, including who performed them and when.

```json
[
  {
    "success": "yes",
    "time": "2024-01-15T08:24:00.000Z",
    "action": "provision",
    "caller": {
      "type": "signature",
      "ip": "198.51.100.10",
      "keyId": "/<account>/keys/<fingerprint>"
    }
  },
  {
    "success": "yes",
    "time": "2024-01-15T12:30:00.000Z",
    "action": "stop",
    "caller": {
      "type": "signature",
      "ip": "198.51.100.10",
      "keyId": "/<account>/keys/<fingerprint>"
    }
  }
]
```

---

## Instance Brands

| Brand | Type | Description |
|-------|------|-------------|
| `joyent` | `smartmachine` | Native SmartOS zone (container). Runs SmartOS/illumos binaries. Highest density and performance. |
| `lx` | `smartmachine` | Linux-branded zone (container). Runs unmodified Linux binaries on SmartOS via LX emulation. Best balance of density and Linux compatibility. |
| `kvm` | `virtualmachine` | Hardware-virtualized VM using KVM. Full guest OS. Legacy -- use `bhyve` for new VMs. |
| `bhyve` | `virtualmachine` | Hardware-virtualized VM using bhyve. Full guest OS with better performance than KVM. Preferred for new VMs. |

**Key differences:**

- **Containers** (`joyent`, `lx`): Share the host kernel. Sub-second boot. Lower overhead. Support ZFS snapshots. Cannot run arbitrary kernels.
- **Virtual machines** (`kvm`, `bhyve`): Full hardware virtualization. Run any OS. Higher overhead. Longer boot times. No ZFS snapshot support (use image creation).

---

## Practical Examples

### Provision a Web Server with CNS

```json
POST /:login/machines
{
  "name": "nginx-01",
  "package": "g1.xsmall",
  "image": "<ubuntu-lx-image-uuid>",
  "networks": ["<fabric-network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "web",
  "tag.triton.cns.services": "nginx:443,nginx:80",
  "metadata.user-script": "#!/bin/bash\nset -euo pipefail\napt-get update\napt-get install -y nginx\nsystemctl enable nginx\nsystemctl start nginx"
}
```

### Graceful Instance Removal (CNS Drain)

Before deleting an instance that is registered in CNS, remove it from DNS first to allow clients to stop sending traffic:

```json
POST /:login/machines/:id/tags
{
  "triton.cns.status": "down"
}
```

Wait for DNS TTL to expire (typically 30 seconds), then stop and delete:

```
POST /:login/machines/:id?action=stop
DELETE /:login/machines/:id
```

### Tag-Based Filtering for Deployment

List all web instances in production:

```
GET /:login/machines?tag.role=web&tag.env=production&state=running
```

### Resize an Instance

```json
POST /:login/machines/:id?action=resize
{
  "package": "g1.medium"
}
```

Then reboot to apply:

```
POST /:login/machines/:id?action=reboot
```

### Snapshot Before Upgrade

```json
POST /:login/machines/:id/snapshots
{
  "name": "pre-upgrade-v2"
}
```

Verify the snapshot was created:

```
GET /:login/machines/:id/snapshots
```

If the upgrade fails, roll back by booting from the snapshot.

### Inject Configuration via Metadata

```json
POST /:login/machines/:id/metadata
{
  "app-config": "{\"redis_host\":\"10.0.2.5\",\"redis_port\":6379}",
  "deploy-version": "v3.0.1"
}
```

Inside the instance, read it:

```bash
mdata-get app-config | jq .
```

### Create a Database VM on a Private Network

```json
POST /:login/machines
{
  "name": "postgres-primary",
  "package": "g1.large",
  "image": "<ubuntu-bhyve-image-uuid>",
  "networks": ["<private-fabric-uuid>"],
  "firewall_enabled": true,
  "tag.role": "database",
  "tag.triton.cns.services": "postgres:5432",
  "metadata.user-script": "#!/bin/bash\nset -euo pipefail\napt-get update\napt-get install -y postgresql\nsystemctl enable postgresql\nsystemctl start postgresql"
}
```

This VM will only be reachable on the private fabric network and discoverable via CNS at `postgres.svc.<account-uuid>.<dc>.cns.<zone>`.

---

## Error Responses

| Status | Code | Description |
|--------|------|-------------|
| 400 | `InvalidArgument` | Missing or invalid parameter (bad UUID, unknown package, etc.). |
| 404 | `ResourceNotFound` | Instance does not exist or is not owned by this account. |
| 409 | `InvalidState` | Action not allowed in current state (e.g., starting a running instance). |
| 422 | `InsufficientCapacity` | No compute nodes with enough resources for the requested package. |
| 503 | `ServiceUnavailable` | Backend infrastructure is temporarily unavailable. |
