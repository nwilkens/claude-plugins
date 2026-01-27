# Networking Patterns

Triton provides fabric networking with VLANs, private networks, and flexible IP management for building secure, isolated infrastructure.

## Network Types

### Public Networks
- Routable from the internet
- Instances get public IP addresses
- Suitable for: load balancers, bastion hosts

### Fabric Networks (Private)
- Isolated virtual networks within your account
- Not routable from internet (unless NAT enabled)
- Suitable for: databases, internal services, inter-tier communication

## VLAN Management

VLANs provide Layer 2 isolation for fabric networks.

### List VLANs
```bash
triton vlan list
```

### Create VLAN
```bash
triton vlan create --name production 100
triton vlan create --name staging 200
triton vlan create --name management 10
```

### Delete VLAN
```bash
triton vlan delete 100
```

### List Networks on VLAN
```bash
triton vlan networks 100
```

## Fabric Network Management

### List Networks
```bash
triton network list

# Show only fabric networks
triton network list fabric=true

# Show only public networks
triton network list public=true
```

### Create Network
```bash
triton network create \
  --name internal-prod \
  --vlan-id 100 \
  --subnet 10.10.0.0/24 \
  --start-ip 10.10.0.10 \
  --end-ip 10.10.0.250 \
  --gateway 10.10.0.1
```

**Options:**
| Option | Description |
|--------|-------------|
| `--name` | Network name |
| `--vlan-id` | VLAN to create network on |
| `--subnet` | CIDR subnet |
| `--start-ip` | First usable IP |
| `--end-ip` | Last usable IP |
| `--gateway` | Gateway IP (optional) |
| `--resolver` | DNS resolver (repeatable) |
| `--no-nat` | Disable NAT to internet |
| `--route` | Static route (repeatable) |

### Network with Custom DNS
```bash
triton network create \
  --name internal-prod \
  --vlan-id 100 \
  --subnet 10.10.0.0/24 \
  --start-ip 10.10.0.10 \
  --end-ip 10.10.0.250 \
  --gateway 10.10.0.1 \
  --resolver 10.10.0.2 \
  --resolver 8.8.8.8
```

### Network with Static Routes
```bash
triton network create \
  --name internal-prod \
  --vlan-id 100 \
  --subnet 10.10.0.0/24 \
  --start-ip 10.10.0.10 \
  --end-ip 10.10.0.250 \
  --gateway 10.10.0.1 \
  --route 10.20.0.0/24=10.10.0.254 \
  --route 10.30.0.0/24=10.10.0.254
```

### Delete Network
```bash
triton network delete internal-prod
```

### Set Default Network
```bash
triton network set-default internal-prod
triton network get-default
```

## Instance Network Configuration

### Create Instance on Specific Network
```bash
triton instance create \
  -n db-01 \
  -N internal-prod \
  -w \
  base-64-lts g4-highmem-4G
```

### Multiple Networks
```bash
triton instance create \
  -n web-01 \
  -N external \
  -N internal-prod \
  -w \
  base-64-lts g4-highcpu-1G
```

### Private Network Only (No Public IP)
```bash
triton instance create \
  -n db-01 \
  -N internal-prod \
  -w \
  base-64-lts g4-highmem-4G
```

## NIC Management

### List Instance NICs
```bash
triton instance nic list myinstance
```

### Add NIC to Running Instance
```bash
triton instance nic create -w myinstance internal-prod
```

### Remove NIC
```bash
triton instance nic delete myinstance MAC_ADDRESS
```

### Get NIC Details
```bash
triton instance nic get myinstance MAC_ADDRESS
```

## Network Architecture Patterns

### Three-Tier Architecture

```
                    Internet
                        │
                  ┌─────┴─────┐
                  │  Public   │
                  │  Network  │
                  └─────┬─────┘
                        │
              ┌─────────┴─────────┐
              │   Load Balancer   │
              │   (role=lb)       │
              └─────────┬─────────┘
                        │
                  ┌─────┴─────┐
                  │  Web VLAN │
                  │  (100)    │
                  └─────┬─────┘
                        │
         ┌──────────────┼──────────────┐
         │              │              │
    ┌────┴────┐   ┌────┴────┐   ┌────┴────┐
    │ web-01  │   │ web-02  │   │ web-03  │
    │ (role=  │   │ (role=  │   │ (role=  │
    │  web)   │   │  web)   │   │  web)   │
    └────┬────┘   └────┬────┘   └────┬────┘
         │              │              │
         └──────────────┼──────────────┘
                        │
                  ┌─────┴─────┐
                  │ API VLAN  │
                  │  (200)    │
                  └─────┬─────┘
                        │
         ┌──────────────┼──────────────┐
         │              │              │
    ┌────┴────┐   ┌────┴────┐   ┌────┴────┐
    │ api-01  │   │ api-02  │   │ api-03  │
    │ (role=  │   │ (role=  │   │ (role=  │
    │  api)   │   │  api)   │   │  api)   │
    └────┬────┘   └────┬────┘   └────┬────┘
         │              │              │
         └──────────────┼──────────────┘
                        │
                  ┌─────┴─────┐
                  │  DB VLAN  │
                  │  (300)    │
                  └─────┬─────┘
                        │
              ┌─────────┴─────────┐
              │      db-01        │
              │    (role=db)      │
              └───────────────────┘
```

### Implementation

```bash
# Create VLANs
triton vlan create --name web 100
triton vlan create --name api 200
triton vlan create --name db 300

# Create Networks
triton network create \
  --name web-net \
  --vlan-id 100 \
  --subnet 10.100.0.0/24 \
  --start-ip 10.100.0.10 \
  --end-ip 10.100.0.250 \
  --gateway 10.100.0.1

triton network create \
  --name api-net \
  --vlan-id 200 \
  --subnet 10.200.0.0/24 \
  --start-ip 10.200.0.10 \
  --end-ip 10.200.0.250 \
  --gateway 10.200.0.1

triton network create \
  --name db-net \
  --vlan-id 300 \
  --subnet 10.30.0.0/24 \
  --start-ip 10.30.0.10 \
  --end-ip 10.30.0.250 \
  --no-nat

# Create instances
# LB on public + web network
triton instance create \
  -n lb-01 \
  -N external \
  -N web-net \
  -t role=lb \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G

# Web tier on web + api network
triton instance create \
  -n web-01 \
  -N web-net \
  -N api-net \
  -t role=web \
  -t triton.cns.services=web:8080 \
  --firewall \
  -w \
  base-64-lts g4-highcpu-1G

# API tier on api + db network
triton instance create \
  -n api-01 \
  -N api-net \
  -N db-net \
  -t role=api \
  -t triton.cns.services=api:3000 \
  --firewall \
  -w \
  base-64-lts g4-highcpu-2G

# DB on db network only
triton instance create \
  -n db-01 \
  -N db-net \
  -t role=db \
  -t triton.cns.services=postgres:5432 \
  --firewall \
  -w \
  base-64-lts g4-highmem-8G
```

### Management Network Pattern

Separate network for SSH/management access:

```bash
# Management VLAN
triton vlan create --name mgmt 10

# Management network
triton network create \
  --name mgmt-net \
  --vlan-id 10 \
  --subnet 10.1.0.0/24 \
  --start-ip 10.1.0.10 \
  --end-ip 10.1.0.250 \
  --gateway 10.1.0.1

# Bastion host
triton instance create \
  -n bastion-01 \
  -N external \
  -N mgmt-net \
  -t role=bastion \
  --firewall \
  -w \
  base-64-lts g4-highcpu-512M

# All other instances on mgmt network for SSH
triton instance create \
  -n web-01 \
  -N web-net \
  -N mgmt-net \
  -t role=web \
  --firewall \
  -w \
  base-64-lts g4-highcpu-1G
```

Firewall rules:
```bash
# Only bastion can SSH to other instances via mgmt network
triton fwrule create -D "ssh-bastion" \
  "FROM tag role=bastion TO all vms ALLOW tcp PORT 22"
```

## Best Practices

1. **Isolate database tier** - No public network, no NAT, only accessible from API tier

2. **Use VLANs for tier separation** - Each application tier on its own VLAN

3. **Limit public network exposure** - Only load balancers and bastion hosts

4. **Use management network** - Separate network for SSH access via bastion

5. **No NAT for internal services** - Use `--no-nat` for networks that don't need internet access

6. **Plan IP addressing** - Use consistent subnet schemes:
   - 10.100.x.x for web tier
   - 10.200.x.x for API tier
   - 10.30.x.x for database tier
   - 10.1.x.x for management

7. **Document network topology** - Maintain diagrams of VLAN and network relationships

## Troubleshooting

### Instance Can't Reach Internet
1. Check if network has NAT enabled
2. Verify gateway is configured
3. Check firewall rules allow outbound traffic

### Instances Can't Communicate
1. Verify both instances are on same network/VLAN
2. Check firewall rules allow traffic between them
3. Verify IPs are in same subnet

### Can't Create Network
1. Verify VLAN exists
2. Check subnet doesn't overlap with existing networks
3. Ensure start/end IPs are within subnet range
