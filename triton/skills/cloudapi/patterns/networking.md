# Networking Patterns

Triton provides fabric networking with VLANs, private networks, and flexible IP management for building secure, isolated infrastructure. All networking operations are available through the CloudAPI REST interface.

## Network Types

### Public Networks
- Routable from the internet
- Instances get public IP addresses
- Suitable for: load balancers, bastion hosts
- Returned by `GET /:login/networks` with `"public": true`

### Fabric Networks (Private)
- Isolated virtual networks within your account
- Not routable from the internet (unless `internet_nat` is enabled)
- Suitable for: databases, internal services, inter-tier communication
- Created under a VLAN via `POST /:login/fabrics/default/vlans/:vlan_id/networks`

## VLAN Management

VLANs provide Layer 2 isolation for fabric networks.

### List VLANs

```
GET /:login/fabrics/default/vlans
```

**Response:**
```json
[
  {
    "vlan_id": 100,
    "name": "production",
    "description": "Production tier networks"
  },
  {
    "vlan_id": 200,
    "name": "staging",
    "description": "Staging environment"
  }
]
```

### Create VLAN

```
POST /:login/fabrics/default/vlans
```

```json
{
  "vlan_id": 100,
  "name": "production",
  "description": "Production tier networks"
}
```

### Get VLAN

```
GET /:login/fabrics/default/vlans/100
```

### Delete VLAN

```
DELETE /:login/fabrics/default/vlans/100
```

A VLAN cannot be deleted while it still has networks attached.

### List Networks on VLAN

```
GET /:login/fabrics/default/vlans/100/networks
```

## Fabric Network Management

### List All Networks

```
GET /:login/networks
```

Returns both public and fabric networks. Use the `public` field to distinguish between them.

**Response:**
```json
[
  {
    "id": "7326787b-8039-436c-a533-5038f6f3f750",
    "name": "external",
    "public": true,
    "fabric": false,
    "subnet": "142.147.4.0/24",
    "gateway": "142.147.4.1"
  },
  {
    "id": "a9c130da-e3ba-40e6-b50c-6bed9521bbd4",
    "name": "internal-prod",
    "public": false,
    "fabric": true,
    "vlan_id": 100,
    "subnet": "10.10.0.0/24",
    "provision_start_ip": "10.10.0.10",
    "provision_end_ip": "10.10.0.250",
    "gateway": "10.10.0.1",
    "internet_nat": true
  }
]
```

### Create Fabric Network

```
POST /:login/fabrics/default/vlans/:vlan_id/networks
```

**Basic network:**
```json
{
  "name": "internal-prod",
  "subnet": "10.10.0.0/24",
  "provision_start_ip": "10.10.0.10",
  "provision_end_ip": "10.10.0.250",
  "gateway": "10.10.0.1",
  "internet_nat": true
}
```

**Network fields:**

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Network name |
| `subnet` | Yes | CIDR subnet (e.g., `10.10.0.0/24`) |
| `provision_start_ip` | Yes | First IP to assign to instances |
| `provision_end_ip` | Yes | Last IP to assign to instances |
| `gateway` | No | Gateway IP address |
| `resolvers` | No | Array of DNS resolver IPs |
| `routes` | No | Object of static routes (`"destination": "gateway"`) |
| `internet_nat` | No | Enable outbound NAT to internet (default: `true`) |
| `description` | No | Human-readable description |

### Network with Custom DNS

```
POST /:login/fabrics/default/vlans/100/networks
```

```json
{
  "name": "internal-prod",
  "subnet": "10.10.0.0/24",
  "provision_start_ip": "10.10.0.10",
  "provision_end_ip": "10.10.0.250",
  "gateway": "10.10.0.1",
  "resolvers": ["10.10.0.2", "8.8.8.8"],
  "internet_nat": true
}
```

### Network with Static Routes

```
POST /:login/fabrics/default/vlans/100/networks
```

```json
{
  "name": "internal-prod",
  "subnet": "10.10.0.0/24",
  "provision_start_ip": "10.10.0.10",
  "provision_end_ip": "10.10.0.250",
  "gateway": "10.10.0.1",
  "routes": {
    "10.20.0.0/24": "10.10.0.254",
    "10.30.0.0/24": "10.10.0.254"
  }
}
```

### Network without NAT (Database Tier)

Set `internet_nat` to `false` for networks that should have no outbound internet access. This is the correct approach for database tiers and internal-only services.

```
POST /:login/fabrics/default/vlans/300/networks
```

```json
{
  "name": "db-net",
  "subnet": "10.30.0.0/24",
  "provision_start_ip": "10.30.0.10",
  "provision_end_ip": "10.30.0.250",
  "internet_nat": false
}
```

### Get Network Details

```
GET /:login/networks/:network_id
```

### Delete Fabric Network

```
DELETE /:login/fabrics/default/vlans/:vlan_id/networks/:network_id
```

A network cannot be deleted while instances are attached to it.

## Instance Network Configuration

### Create Instance on Specific Network

Attach an instance to one or more networks by passing their UUIDs in the `networks` array.

```
POST /:login/machines
```

```json
{
  "name": "db-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<internal-prod-network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "db",
  "tag.triton.cns.services": "postgres:5432"
}
```

### Multiple Networks

Attach an instance to both a public network and a private fabric network. The instance will receive an IP on each network.

```
POST /:login/machines
```

```json
{
  "name": "web-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": [
    "<external-network-uuid>",
    "<internal-prod-network-uuid>"
  ],
  "firewall_enabled": true,
  "tag.role": "web",
  "tag.triton.cns.services": "web:8080"
}
```

### Private Network Only (No Public IP)

Omit the public network UUID from the `networks` array. The instance will only be reachable from other instances on the same fabric network.

```
POST /:login/machines
```

```json
{
  "name": "db-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<internal-prod-network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "db"
}
```

## NIC Management

### List Instance NICs

```
GET /:login/machines/:machine_id/nics
```

**Response:**
```json
[
  {
    "mac": "90:b8:d0:aa:bb:cc",
    "primary": true,
    "ip": "10.10.0.15",
    "netmask": "255.255.255.0",
    "gateway": "10.10.0.1",
    "state": "running",
    "network": "<network-uuid>"
  }
]
```

### Add NIC to Running Instance

```
POST /:login/machines/:machine_id/nics
```

```json
{
  "network": "<network-uuid>"
}
```

### Remove NIC

```
DELETE /:login/machines/:machine_id/nics/:mac_address
```

### Get NIC Details

```
GET /:login/machines/:machine_id/nics/:mac_address
```

## Network Architecture Patterns

### Three-Tier Architecture

```
                    Internet
                        |
                  +-----+-----+
                  |  Public   |
                  |  Network  |
                  +-----+-----+
                        |
              +---------+---------+
              |   Load Balancer   |
              |   (role=lb)       |
              +---------+---------+
                        |
                  +-----+-----+
                  |  Web VLAN |
                  |  (100)    |
                  +-----+-----+
                        |
         +--------------+--------------+
         |              |              |
    +----+----+   +----+----+   +----+----+
    | web-01  |   | web-02  |   | web-03  |
    | (role=  |   | (role=  |   | (role=  |
    |  web)   |   |  web)   |   |  web)   |
    +----+----+   +----+----+   +----+----+
         |              |              |
         +--------------+--------------+
                        |
                  +-----+-----+
                  | API VLAN  |
                  |  (200)    |
                  +-----+-----+
                        |
         +--------------+--------------+
         |              |              |
    +----+----+   +----+----+   +----+----+
    | api-01  |   | api-02  |   | api-03  |
    | (role=  |   | (role=  |   | (role=  |
    |  api)   |   |  api)   |   |  api)   |
    +----+----+   +----+----+   +----+----+
         |              |              |
         +--------------+--------------+
                        |
                  +-----+-----+
                  |  DB VLAN  |
                  |  (300)    |
                  +-----+-----+
                        |
              +---------+---------+
              |      db-01        |
              |    (role=db)      |
              +-------------------+
```

### Implementation

**Step 1: Create VLANs**

```
POST /:login/fabrics/default/vlans
```

Web VLAN:
```json
{ "vlan_id": 100, "name": "web", "description": "Web tier" }
```

API VLAN:
```json
{ "vlan_id": 200, "name": "api", "description": "API tier" }
```

DB VLAN:
```json
{ "vlan_id": 300, "name": "db", "description": "Database tier" }
```

**Step 2: Create Networks**

Web network:
```
POST /:login/fabrics/default/vlans/100/networks
```
```json
{
  "name": "web-net",
  "subnet": "10.100.0.0/24",
  "provision_start_ip": "10.100.0.10",
  "provision_end_ip": "10.100.0.250",
  "gateway": "10.100.0.1",
  "internet_nat": true
}
```

API network:
```
POST /:login/fabrics/default/vlans/200/networks
```
```json
{
  "name": "api-net",
  "subnet": "10.200.0.0/24",
  "provision_start_ip": "10.200.0.10",
  "provision_end_ip": "10.200.0.250",
  "gateway": "10.200.0.1",
  "internet_nat": true
}
```

DB network (no NAT -- no internet access):
```
POST /:login/fabrics/default/vlans/300/networks
```
```json
{
  "name": "db-net",
  "subnet": "10.30.0.0/24",
  "provision_start_ip": "10.30.0.10",
  "provision_end_ip": "10.30.0.250",
  "internet_nat": false
}
```

**Step 3: Create Instances**

Load balancer on public + web network:
```
POST /:login/machines
```
```json
{
  "name": "lb-01",
  "image": "<cloud-load-balancer-image-uuid>",
  "package": "<package-uuid>",
  "networks": [
    "<external-network-uuid>",
    "<web-net-uuid>"
  ],
  "firewall_enabled": true,
  "tag.role": "lb",
  "metadata.cloud.tritoncompute:loadbalancer": "true",
  "metadata.cloud.tritoncompute:portmap": "https-http://443:web.svc.ACCOUNT.DC.cns.PROVIDER.zone:8080{check:/healthz}"
}
```

Web tier on web + API networks:
```
POST /:login/machines
```
```json
{
  "name": "web-01",
  "image": "<base-64-lts-image-uuid>",
  "package": "<package-uuid>",
  "networks": [
    "<web-net-uuid>",
    "<api-net-uuid>"
  ],
  "firewall_enabled": true,
  "tag.role": "web",
  "tag.triton.cns.services": "web:8080"
}
```

API tier on API + DB networks:
```
POST /:login/machines
```
```json
{
  "name": "api-01",
  "image": "<base-64-lts-image-uuid>",
  "package": "<package-uuid>",
  "networks": [
    "<api-net-uuid>",
    "<db-net-uuid>"
  ],
  "firewall_enabled": true,
  "tag.role": "api",
  "tag.triton.cns.services": "api:3000"
}
```

DB on DB network only (no public, no NAT):
```
POST /:login/machines
```
```json
{
  "name": "db-01",
  "image": "<base-64-lts-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<db-net-uuid>"],
  "firewall_enabled": true,
  "tag.role": "db",
  "tag.triton.cns.services": "postgres:5432"
}
```

### Management / Bastion Network Pattern

A separate network for SSH and management access. Only the bastion host has a public IP; all other instances are reached through it.

**Create management VLAN and network:**

```
POST /:login/fabrics/default/vlans
```
```json
{ "vlan_id": 10, "name": "mgmt", "description": "Management and SSH access" }
```

```
POST /:login/fabrics/default/vlans/10/networks
```
```json
{
  "name": "mgmt-net",
  "subnet": "10.1.0.0/24",
  "provision_start_ip": "10.1.0.10",
  "provision_end_ip": "10.1.0.250",
  "gateway": "10.1.0.1",
  "internet_nat": true
}
```

**Create bastion host (public + management networks):**

```
POST /:login/machines
```
```json
{
  "name": "bastion-01",
  "image": "<base-64-lts-image-uuid>",
  "package": "<package-uuid>",
  "networks": [
    "<external-network-uuid>",
    "<mgmt-net-uuid>"
  ],
  "firewall_enabled": true,
  "tag.role": "bastion"
}
```

**Attach all other instances to the management network as a secondary NIC:**

```
POST /:login/machines
```
```json
{
  "name": "web-01",
  "image": "<base-64-lts-image-uuid>",
  "package": "<package-uuid>",
  "networks": [
    "<web-net-uuid>",
    "<mgmt-net-uuid>"
  ],
  "firewall_enabled": true,
  "tag.role": "web"
}
```

**Restrict SSH access via firewall rule:**

```
POST /:login/fwrules
```
```json
{
  "rule": "FROM tag role = bastion TO all vms ALLOW tcp PORT 22",
  "enabled": true,
  "description": "Only bastion can SSH to other instances"
}
```

### When to Disable `internet_nat`

Set `"internet_nat": false` on fabric networks for:

- **Database networks** -- databases should not initiate outbound connections to the internet
- **Internal-only services** -- caches, message queues, and internal APIs that have no reason to reach the internet
- **High-security tiers** -- any tier where outbound exfiltration should be prevented at the network level
- **Compliance requirements** -- environments where audit controls require strict egress control

Instances on networks without NAT can still communicate with other instances on the same or peered fabric networks. They simply cannot reach the public internet.

## Best Practices

1. **Isolate the database tier** -- No public network, no NAT, only accessible from the API tier

2. **Use VLANs for tier separation** -- Each application tier on its own VLAN prevents lateral movement

3. **Limit public network exposure** -- Only load balancers and bastion hosts should have public IPs

4. **Use a management network** -- Separate network for SSH access via bastion host

5. **Disable NAT for internal services** -- Use `"internet_nat": false` for networks that should not reach the internet

6. **Plan IP addressing** -- Use consistent subnet schemes:
   - `10.100.x.x` for web tier
   - `10.200.x.x` for API tier
   - `10.30.x.x` for database tier
   - `10.1.x.x` for management

7. **Document network topology** -- Maintain diagrams of VLAN and network relationships

8. **Resolve network UUIDs before instance creation** -- List networks with `GET /:login/networks` to obtain the UUID for each network before passing them in the `networks` array

## Troubleshooting

### Instance Cannot Reach the Internet
1. Check if the fabric network has `"internet_nat": true`
2. Verify a gateway is configured on the network
3. Check firewall rules allow outbound traffic

### Instances Cannot Communicate
1. Verify both instances are on the same fabric network or VLAN
2. Check firewall rules allow traffic between them (tag-based rules work well here)
3. Verify IPs are in the same subnet -- use `GET /:login/machines/:id/nics` to inspect

### Cannot Create a Network
1. Verify the VLAN exists: `GET /:login/fabrics/default/vlans/:vlan_id`
2. Check the subnet does not overlap with existing networks on the same VLAN
3. Ensure `provision_start_ip` and `provision_end_ip` are within the subnet range

### Cannot Delete a Network
1. Ensure no instances are still attached to the network
2. Remove all NICs from the network first, then retry the delete
