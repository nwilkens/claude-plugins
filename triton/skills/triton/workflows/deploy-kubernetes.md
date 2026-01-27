# Deploy Kubernetes Cluster

Deploy k3s Kubernetes on Triton using metadata-based provisioning.

## Overview

This workflow demonstrates deploying lightweight Kubernetes (k3s) clusters on Triton, from single-node development setups to multi-node production clusters.

## Prerequisites

1. Triton CLI configured with a profile
2. CNS enabled on your account: `triton account update triton_cns_enabled=true`
3. A fabric network for internal communication
4. Note your account UUID: `triton account get | grep '"id"'`

## Single-Node Development Cluster

A quick single-node cluster for development and testing.

### Step 1: Create Firewall Rules

> **WARNING:** Triton firewall does NOT support port range syntax. Create specific port rules instead.

```bash
# Allow K8s API access
triton fwrule create -D "k8s-api" \
  "FROM any TO tag role=k8s ALLOW tcp PORT 6443"

# Allow SSH access
triton fwrule create -D "k8s-ssh" \
  "FROM any TO tag role=k8s ALLOW tcp PORT 22"

# Allow specific NodePort (create one per exposed service)
# Example: website on NodePort 30080
triton fwrule create -D "k8s-nodeport-web" \
  "FROM any TO tag role=k8s ALLOW tcp PORT 30080"

# Allow ICMP for diagnostics
triton fwrule create -D "k8s-icmp" \
  "FROM any TO tag role=k8s ALLOW icmp TYPE all"
```

> **Note:** For NodePorts, you must create individual rules for each port (30080, 30443, etc). The syntax `PORT >= 30000 AND PORT <= 32767` does NOT work.

### Step 2: Create k3s Instance

```bash
triton instance create \
  -n k8s-dev-01 \
  -t triton.cns.services=k8s-api:6443 \
  -t role=k8s \
  -t env=development \
  -N My-Fabric-Network \
  --firewall \
  -m "user-script=#!/bin/bash
set -e
exec > /var/log/k3s-install.log 2>&1

echo 'Waiting for cloud-init...'
cloud-init status --wait

echo 'Installing k3s...'
curl -sfL https://get.k3s.io | sh -s - \
  --write-kubeconfig-mode 644 \
  --disable traefik \
  --tls-san k8s-api.svc.\$(mdata-get sdc:account_uuid).\$(mdata-get sdc:datacenter).cns.mnx.io

echo 'Waiting for k3s to be ready...'
until kubectl get nodes; do sleep 5; done

echo 'k3s installation complete'
touch /root/.k3s-installed
" \
  -w \
  ubuntu-24.04 m5d.medium
```

### Step 3: Retrieve kubeconfig

Wait for installation to complete, then retrieve the config:

```bash
# Check if installation is complete
triton instance ssh k8s-dev-01 "test -f /root/.k3s-installed && echo 'Ready' || echo 'Not ready'"

# Get kubeconfig
triton instance ssh k8s-dev-01 "cat /etc/rancher/k3s/k3s.yaml" > ~/.kube/k8s-dev-config

# Update server URL to use CNS or public IP
INSTANCE_IP=$(triton instance get k8s-dev-01 -j | jq -r '.primaryIp')
sed -i.bak "s|127.0.0.1|${INSTANCE_IP}|g" ~/.kube/k8s-dev-config

# Test connection
KUBECONFIG=~/.kube/k8s-dev-config kubectl get nodes
```

## Multi-Node Production Cluster

A high-availability cluster with dedicated control plane and worker nodes.

### Step 1: Create Network Infrastructure

```bash
# Create VLAN for Kubernetes
triton vlan create --name kubernetes 300

# Create network
triton network create \
  --name k8s-internal \
  --vlan-id 300 \
  --subnet 10.30.0.0/24 \
  --start-ip 10.30.0.10 \
  --end-ip 10.30.0.250 \
  --gateway 10.30.0.1
```

### Step 2: Create Firewall Rules

> **WARNING:** Triton firewall does NOT support port range syntax.

```bash
# Internal cluster communication (TCP) - all ports between k8s nodes
triton fwrule create -D "k8s-internal-tcp" \
  "FROM tag role=k8s TO tag role=k8s ALLOW tcp PORT all"

# Internal cluster communication (UDP) - for flannel/CNI
triton fwrule create -D "k8s-internal-udp" \
  "FROM tag role=k8s TO tag role=k8s ALLOW udp PORT all"

# SSH access
triton fwrule create -D "k8s-ssh" \
  "FROM any TO tag role=k8s ALLOW tcp PORT 22"

# K8s API access (external)
triton fwrule create -D "k8s-api-external" \
  "FROM any TO tag role=k8s-control ALLOW tcp PORT 6443"

# Specific NodePort for web traffic (create per-service)
triton fwrule create -D "k8s-nodeport-http" \
  "FROM any TO tag role=k8s-worker ALLOW tcp PORT 30080"

# ICMP for diagnostics
triton fwrule create -D "k8s-icmp" \
  "FROM any TO tag role=k8s ALLOW icmp TYPE all"
```

> **Note:** Create individual NodePort rules for each service you expose. The range syntax `(PORT >= 30000 AND PORT <= 32767)` does NOT work.

### Step 3: Deploy Control Plane Node

```bash
# Generate a token for joining workers
K3S_TOKEN=$(openssl rand -hex 32)
echo "K3S_TOKEN: ${K3S_TOKEN}" > ~/.k3s-token

triton instance create \
  -n k8s-control-01 \
  -t triton.cns.services=k8s-api:6443 \
  -t role=k8s \
  -t role=k8s-control \
  -t env=production \
  -N k8s-internal \
  --firewall \
  -m "user-script=#!/bin/bash
set -e
exec > /var/log/k3s-install.log 2>&1

echo 'Waiting for cloud-init...'
cloud-init status --wait

ACCOUNT_UUID=\$(mdata-get sdc:account_uuid)
DATACENTER=\$(mdata-get sdc:datacenter)

echo 'Installing k3s control plane...'
curl -sfL https://get.k3s.io | sh -s - server \
  --token '${K3S_TOKEN}' \
  --write-kubeconfig-mode 644 \
  --disable traefik \
  --tls-san k8s-api.svc.\${ACCOUNT_UUID}.\${DATACENTER}.cns.mnx.io \
  --node-label node-role=control-plane

echo 'Waiting for k3s to be ready...'
until kubectl get nodes; do sleep 5; done

echo 'k3s control plane ready'
touch /root/.k3s-installed
" \
  -w \
  ubuntu-24.04 m5d.large
```

### Step 4: Deploy Worker Nodes

Get the control plane IP first:

```bash
CONTROL_IP=$(triton instance get k8s-control-01 -j | jq -r '.ips[0]')
```

Deploy workers (run multiple times with different names for more workers):

```bash
for i in 1 2 3; do
triton instance create \
  -n k8s-worker-0${i} \
  -t triton.cns.services=k8s-worker:80 \
  -t role=k8s \
  -t role=k8s-worker \
  -t env=production \
  -N k8s-internal \
  --firewall \
  -m "user-script=#!/bin/bash
set -e
exec > /var/log/k3s-install.log 2>&1

echo 'Waiting for cloud-init...'
cloud-init status --wait

echo 'Installing k3s agent...'
curl -sfL https://get.k3s.io | K3S_URL='https://${CONTROL_IP}:6443' K3S_TOKEN='${K3S_TOKEN}' sh -s - agent \
  --node-label node-role=worker

echo 'k3s agent installed'
touch /root/.k3s-installed
" \
  -w \
  ubuntu-24.04 m5d.large &
done
wait
```

### Step 5: Retrieve and Configure kubeconfig

```bash
# Get kubeconfig from control plane
triton instance ssh k8s-control-01 "cat /etc/rancher/k3s/k3s.yaml" > ~/.kube/k8s-prod-config

# Get control plane IP or use CNS name
ACCOUNT_UUID=$(triton account get -j | jq -r '.id')
DC=$(triton profile get -j | jq -r '.url' | sed 's|.*//||;s|\..*||')

# Update with CNS hostname (if DNS is configured) or IP
sed -i.bak "s|127.0.0.1|k8s-api.svc.${ACCOUNT_UUID}.${DC}.cns.mnx.io|g" ~/.kube/k8s-prod-config

# Verify cluster
KUBECONFIG=~/.kube/k8s-prod-config kubectl get nodes
```

## CNS Integration for Kubernetes Services

Use CNS for external service discovery alongside Kubernetes internal DNS.

### Expose Services via CNS

Tag worker nodes to expose services:

```bash
# All workers serve the web tier
triton instance tag set k8s-worker-01 triton.cns.services=k8s-web:30080
triton instance tag set k8s-worker-02 triton.cns.services=k8s-web:30080
triton instance tag set k8s-worker-03 triton.cns.services=k8s-web:30080
```

### Deploy Application with NodePort

```yaml
# web-service.yaml
apiVersion: v1
kind: Service
metadata:
  name: web
spec:
  type: NodePort
  ports:
    - port: 80
      nodePort: 30080
  selector:
    app: web
```

Access via CNS: `k8s-web.svc.<account-uuid>.<dc>.cns.mnx.io:30080`

## Add Load Balancer

For production, add a Triton-Moirai load balancer in front of the cluster:

```bash
ACCOUNT_UUID=$(triton account get -j | jq -r '.id')
DC=$(triton profile get -j | jq -r '.url' | sed 's|.*//||;s|\..*||')

triton instance create \
  -n k8s-lb \
  -t role=lb \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=https-http://443:k8s-web.svc.${ACCOUNT_UUID}.${DC}.cns.mnx.io:30080{check:/healthz}" \
  -m cloud.tritoncompute:certificate_name=k8s.example.com \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

## Scaling Workers

### Add a Worker

```bash
triton instance create \
  -n k8s-worker-04 \
  -t triton.cns.services=k8s-worker:80,k8s-web:30080 \
  -t role=k8s \
  -t role=k8s-worker \
  -t env=production \
  -N k8s-internal \
  --firewall \
  -m "user-script=#!/bin/bash
set -e
cloud-init status --wait
curl -sfL https://get.k3s.io | K3S_URL='https://${CONTROL_IP}:6443' K3S_TOKEN='${K3S_TOKEN}' sh -s - agent
touch /root/.k3s-installed
" \
  -w \
  ubuntu-24.04 m5d.large
```

### Remove a Worker

```bash
# Drain the node first
KUBECONFIG=~/.kube/k8s-prod-config kubectl drain k8s-worker-04 --ignore-daemonsets --delete-emptydir-data

# Remove from CNS
triton instance metadata set k8s-worker-04 triton.cns.status=down

# Wait for CNS propagation (30-60 seconds)
sleep 60

# Delete the instance
triton instance delete k8s-worker-04
```

## Troubleshooting

### Check k3s Installation Logs

```bash
triton instance ssh k8s-control-01 "cat /var/log/k3s-install.log"
```

### Check k3s Service Status

```bash
triton instance ssh k8s-control-01 "systemctl status k3s"
triton instance ssh k8s-worker-01 "systemctl status k3s-agent"
```

### View k3s Logs

```bash
triton instance ssh k8s-control-01 "journalctl -u k3s -f"
```

### Verify CNS DNS Resolution

```bash
# From any Triton instance
dig k8s-api.svc.<account-uuid>.<dc>.cns.mnx.io
```

## Deploying Container Images to k3s

When building container images locally (especially on macOS/ARM), you must build for the correct architecture and distribute to all nodes.

### Build for Linux/AMD64

```bash
# Build for correct architecture (required if building on macOS M1/M2/M3)
docker build --platform linux/amd64 -t myapp:latest .

# Export and compress for faster transfer
docker save myapp:latest | gzip > /tmp/myapp.tar.gz
```

### Distribute to All Nodes

```bash
# Get node IPs
CONTROL_IP=$(triton instance get k8s-control-01 -j | jq -r '.primaryIp')
WORKER_1_IP=$(triton instance get k8s-worker-01 -j | jq -r '.primaryIp')
WORKER_2_IP=$(triton instance get k8s-worker-02 -j | jq -r '.primaryIp')

# Copy and import to each node
for ip in $CONTROL_IP $WORKER_1_IP $WORKER_2_IP; do
  echo "Deploying to $ip..."
  scp /tmp/myapp.tar.gz root@$ip:/tmp/
  ssh root@$ip 'gunzip -c /tmp/myapp.tar.gz | ctr -n k8s.io images import - && rm /tmp/myapp.tar.gz'
done
```

> **Note:** Use `ctr -n k8s.io images import` for k3s. The `-n k8s.io` namespace is required.

## Lessons Learned / Common Pitfalls

### 1. Firewall Port Ranges Don't Work

**Problem:** Rules like `PORT >= 30000 AND PORT <= 32767` silently fail.

**Solution:** Create specific rules for each port:
```bash
triton fwrule create -D "nodeport-web" "FROM any TO tag role=k8s ALLOW tcp PORT 30080"
triton fwrule create -D "nodeport-api" "FROM any TO tag role=k8s ALLOW tcp PORT 30443"
```

### 2. Two CNS DNS Zones (Public vs Private)

**Problem:** Load balancer can't reach backends, or Let's Encrypt fails.

**Explanation:**
- `*.cns.parlercloud.zone` → Resolves to **fabric/private IPs**
- `*.parlercloud.net` → Resolves to **public IPs**

**Solution:**
- Use **private zone** for load balancer → backend communication (portmap)
- Use **public zone** for Let's Encrypt certificate_name

```bash
# Load balancer portmap uses PRIVATE zone (backends on fabric network)
-m "cloud.tritoncompute:portmap=http://80:k8s-workers.svc.ACCOUNT.DC.cns.parlercloud.zone:30080"

# Certificate uses PUBLIC zone (for Let's Encrypt validation)
-m "cloud.tritoncompute:certificate_name=k8s-www.svc.ACCOUNT.DC.parlercloud.net"
```

### 3. Let's Encrypt Certificate Symlink Issue

**Problem:** HTTPS works but serves self-signed cert instead of Let's Encrypt.

**Cause:** HAProxy uses `/opt/triton/tls/default/` which symlinks to self-signed certs by default.

**Solution:** After certificate is issued (~1-2 min), fix the symlink:
```bash
ssh root@$LB_IP "
  rm /opt/triton/tls/default
  ln -s /opt/triton/tls/\$DOMAIN /opt/triton/tls/default
  svcadm restart haproxy
"
```

### 4. Container Image Architecture Mismatch

**Problem:** Pods crash with `exec format error`.

**Cause:** Image built on ARM (macOS M1/M2/M3) but cluster runs AMD64.

**Solution:** Always specify platform when building:
```bash
docker build --platform linux/amd64 -t myapp:latest .
```

### 5. HAProxy Not Reloading After Metadata Changes

**Problem:** Load balancer config changes don't take effect.

**Solution:** Manually trigger reload:
```bash
ssh root@$LB_IP "/opt/triton/clb/reconfigure && svcadm restart haproxy"
```

### 6. Use Fabric Network for Internal Communication

**Problem:** Cluster communication unreliable or high latency.

**Solution:** Always use fabric network for internal k8s communication:
```bash
triton instance create \
  --network=public \
  --network=My-Fabric-Network \  # Internal communication
  ...
```

Configure k3s to use fabric interface:
```bash
curl -sfL https://get.k3s.io | sh -s - server \
  --flannel-iface eth1 \          # Fabric network interface
  --node-ip $FABRIC_IP \
  --advertise-address $FABRIC_IP
```

## See Also

- [Metadata-Based Provisioning](../patterns/metadata-provisioning.md) - User-script patterns
- [CNS Service Discovery](../patterns/cns-service-discovery.md) - DNS-based service discovery
- [Load Balancing](../patterns/load-balancing.md) - Triton-Moirai setup
- [Firewall Rules](../patterns/firewall-rules.md) - Security patterns
