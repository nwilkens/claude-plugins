# Triton CloudAPI Skill

Manage Triton DataCenter infrastructure using the CloudAPI REST interface. Create instances, configure fabric networks, set up CNS service discovery, manage firewall rules, and deploy load balancers — all via HTTP.

## When to Use This Skill

- Creating or managing compute instances (VMs/containers)
- Setting up fabric networks and VLANs for multi-tier applications
- Configuring firewall rules for security
- Using CNS for automatic DNS-based service discovery
- Deploying load balancers with TLS termination (Triton-Moirai)
- Provisioning instances with user-scripts and metadata
- Managing images, packages, volumes, snapshots, and SSH keys

## Authentication

CloudAPI uses **HTTP Signature authentication** with SSH keys.

**Headers:**
```
Authorization: Signature keyId="/<account>/keys/<key-fingerprint>",
  algorithm="rsa-sha256",
  headers="date",
  signature="<base64-signature>"
Date: <RFC 1123 date>
```

All endpoints are prefixed with `/<account>/` where `<account>` is the login name.

**Content-Type:** `application/json` for all request/response bodies.

**API Versioning:** Set via `Accept-Version` header (e.g., `~9`).

## Base URL

```
https://<cloudapi-host>/<account>/
```

The Triton Portal backend proxies all CloudAPI requests, handling authentication automatically via the configured operator key.

## Core API Endpoints

| Resource | Endpoint Prefix | Reference |
|----------|----------------|-----------|
| Instances | `/:login/machines` | [api/instances.md](api/instances.md) |
| Networks & Fabrics | `/:login/networks`, `/:login/fabrics` | [api/networking.md](api/networking.md) |
| Firewall Rules | `/:login/fwrules` | [api/firewall.md](api/firewall.md) |
| Images | `/:login/images` | [api/images.md](api/images.md) |
| Packages | `/:login/packages` | [api/packages.md](api/packages.md) |
| Volumes | `/:login/volumes` | [api/volumes.md](api/volumes.md) |
| SSH Keys | `/:login/keys` | (standard CRUD) |
| Snapshots | `/:login/machines/:id/snapshots` | (standard CRUD) |
| Account | `/:login` | (GET/POST) |
| Datacenters | `/:login/datacenters` | (GET) |

## Infrastructure Patterns

| Pattern | Reference |
|---------|-----------|
| Networking (VLANs, fabrics, multi-tier) | [patterns/networking.md](patterns/networking.md) |
| CNS Service Discovery | [patterns/cns-service-discovery.md](patterns/cns-service-discovery.md) |
| Firewall Rule Strategies | [patterns/firewall-strategies.md](patterns/firewall-strategies.md) |
| Load Balancing (Triton-Moirai) | [patterns/load-balancing.md](patterns/load-balancing.md) |
| Metadata Provisioning | [patterns/metadata-provisioning.md](patterns/metadata-provisioning.md) |

## Deployment Workflows

| Workflow | Reference |
|----------|-----------|
| 3-Tier Web Application | [workflows/deploy-web-app.md](workflows/deploy-web-app.md) |
| Kubernetes (k3s) Cluster | [workflows/deploy-kubernetes.md](workflows/deploy-kubernetes.md) |

## Quick Reference

### Create an Instance
```
POST /:login/machines
{
  "name": "my-app-01",
  "image": "<image-uuid>",
  "package": "<package-uuid-or-name>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.triton.cns.services": "myapp:8080"
}
```

### Create a Fabric Network
```
POST /:login/fabrics/default/vlans/:vlan_id/networks
{
  "name": "app-network",
  "subnet": "10.0.1.0/24",
  "gateway": "10.0.1.1",
  "provision_start_ip": "10.0.1.2",
  "provision_end_ip": "10.0.1.254",
  "internet_nat": true
}
```

### Create a Firewall Rule
```
POST /:login/fwrules
{
  "rule": "FROM any TO tag role = web ALLOW tcp PORT 443",
  "enabled": true,
  "description": "Allow HTTPS to web tier"
}
```

## Key Concepts

- **Fabric Networks**: Private virtual networks within an account, isolated from other tenants. Use VLANs for further segmentation.
- **CNS (Container Name Service)**: Automatic DNS registration for instances via tags. Two zones: private (`.cns.*.zone`) and public (`*.net`).
- **Triton-Moirai**: Metadata-driven HAProxy load balancer that auto-discovers backends via CNS DNS.
- **User-Scripts**: Shell scripts passed as metadata, executed on first boot for automated provisioning.
- **Brands**: Instance types — `joyent` (SmartOS zones), `lx` (Linux zones), `kvm`/`bhyve` (full VMs).

## Best Practices

1. **Always enable firewalls** on production instances (`firewall_enabled: true`)
2. **Use CNS tags** for service discovery: `triton.cns.services=service:port`
3. **Isolate tiers** on separate fabric networks/VLANs
4. **Keep databases on private networks** — no public IP, no NAT
5. **Use tag-based firewall rules** for dynamic security groups
6. **Provision with user-scripts** instead of manual SSH configuration
7. **Use Triton-Moirai** for load balancing with automatic TLS
8. **Graceful scaling** — set `triton.cns.status=down` before removing instances
