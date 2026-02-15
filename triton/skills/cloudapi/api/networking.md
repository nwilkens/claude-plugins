# Networking API

Networks, fabric VLANs, fabric networks, and NICs (network interfaces). Triton provides two categories of networks: **public networks** managed by the operator and **fabric (private) networks** owned by individual accounts. Fabric networks use VLANs for segmentation, enabling multi-tier architectures with full tenant isolation.

---

## Concepts

### Public Networks vs Fabric Networks

| | Public Networks | Fabric Networks |
|--|----------------|-----------------|
| **Visibility** | Shared across all accounts | Private to the owning account |
| **IP space** | Operator-assigned public IPs | Account-defined RFC 1918 subnets |
| **Isolation** | None (internet-routable) | Full tenant isolation at the network layer |
| **Management** | Operator-managed | Account-managed via API |
| **Use case** | Internet-facing services | Internal communication between tiers |

Public networks appear in `GET /:login/networks` with `"public": true`. Fabric networks appear with `"fabric": true` and `"public": false`.

### Fabric Tenant Isolation

Fabric networks are implemented as overlay networks (VXLAN). Each account's fabric traffic is completely isolated from every other account, even when using identical subnets (e.g., two accounts can both use `10.0.0.0/24` without conflict). Within an account, VLANs provide further segmentation between tiers.

### VLAN Segmentation

VLANs partition an account's fabric into isolated broadcast domains. Instances on VLAN 100 cannot communicate with instances on VLAN 200 unless explicitly routed. This is the foundation for multi-tier architectures where web, application, and database tiers each live on separate VLANs with controlled access between them.

### Internet NAT

Fabric networks can optionally enable `internet_nat`, which provides outbound internet access to instances on that network without assigning a public IP. When `internet_nat` is `true`, instances can reach the internet (for package updates, API calls, etc.) but are not directly reachable from the internet.

**When to disable internet NAT:**
- Database instances that should have no internet connectivity at all
- High-security tiers where all traffic must flow through a controlled gateway
- Compliance environments requiring strict network egress controls

---

## Networks

### ListNetworks

List all networks available to the account, including both public operator networks and the account's fabric networks.

```
GET /:login/networks
```

**Response:** `200 OK`

```json
[
  {
    "id": "7326787b-8039-436c-a533-5038f7c3abba",
    "name": "external",
    "public": true,
    "fabric": false,
    "description": "Operator external network",
    "subnet": "203.0.113.0/24",
    "gateway": "203.0.113.1",
    "provision_start_ip": "203.0.113.2",
    "provision_end_ip": "203.0.113.254",
    "resolvers": ["8.8.8.8", "8.8.4.4"],
    "routes": {},
    "internet_nat": false
  },
  {
    "id": "a1c2e3f4-5678-9abc-def0-1234567890ab",
    "name": "app-network",
    "public": false,
    "fabric": true,
    "description": "Application tier network",
    "subnet": "10.0.1.0/24",
    "gateway": "10.0.1.1",
    "provision_start_ip": "10.0.1.2",
    "provision_end_ip": "10.0.1.254",
    "resolvers": ["8.8.8.8"],
    "routes": {},
    "internet_nat": true
  }
]
```

### GetNetwork

Get details for a specific network.

```
GET /:login/networks/:id
```

**Parameters:**

| Name | In | Type | Description |
|------|----|------|-------------|
| `id` | path | UUID | Network ID |

**Response:** `200 OK`

```json
{
  "id": "a1c2e3f4-5678-9abc-def0-1234567890ab",
  "name": "app-network",
  "public": false,
  "fabric": true,
  "description": "Application tier network",
  "subnet": "10.0.1.0/24",
  "gateway": "10.0.1.1",
  "provision_start_ip": "10.0.1.2",
  "provision_end_ip": "10.0.1.254",
  "resolvers": ["8.8.8.8", "8.8.4.4"],
  "routes": {"10.0.2.0/24": "10.0.1.1"},
  "internet_nat": true
}
```

### Network Object

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique network identifier |
| `name` | string | Network name |
| `public` | boolean | `true` for operator-managed public networks |
| `fabric` | boolean | `true` for account-owned fabric networks |
| `description` | string | Human-readable description |
| `subnet` | string | CIDR notation subnet |
| `gateway` | string | Gateway IP address |
| `provision_start_ip` | string | First IP in the provisioning range |
| `provision_end_ip` | string | Last IP in the provisioning range |
| `resolvers` | array[string] | DNS resolver IP addresses |
| `routes` | object | Static routes (`destination CIDR` -> `gateway IP`) |
| `internet_nat` | boolean | Whether outbound internet NAT is enabled (fabric networks only) |

---

## Network IPs

Manage individual IP addresses within a network. Used primarily for IP reservation, which prevents an IP from being automatically assigned to new instances.

### ListNetworkIPs

```
GET /:login/networks/:id/ips
```

**Response:** `200 OK`

```json
[
  {
    "ip": "10.0.1.2",
    "reserved": false,
    "managed": false,
    "belongs_to_uuid": "b6bc7e9a-4321-abcd-ef01-23456789abcd"
  },
  {
    "ip": "10.0.1.3",
    "reserved": true,
    "managed": false,
    "belongs_to_uuid": ""
  }
]
```

### GetNetworkIP

```
GET /:login/networks/:id/ips/:ip
```

**Parameters:**

| Name | In | Type | Description |
|------|----|------|-------------|
| `id` | path | UUID | Network ID |
| `ip` | path | string | IP address |

**Response:** `200 OK`

```json
{
  "ip": "10.0.1.5",
  "reserved": true,
  "managed": false,
  "belongs_to_uuid": ""
}
```

### UpdateNetworkIP

Reserve or unreserve an IP address. Reserved IPs are excluded from automatic provisioning, allowing you to hold specific addresses for future use or manual assignment.

```
PUT /:login/networks/:id/ips/:ip
```

**Request body:**

```json
{
  "reserved": true
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reserved` | boolean | yes | `true` to reserve, `false` to unreserve |

**Response:** `200 OK`

```json
{
  "ip": "10.0.1.5",
  "reserved": true,
  "managed": false,
  "belongs_to_uuid": ""
}
```

### IP Object

| Field | Type | Description |
|-------|------|-------------|
| `ip` | string | The IP address |
| `reserved` | boolean | Whether the IP is reserved (excluded from auto-assignment) |
| `managed` | boolean | Whether the IP is managed by the system (e.g., gateway) |
| `belongs_to_uuid` | UUID or empty string | Instance UUID if the IP is currently assigned |

**Use cases for IP reservation:**
- Reserve a known IP for a database instance before provisioning
- Hold IPs for future blue-green deployments
- Prevent specific addresses from being assigned during scaling operations

---

## Fabric VLANs

VLANs segment an account's fabric network space. Each VLAN is an isolated Layer 2 domain. Fabric networks are created on top of VLANs.

All VLAN endpoints use the `default` fabric (currently the only supported fabric).

### ListFabricVLANs

```
GET /:login/fabrics/default/vlans
```

**Response:** `200 OK`

```json
[
  {
    "vlan_id": 100,
    "name": "web-vlan",
    "description": "Web tier VLAN"
  },
  {
    "vlan_id": 200,
    "name": "api-vlan",
    "description": "API/application tier VLAN"
  },
  {
    "vlan_id": 300,
    "name": "db-vlan",
    "description": "Database tier VLAN"
  }
]
```

### CreateFabricVLAN

```
POST /:login/fabrics/default/vlans
```

**Request body:**

```json
{
  "vlan_id": 100,
  "name": "web-vlan",
  "description": "Web tier VLAN"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vlan_id` | integer | yes | VLAN ID (0-4095) |
| `name` | string | yes | VLAN name |
| `description` | string | no | Human-readable description |

**Response:** `201 Created`

```json
{
  "vlan_id": 100,
  "name": "web-vlan",
  "description": "Web tier VLAN"
}
```

### GetFabricVLAN

```
GET /:login/fabrics/default/vlans/:vlan_id
```

**Parameters:**

| Name | In | Type | Description |
|------|----|------|-------------|
| `vlan_id` | path | integer | VLAN ID |

**Response:** `200 OK`

```json
{
  "vlan_id": 100,
  "name": "web-vlan",
  "description": "Web tier VLAN"
}
```

### DeleteFabricVLAN

Delete a VLAN. The VLAN must have no networks on it before deletion.

```
DELETE /:login/fabrics/default/vlans/:vlan_id
```

**Response:** `204 No Content`

### VLAN Object

| Field | Type | Description |
|-------|------|-------------|
| `vlan_id` | integer | VLAN identifier (0-4095) |
| `name` | string | VLAN name |
| `description` | string | Human-readable description |

---

## Fabric Networks

Fabric networks are created on VLANs and define the IP subnet, gateway, and provisioning range for instances. Each VLAN can host multiple fabric networks.

### ListFabricNetworks

List all networks on a specific VLAN.

```
GET /:login/fabrics/default/vlans/:vlan_id/networks
```

**Response:** `200 OK`

```json
[
  {
    "id": "a1c2e3f4-5678-9abc-def0-1234567890ab",
    "name": "web-network",
    "public": false,
    "fabric": true,
    "description": "Web tier network on VLAN 100",
    "subnet": "10.0.1.0/24",
    "gateway": "10.0.1.1",
    "provision_start_ip": "10.0.1.2",
    "provision_end_ip": "10.0.1.254",
    "resolvers": ["8.8.8.8", "8.8.4.4"],
    "routes": {},
    "internet_nat": true
  }
]
```

### CreateFabricNetwork

Create a new fabric network on a VLAN.

```
POST /:login/fabrics/default/vlans/:vlan_id/networks
```

**Request body:**

```json
{
  "name": "web-network",
  "subnet": "10.0.1.0/24",
  "gateway": "10.0.1.1",
  "provision_start_ip": "10.0.1.2",
  "provision_end_ip": "10.0.1.254",
  "internet_nat": true,
  "resolvers": ["8.8.8.8", "8.8.4.4"],
  "description": "Web tier network"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | yes | Network name |
| `subnet` | string | yes | CIDR notation subnet |
| `gateway` | string | no | Gateway IP (must be within subnet) |
| `provision_start_ip` | string | yes | First IP available for provisioning |
| `provision_end_ip` | string | yes | Last IP available for provisioning |
| `internet_nat` | boolean | no | Enable outbound internet NAT (default: `true`) |
| `resolvers` | array[string] | no | DNS resolvers |
| `routes` | object | no | Static routes (destination CIDR -> gateway) |
| `description` | string | no | Human-readable description |

**Response:** `201 Created` -- returns the full network object.

### GetFabricNetwork

```
GET /:login/fabrics/default/vlans/:vlan_id/networks/:net_id
```

**Parameters:**

| Name | In | Type | Description |
|------|----|------|-------------|
| `vlan_id` | path | integer | VLAN ID |
| `net_id` | path | UUID | Network ID |

**Response:** `200 OK` -- returns the full network object.

### UpdateFabricNetwork

Update properties of an existing fabric network.

```
PUT /:login/fabrics/default/vlans/:vlan_id/networks/:net_id
```

**Request body:** Any combination of mutable fields:

```json
{
  "description": "Updated description",
  "routes": {"10.0.2.0/24": "10.0.1.1"},
  "resolvers": ["1.1.1.1", "8.8.8.8"]
}
```

**Response:** `200 OK` -- returns the updated network object.

### DeleteFabricNetwork

Delete a fabric network. The network must have no instances provisioned on it.

```
DELETE /:login/fabrics/default/vlans/:vlan_id/networks/:net_id
```

**Response:** `204 No Content`

---

## NICs (Network Interfaces)

NICs represent network interfaces attached to an instance. Every instance has at least one NIC assigned at creation. Additional NICs can be added post-creation to connect an instance to multiple networks.

### ListNics

```
GET /:login/machines/:id/nics
```

**Parameters:**

| Name | In | Type | Description |
|------|----|------|-------------|
| `id` | path | UUID | Instance ID |

**Response:** `200 OK`

```json
[
  {
    "ip": "10.0.1.5",
    "mac": "90:b8:d0:2e:f1:01",
    "primary": true,
    "netmask": "255.255.255.0",
    "gateway": "10.0.1.1",
    "network": "a1c2e3f4-5678-9abc-def0-1234567890ab",
    "state": "running"
  },
  {
    "ip": "10.0.2.10",
    "mac": "90:b8:d0:2e:f1:02",
    "primary": false,
    "netmask": "255.255.255.0",
    "gateway": "10.0.2.1",
    "network": "b2d3f4a5-6789-abcd-ef01-234567890abc",
    "state": "running"
  }
]
```

### GetNic

```
GET /:login/machines/:id/nics/:mac
```

**Parameters:**

| Name | In | Type | Description |
|------|----|------|-------------|
| `id` | path | UUID | Instance ID |
| `mac` | path | string | MAC address (colon-separated) |

**Response:** `200 OK`

```json
{
  "ip": "10.0.1.5",
  "mac": "90:b8:d0:2e:f1:01",
  "primary": true,
  "netmask": "255.255.255.0",
  "gateway": "10.0.1.1",
  "network": "a1c2e3f4-5678-9abc-def0-1234567890ab",
  "state": "running"
}
```

### AddNic

Add a new NIC to an existing instance, connecting it to an additional network. The instance must be stopped or support hot-add.

```
POST /:login/machines/:id/nics
```

**Request body:**

```json
{
  "network": "b2d3f4a5-6789-abcd-ef01-234567890abc"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network` | UUID | yes | Network ID to attach |

**Response:** `201 Created`

```json
{
  "ip": "10.0.2.10",
  "mac": "90:b8:d0:2e:f1:02",
  "primary": false,
  "netmask": "255.255.255.0",
  "gateway": "10.0.2.1",
  "network": "b2d3f4a5-6789-abcd-ef01-234567890abc",
  "state": "provisioning"
}
```

### RemoveNic

Remove a NIC from an instance. The primary NIC cannot be removed.

```
DELETE /:login/machines/:id/nics/:mac
```

**Response:** `204 No Content`

### NIC Object

| Field | Type | Description |
|-------|------|-------------|
| `ip` | string | IP address assigned to this interface |
| `mac` | string | MAC address (colon-separated hex) |
| `primary` | boolean | Whether this is the primary NIC |
| `netmask` | string | Subnet mask |
| `gateway` | string | Gateway IP address |
| `network` | UUID | Network ID this NIC is attached to |
| `state` | string | NIC state (`provisioning`, `running`, `stopped`) |

---

## Example: 3-Tier Network Architecture

A common pattern isolates web, API, and database tiers on separate VLANs. Each tier gets its own subnet with appropriate internet access settings.

```
                    Internet
                       |
              [Public Network]
                       |
         +-------------+-------------+
         |                           |
   +-----------+              +-----------+
   | Web Tier  |              |    LB     |
   | VLAN 100  |              | (public)  |
   | 10.0.1/24 |              +-----------+
   +-----+-----+
         |
   +-----------+
   | API Tier  |
   | VLAN 200  |
   | 10.0.2/24 |
   +-----+-----+
         |
   +-----------+
   | DB Tier   |
   | VLAN 300  |
   | 10.0.3/24 |
   +-----------+
```

### Step 1: Create VLANs

```
POST /:login/fabrics/default/vlans
{"vlan_id": 100, "name": "web-vlan", "description": "Web/frontend tier"}

POST /:login/fabrics/default/vlans
{"vlan_id": 200, "name": "api-vlan", "description": "API/application tier"}

POST /:login/fabrics/default/vlans
{"vlan_id": 300, "name": "db-vlan", "description": "Database tier"}
```

### Step 2: Create Fabric Networks

**Web network** -- internet NAT enabled for outbound access (package updates, CDN, etc.):

```
POST /:login/fabrics/default/vlans/100/networks
{
  "name": "web-network",
  "subnet": "10.0.1.0/24",
  "gateway": "10.0.1.1",
  "provision_start_ip": "10.0.1.2",
  "provision_end_ip": "10.0.1.254",
  "internet_nat": true,
  "resolvers": ["8.8.8.8", "8.8.4.4"],
  "description": "Web tier network"
}
```

**API network** -- internet NAT enabled for calling external APIs:

```
POST /:login/fabrics/default/vlans/200/networks
{
  "name": "api-network",
  "subnet": "10.0.2.0/24",
  "gateway": "10.0.2.1",
  "provision_start_ip": "10.0.2.2",
  "provision_end_ip": "10.0.2.254",
  "internet_nat": true,
  "resolvers": ["8.8.8.8", "8.8.4.4"],
  "description": "API tier network"
}
```

**Database network** -- internet NAT **disabled** for maximum isolation:

```
POST /:login/fabrics/default/vlans/300/networks
{
  "name": "db-network",
  "subnet": "10.0.3.0/24",
  "gateway": "10.0.3.1",
  "provision_start_ip": "10.0.3.2",
  "provision_end_ip": "10.0.3.254",
  "internet_nat": false,
  "resolvers": [],
  "description": "Database tier network - no internet access"
}
```

### Step 3: Provision Instances on Each Tier

```
POST /:login/machines
{
  "name": "web-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<web-network-uuid>", "<public-network-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "web:443"
}
```

```
POST /:login/machines
{
  "name": "api-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<api-network-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "api:8080"
}
```

```
POST /:login/machines
{
  "name": "db-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<db-network-uuid>"],
  "firewall_enabled": true
}
```

Note: The web instance has both a fabric NIC and a public NIC for direct internet-facing traffic. The API instance has only a fabric NIC with NAT for outbound access. The database instance has only a fabric NIC with no NAT, fully isolated from the internet.

### Step 4: Add Cross-Tier Connectivity

If an API instance needs to reach the database tier, add a NIC on the database network:

```
POST /:login/machines/<api-01-uuid>/nics
{
  "network": "<db-network-uuid>"
}
```

After this, the API instance has two NICs: one on `10.0.2.x` (API tier) and one on `10.0.3.x` (DB tier), allowing it to communicate directly with database instances.

### Step 5: Reserve IPs for Critical Services

Reserve a known IP for the primary database before provisioning:

```
PUT /:login/networks/<db-network-uuid>/ips/10.0.3.10
{
  "reserved": true
}
```

Then provision the database instance and it will not receive `10.0.3.10` via auto-assignment, leaving it available for manual or planned assignment.

---

## Error Codes

| HTTP Status | Meaning |
|-------------|---------|
| `400 Bad Request` | Invalid parameters (e.g., malformed subnet, overlapping IP range) |
| `404 Not Found` | Network, VLAN, NIC, or IP not found |
| `409 Conflict` | VLAN ID already exists, or network has active instances (cannot delete) |
| `422 Unprocessable Entity` | Semantic error (e.g., gateway outside subnet, primary NIC removal) |
