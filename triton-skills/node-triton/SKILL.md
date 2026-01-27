# Triton CLI Infrastructure Management

A skill for managing Triton DataCenter infrastructure using the `triton` CLI. Create instances, configure networks, set up CNS service discovery, manage firewall rules, and deploy load balancers.

## When to Use This Skill

Use this skill when:
- Creating, managing, or deleting Triton compute instances
- Configuring networking (VLANs, fabric networks, NICs)
- Setting up CNS service discovery with instance tags
- Managing firewall rules for security
- Deploying Triton-Moirai load balancers
- Creating and mounting volumes for persistent storage
- Managing SSH keys and account settings
- Scaling infrastructure horizontally

## Prerequisites

### 1. Profile Configuration
Before using any commands, ensure a profile is configured:

```bash
# List existing profiles
triton profile list

# Create a new profile (interactive)
triton profile create

# Set current profile
triton profile set-current <profile-name>

# Get current profile info
triton profile get
```

### 2. Enable CNS (Container Name Service)
CNS must be enabled on the account for DNS-based service discovery:

```bash
triton account update triton_cns_enabled=true
```

### 3. Note Your Account UUID
Many CNS DNS names include your account UUID:

```bash
triton account get
# Note the "id" field - this is your account UUID
```

## Core Command Categories

### Instance Management
Create and manage compute instances (VMs/containers):
- `triton instance create` - Create new instances
- `triton instance list` - List all instances
- `triton instance get` - Get instance details
- `triton instance delete` - Delete instances
- `triton instance start/stop/reboot` - Control instance state
- `triton instance ssh` - SSH to an instance
- `triton instance tag set` - Set instance tags (including CNS tags)
- `triton instance metadata set` - Set instance metadata

### Networking
Configure networks, VLANs, and firewall rules:
- `triton network list` - List available networks
- `triton vlan list/create` - Manage fabric VLANs
- `triton fwrule create` - Create firewall rules
- `triton instance nic create` - Add network interfaces

### Images and Packages
Select OS images and instance sizes:
- `triton image list` - List available images
- `triton package list` - List available instance sizes

### Storage
Manage persistent volumes:
- `triton volume create` - Create NFS volumes
- `triton volume list` - List volumes

### Account and Profiles
Manage account settings and CLI profiles:
- `triton account get/update` - View/update account settings
- `triton profile list/create/set-current` - Manage CLI profiles

## Quick Reference

### Create an Instance with CNS Service
```bash
triton instance create \
  -n my-web-server \
  -t triton.cns.services=web:8080 \
  -t env=production \
  -t role=web \
  --firewall \
  -w \
  base-64-lts g4-highcpu-1G
```

### Deploy a Load Balancer
```bash
triton instance create \
  -n my-lb \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=https-http://443:web.svc.ACCOUNT_UUID.DC.cns.mnx.io:8080{check:/healthz}" \
  -m cloud.tritoncompute:certificate_name=example.com \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

### Create a Firewall Rule
```bash
triton fwrule create -D "allow-https" \
  "FROM any TO tag role=web ALLOW tcp PORT 443"
```

## Documentation Structure

- **[commands/REFERENCE.md](commands/REFERENCE.md)** - Complete CLI command reference
- **[patterns/cns-service-discovery.md](patterns/cns-service-discovery.md)** - CNS tagging and DNS patterns
- **[patterns/load-balancing.md](patterns/load-balancing.md)** - Triton-Moirai load balancer setup
- **[patterns/firewall-rules.md](patterns/firewall-rules.md)** - Firewall rule patterns
- **[patterns/networking.md](patterns/networking.md)** - Network architecture patterns
- **[workflows/deploy-web-app.md](workflows/deploy-web-app.md)** - Complete 3-tier deployment example

## Key Best Practices

1. **Always enable firewall** on production instances with `--firewall`
2. **Use CNS tags** for automatic service discovery: `triton.cns.services=service:port`
3. **Use tag-based firewall rules** for dynamic security: `tag role=web`
4. **Put databases on private networks** - never expose directly to public
5. **Use Triton-Moirai** for load balancing with TLS termination
6. **Graceful scaling**: set `triton.cns.status=down` before removing instances
