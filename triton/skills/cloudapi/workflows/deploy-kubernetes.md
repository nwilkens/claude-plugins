# Deploy a Kubernetes (k3s) Cluster via CloudAPI

Complete walkthrough for deploying a production-ready k3s Kubernetes cluster on Triton DataCenter using CloudAPI REST calls.

## Architecture

```
Internet --> HTTPS:443 --> Moirai Load Balancer (optional)
                               | NodePort:30000-32767
                          +-----------+-----------+
                          |           |           |
                     Worker-01   Worker-02   Worker-03
                          |           |           |
                          +-----+-----+-----------+
                                |
                          Control Plane
                          (k3s server, etcd, API:6443)
```

All nodes run on a shared fabric network. The control plane runs k3s server with an embedded etcd datastore. Worker nodes join via the k3s agent using a shared token. An optional Moirai load balancer can be placed in front of worker nodes for external ingress.

## Important Gotchas

Before starting, be aware of these Triton-specific considerations:

1. **Firewall port ranges do not work.** Triton firewall rules do not support `PORT 30000-32767` syntax. You must either use `PORT all` to allow all TCP traffic, or create individual rules for each specific port you need.

2. **Container images must be AMD64.** Triton runs on AMD64 (x86_64) infrastructure. When pulling container images inside your k3s cluster, ensure they are built for `linux/amd64`. Multi-arch images typically work. ARM64-only images will fail.

3. **Let's Encrypt certificate symlink.** Some LX-brand zones may not have the CA certificate bundle at the path that Let's Encrypt clients expect. If you encounter TLS validation errors with certbot or similar tools, create a symlink: `ln -sf /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-bundle.crt`.

4. **User-script execution.** User-scripts run as root on first boot only. If the instance is restarted, the script does not re-execute. Use systemd services for anything that must persist across reboots.

5. **CNS propagation.** DNS records via CNS typically propagate within 30 seconds, but during initial provisioning allow up to 2 minutes before relying on CNS hostnames.

## Prerequisites

All requests use the base URL `https://<cloudapi-host>/<account>/` with HTTP Signature authentication. Replace `<account>` with your login name throughout.

### Identify Image and Package UUIDs

The k3s cluster runs on Linux instances. Find a suitable image:

```
GET /:login/images?name=ubuntu-24.04
```

**Response:**
```json
[
  {
    "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "ubuntu-24.04",
    "version": "20250101",
    "os": "linux",
    "type": "lx-dataset",
    "state": "active"
  }
]
```

Select a package for the control plane (needs more resources) and workers:

```
GET /:login/packages
```

Recommended sizing:

| Role | Package | Memory | vCPU | Disk |
|------|---------|--------|------|------|
| Control plane | g1.small | 2048 MiB | 2 | 51200 MiB |
| Worker node | g1.medium | 4096 MiB | 4 | 102400 MiB |

### Enable CNS

```
GET /:login/config
```

If `triton_cns_enabled` is `false`:

```
PUT /:login/config
Content-Type: application/json

{
  "triton_cns_enabled": true
}
```

---

## Step 1: Create Network Infrastructure

### Create VLAN

```
POST /:login/fabrics/default/vlans
Content-Type: application/json

{
  "vlan_id": 10,
  "name": "k8s-vlan",
  "description": "Kubernetes cluster VLAN"
}
```

**Response:**
```json
{
  "vlan_id": 10,
  "name": "k8s-vlan",
  "description": "Kubernetes cluster VLAN"
}
```

### Create Fabric Network

```
POST /:login/fabrics/default/vlans/10/networks
Content-Type: application/json

{
  "name": "k8s-network",
  "description": "Kubernetes cluster fabric network",
  "subnet": "10.10.0.0/24",
  "gateway": "10.10.0.1",
  "provision_start_ip": "10.10.0.10",
  "provision_end_ip": "10.10.0.250",
  "resolvers": ["8.8.8.8", "8.8.4.4"],
  "internet_nat": true
}
```

**Response:**
```json
{
  "id": "k8s-net-uuid-001",
  "name": "k8s-network",
  "public": false,
  "fabric": true,
  "subnet": "10.10.0.0/24",
  "gateway": "10.10.0.1",
  "provision_start_ip": "10.10.0.10",
  "provision_end_ip": "10.10.0.250",
  "resolvers": ["8.8.8.8", "8.8.4.4"],
  "internet_nat": true,
  "vlan_id": 10
}
```

Save the `id` as `K8S_NETWORK_ID`.

`internet_nat: true` is required so nodes can pull container images from public registries.

---

## Step 2: Create Firewall Rules

**Reminder:** Triton firewall rules do not support port ranges. `PORT 30000-32767` is not valid syntax. Use `PORT all` for broad access or create individual rules for each specific NodePort.

### Allow SSH (Administration)

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"k8s\" ALLOW tcp PORT 22",
  "enabled": true,
  "description": "Allow SSH to all Kubernetes nodes"
}
```

### Allow Kubernetes API (Port 6443)

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"k8s-master\" ALLOW tcp PORT 6443",
  "enabled": true,
  "description": "Allow Kubernetes API access to control plane"
}
```

### Allow Intra-Cluster Communication

K3s nodes need to communicate freely with each other. The simplest approach is to allow all traffic between nodes tagged as k8s:

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM tag \"role\" = \"k8s\" TO tag \"role\" = \"k8s\" ALLOW tcp PORT all",
  "enabled": true,
  "description": "Allow all TCP between Kubernetes nodes"
}
```

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM tag \"role\" = \"k8s\" TO tag \"role\" = \"k8s\" ALLOW udp PORT all",
  "enabled": true,
  "description": "Allow all UDP between Kubernetes nodes (flannel VXLAN)"
}
```

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM tag \"role\" = \"k8s-master\" TO tag \"role\" = \"k8s-master\" ALLOW tcp PORT all",
  "enabled": true,
  "description": "Allow all TCP on control plane nodes"
}
```

### Allow NodePorts from Internet (for Services)

Since port ranges are not supported, allow all TCP to worker nodes or create individual rules per NodePort:

**Option A: Allow all TCP to workers (simpler, less restrictive)**

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"k8s-worker\" ALLOW tcp PORT all",
  "enabled": true,
  "description": "Allow all TCP to worker nodes (includes NodePorts)"
}
```

**Option B: Individual NodePort rules (more restrictive)**

For each specific NodePort your services use:

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"k8s-worker\" ALLOW tcp PORT 30080",
  "enabled": true,
  "description": "Allow NodePort 30080 for web application"
}
```

### Allow HTTP/HTTPS (for load balancer, if used)

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"k8s-lb\" ALLOW tcp PORT 80",
  "enabled": true,
  "description": "Allow HTTP to Kubernetes load balancer"
}
```

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"k8s-lb\" ALLOW tcp PORT 443",
  "enabled": true,
  "description": "Allow HTTPS to Kubernetes load balancer"
}
```

---

## Step 3: Deploy the Control Plane

The control plane runs k3s in server mode. The user-script installs k3s, configures it, and writes the join token to a known location.

### Generate a Shared Token

Generate a random token that workers will use to join the cluster. You will embed this in the user-scripts:

```
K3S_TOKEN="my-secure-cluster-token-$(openssl rand -hex 16)"
```

Use this token value in both the server and agent user-scripts below.

### Create Control Plane Instance

```
POST /:login/machines
Content-Type: application/json

{
  "name": "k8s-master-01",
  "image": "<image-uuid>",
  "package": "<cp-package-uuid>",
  "networks": ["<K8S_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "k8s-master",
    "cluster": "my-k8s",
    "triton.cns.services": "k8s-master"
  },
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\n\nexport K3S_TOKEN='my-secure-cluster-token-REPLACE_WITH_GENERATED_VALUE'\nexport INSTALL_K3S_EXEC='server'\n\n# Ensure CA certificates are available\nln -sf /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-bundle.crt 2>/dev/null || true\n\n# Install k3s server\ncurl -sfL https://get.k3s.io | sh -s - server \\\n  --token \"$K3S_TOKEN\" \\\n  --tls-san k8s-master.svc.triton.zone \\\n  --tls-san $(hostname -I | awk '{print $1}') \\\n  --write-kubeconfig-mode 644 \\\n  --disable traefik \\\n  --flannel-backend vxlan\n\n# Wait for k3s to be ready\nfor i in $(seq 1 60); do\n  if /usr/local/bin/kubectl get nodes >/dev/null 2>&1; then\n    echo 'k3s server is ready'\n    break\n  fi\n  echo \"Waiting for k3s to start... ($i/60)\"\n  sleep 5\ndone\n\n# Display node status\n/usr/local/bin/kubectl get nodes -o wide\n\necho 'Control plane provisioning complete'\n"
  }
}
```

**Response:**
```json
{
  "id": "master-machine-uuid-001",
  "name": "k8s-master-01",
  "state": "provisioning",
  "ips": ["10.10.0.10"]
}
```

**Key k3s server flags:**

| Flag | Purpose |
|------|---------|
| `--token` | Shared secret for worker nodes to join |
| `--tls-san` | Additional SANs for the API server TLS certificate (include CNS hostname and IP) |
| `--write-kubeconfig-mode 644` | Make kubeconfig readable for non-root access |
| `--disable traefik` | Disable built-in Traefik ingress (we use Moirai instead) |
| `--flannel-backend vxlan` | Use VXLAN for pod networking overlay |

### Wait for Control Plane to Be Running

```
GET /:login/machines/<master-machine-uuid-001>
```

Poll until `state` is `"running"`. After the machine is running, allow additional time (1-2 minutes) for k3s to complete initialization via the user-script.

---

## Step 4: Deploy Worker Nodes

Worker nodes run k3s in agent mode and join the cluster using the shared token. They discover the control plane via CNS DNS.

### Worker Node 1

```
POST /:login/machines
Content-Type: application/json

{
  "name": "k8s-worker-01",
  "image": "<image-uuid>",
  "package": "<worker-package-uuid>",
  "networks": ["<K8S_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "k8s-worker",
    "cluster": "my-k8s",
    "triton.cns.services": "k8s-workers"
  },
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\n\nexport K3S_TOKEN='my-secure-cluster-token-REPLACE_WITH_GENERATED_VALUE'\nexport K3S_URL='https://k8s-master.svc.triton.zone:6443'\n\n# Ensure CA certificates are available\nln -sf /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-bundle.crt 2>/dev/null || true\n\n# Wait for control plane DNS to be resolvable\nfor i in $(seq 1 30); do\n  if getent hosts k8s-master.svc.triton.zone >/dev/null 2>&1; then\n    echo 'Control plane DNS resolved'\n    break\n  fi\n  echo \"Waiting for CNS DNS propagation... ($i/30)\"\n  sleep 10\ndone\n\n# Wait for k3s API to be reachable\nfor i in $(seq 1 30); do\n  if curl -sk https://k8s-master.svc.triton.zone:6443/healthz >/dev/null 2>&1; then\n    echo 'K3s API is reachable'\n    break\n  fi\n  echo \"Waiting for k3s API... ($i/30)\"\n  sleep 10\ndone\n\n# Install k3s agent\ncurl -sfL https://get.k3s.io | sh -s - agent \\\n  --token \"$K3S_TOKEN\" \\\n  --server \"$K3S_URL\"\n\necho 'Worker node provisioning complete'\n"
  }
}
```

**Response:**
```json
{
  "id": "worker-machine-uuid-001",
  "name": "k8s-worker-01",
  "state": "provisioning",
  "ips": ["10.10.0.11"]
}
```

### Worker Node 2

```
POST /:login/machines
Content-Type: application/json

{
  "name": "k8s-worker-02",
  "image": "<image-uuid>",
  "package": "<worker-package-uuid>",
  "networks": ["<K8S_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "k8s-worker",
    "cluster": "my-k8s",
    "triton.cns.services": "k8s-workers"
  },
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\n\nexport K3S_TOKEN='my-secure-cluster-token-REPLACE_WITH_GENERATED_VALUE'\nexport K3S_URL='https://k8s-master.svc.triton.zone:6443'\n\nln -sf /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-bundle.crt 2>/dev/null || true\n\nfor i in $(seq 1 30); do\n  if getent hosts k8s-master.svc.triton.zone >/dev/null 2>&1; then\n    echo 'Control plane DNS resolved'\n    break\n  fi\n  echo \"Waiting for CNS DNS propagation... ($i/30)\"\n  sleep 10\ndone\n\nfor i in $(seq 1 30); do\n  if curl -sk https://k8s-master.svc.triton.zone:6443/healthz >/dev/null 2>&1; then\n    echo 'K3s API is reachable'\n    break\n  fi\n  echo \"Waiting for k3s API... ($i/30)\"\n  sleep 10\ndone\n\ncurl -sfL https://get.k3s.io | sh -s - agent \\\n  --token \"$K3S_TOKEN\" \\\n  --server \"$K3S_URL\"\n\necho 'Worker node provisioning complete'\n"
  }
}
```

### Worker Node 3

```
POST /:login/machines
Content-Type: application/json

{
  "name": "k8s-worker-03",
  "image": "<image-uuid>",
  "package": "<worker-package-uuid>",
  "networks": ["<K8S_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "k8s-worker",
    "cluster": "my-k8s",
    "triton.cns.services": "k8s-workers"
  },
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\n\nexport K3S_TOKEN='my-secure-cluster-token-REPLACE_WITH_GENERATED_VALUE'\nexport K3S_URL='https://k8s-master.svc.triton.zone:6443'\n\nln -sf /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-bundle.crt 2>/dev/null || true\n\nfor i in $(seq 1 30); do\n  if getent hosts k8s-master.svc.triton.zone >/dev/null 2>&1; then\n    break\n  fi\n  sleep 10\ndone\n\nfor i in $(seq 1 30); do\n  if curl -sk https://k8s-master.svc.triton.zone:6443/healthz >/dev/null 2>&1; then\n    break\n  fi\n  sleep 10\ndone\n\ncurl -sfL https://get.k3s.io | sh -s - agent \\\n  --token \"$K3S_TOKEN\" \\\n  --server \"$K3S_URL\"\n\necho 'Worker node provisioning complete'\n"
  }
}
```

All worker nodes can be created in parallel. They will retry connecting to the control plane until it becomes available.

---

## Step 5: Retrieve Kubeconfig

Once the control plane is running and k3s has initialized, the kubeconfig is available at `/etc/rancher/k3s/k3s.yaml` on the master node.

SSH into the control plane and retrieve it:

```bash
ssh root@10.10.0.10 cat /etc/rancher/k3s/k3s.yaml
```

The kubeconfig will contain `server: https://127.0.0.1:6443`. Replace this with the control plane's CNS hostname or IP:

```yaml
apiVersion: v1
clusters:
- cluster:
    certificate-authority-data: <base64-ca-cert>
    server: https://k8s-master.svc.<account>.<dc>.cns.triton.zone:6443
  name: default
contexts:
- context:
    cluster: default
    user: default
  name: default
current-context: default
kind: Config
preferences: {}
users:
- name: default
  user:
    client-certificate-data: <base64-client-cert>
    client-key-data: <base64-client-key>
```

Save this to your local `~/.kube/config` and verify:

```bash
kubectl get nodes
```

**Expected output:**
```
NAME             STATUS   ROLES                  AGE   VERSION
k8s-master-01    Ready    control-plane,master   5m    v1.31.x+k3s1
k8s-worker-01    Ready    <none>                 3m    v1.31.x+k3s1
k8s-worker-02    Ready    <none>                 3m    v1.31.x+k3s1
k8s-worker-03    Ready    <none>                 3m    v1.31.x+k3s1
```

---

## Step 6: Deploy Moirai Load Balancer (Optional)

For production clusters, place a Moirai load balancer in front of the worker nodes. This provides a stable public IP, TLS termination, and health-checked routing to NodePort services.

### Find the Public Network

```
GET /:login/networks
```

Find the network with `"public": true` and save its `id` as `PUBLIC_NETWORK_ID`.

### Create Load Balancer Instance

```
POST /:login/machines
Content-Type: application/json

{
  "name": "k8s-lb-01",
  "image": "<lb-image-uuid>",
  "package": "<lb-package-uuid>",
  "networks": ["<PUBLIC_NETWORK_ID>", "<K8S_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "k8s-lb",
    "cluster": "my-k8s",
    "triton.cns.services": "k8s-lb"
  },
  "metadata": {
    "cloud.tritoncompute:loadbalancer": "true",
    "cloud.tritoncompute:backend_service": "k8s-workers",
    "cloud.tritoncompute:backend_port": "30080",
    "cloud.tritoncompute:frontend_port": "443",
    "cloud.tritoncompute:health_check_path": "/healthz",
    "cloud.tritoncompute:health_check_interval": "5000",
    "cloud.tritoncompute:tls_domain": "k8s.example.com",
    "cloud.tritoncompute:tls_email": "admin@example.com"
  }
}
```

**Response:**
```json
{
  "id": "lb-machine-uuid-001",
  "name": "k8s-lb-01",
  "state": "provisioning",
  "primaryIp": "203.0.113.100",
  "ips": ["203.0.113.100", "10.10.0.20"]
}
```

The load balancer discovers worker nodes via the `k8s-workers` CNS service name. When you deploy a Kubernetes service with `type: NodePort` on port 30080, Moirai routes traffic from `https://k8s.example.com` to the workers.

### Example: Deploy a NodePort Service in Kubernetes

After the load balancer is running, deploy a sample application:

```yaml
# deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: web-app
  template:
    metadata:
      labels:
        app: web-app
    spec:
      containers:
      - name: web
        image: nginx:latest    # Must be AMD64 compatible
        ports:
        - containerPort: 80
---
apiVersion: v1
kind: Service
metadata:
  name: web-app
spec:
  type: NodePort
  selector:
    app: web-app
  ports:
  - port: 80
    targetPort: 80
    nodePort: 30080
```

Apply it:

```bash
kubectl apply -f deployment.yaml
```

Moirai automatically routes `https://k8s.example.com` to port 30080 on all worker nodes.

---

## Step 7: CNS Integration for External Service Access

CNS provides DNS names for your Kubernetes nodes. This is useful for services that need direct access without going through the load balancer.

### CNS Naming Convention

| CNS Service Tag | DNS Name (private) |
|----------------|-------------------|
| `k8s-master` | `k8s-master.svc.<account>.<dc>.cns.triton.zone` |
| `k8s-workers` | `k8s-workers.svc.<account>.<dc>.cns.triton.zone` |
| `k8s-lb` | `k8s-lb.svc.<account>.<dc>.triton.zone` (public) |

### Accessing the Kubernetes API Externally

Point your kubeconfig at the CNS name:

```
server: https://k8s-master.svc.<account>.<dc>.cns.triton.zone:6443
```

This name automatically resolves to the control plane's fabric IP.

### Multi-Master (HA) Considerations

For high availability, deploy multiple control plane nodes with the same CNS tag. k3s supports multi-server mode with `--cluster-init`:

```json
{
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\nexport K3S_TOKEN='my-secure-cluster-token-REPLACE_WITH_GENERATED_VALUE'\ncurl -sfL https://get.k3s.io | sh -s - server \\\n  --token \"$K3S_TOKEN\" \\\n  --cluster-init \\\n  --tls-san k8s-master.svc.triton.zone \\\n  --write-kubeconfig-mode 644 \\\n  --disable traefik \\\n  --flannel-backend vxlan\n"
  }
}
```

Additional control plane nodes join with `--server`:

```json
{
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\nexport K3S_TOKEN='my-secure-cluster-token-REPLACE_WITH_GENERATED_VALUE'\ncurl -sfL https://get.k3s.io | sh -s - server \\\n  --token \"$K3S_TOKEN\" \\\n  --server https://k8s-master.svc.triton.zone:6443 \\\n  --tls-san k8s-master.svc.triton.zone \\\n  --write-kubeconfig-mode 644 \\\n  --disable traefik \\\n  --flannel-backend vxlan\n"
  }
}
```

All control plane nodes share the same `triton.cns.services: k8s-master` tag, so CNS creates a round-robin DNS record across all of them.

---

## Step 8: Scaling Workers

### Scale Up: Add Workers

Create additional worker instances with the same configuration and CNS tag. They join automatically via the k3s agent.

```
POST /:login/machines
Content-Type: application/json

{
  "name": "k8s-worker-04",
  "image": "<image-uuid>",
  "package": "<worker-package-uuid>",
  "networks": ["<K8S_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "k8s-worker",
    "cluster": "my-k8s",
    "triton.cns.services": "k8s-workers"
  },
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\nexport K3S_TOKEN='my-secure-cluster-token-REPLACE_WITH_GENERATED_VALUE'\nexport K3S_URL='https://k8s-master.svc.triton.zone:6443'\nln -sf /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-bundle.crt 2>/dev/null || true\nfor i in $(seq 1 30); do\n  if curl -sk https://k8s-master.svc.triton.zone:6443/healthz >/dev/null 2>&1; then break; fi\n  sleep 10\ndone\ncurl -sfL https://get.k3s.io | sh -s - agent --token \"$K3S_TOKEN\" --server \"$K3S_URL\"\n"
  }
}
```

After the instance is running, verify it joined the cluster:

```bash
kubectl get nodes
```

If using Moirai, the new worker is automatically discovered via CNS DNS -- no load balancer reconfiguration is needed.

### Scale Down: Remove Workers

**Step 1: Drain the node in Kubernetes**

```bash
kubectl drain k8s-worker-03 --ignore-daemonsets --delete-emptydir-data
```

**Step 2: Remove the node from the cluster**

```bash
kubectl delete node k8s-worker-03
```

**Step 3: Mark the instance as down in CNS**

```
PUT /:login/machines/<worker-machine-uuid>/tags
Content-Type: application/json

{
  "triton.cns.status": "down"
}
```

**Step 4: Stop and delete the instance**

```
POST /:login/machines/<worker-machine-uuid>?action=stop
```

Wait for the machine to stop, then:

```
DELETE /:login/machines/<worker-machine-uuid>
```

---

## Verification

### Check All Cluster Nodes via CloudAPI

```
GET /:login/machines?tag.cluster=my-k8s
```

**Response:**
```json
[
  { "id": "master-machine-uuid-001", "name": "k8s-master-01", "state": "running", "ips": ["10.10.0.10"] },
  { "id": "worker-machine-uuid-001", "name": "k8s-worker-01", "state": "running", "ips": ["10.10.0.11"] },
  { "id": "worker-machine-uuid-002", "name": "k8s-worker-02", "state": "running", "ips": ["10.10.0.12"] },
  { "id": "worker-machine-uuid-003", "name": "k8s-worker-03", "state": "running", "ips": ["10.10.0.13"] },
  { "id": "lb-machine-uuid-001", "name": "k8s-lb-01", "state": "running", "ips": ["203.0.113.100", "10.10.0.20"] }
]
```

### Check Kubernetes Health

```bash
# Node status
kubectl get nodes -o wide

# System pods
kubectl get pods -n kube-system

# Cluster info
kubectl cluster-info
```

### Test a Deployment

```bash
# Deploy nginx
kubectl create deployment test-nginx --image=nginx:latest
kubectl expose deployment test-nginx --type=NodePort --port=80

# Check the assigned NodePort
kubectl get svc test-nginx
# NAME         TYPE       CLUSTER-IP    EXTERNAL-IP   PORT(S)        AGE
# test-nginx   NodePort   10.43.x.x    <none>        80:3xxxx/TCP   10s

# Access via worker node IP
curl http://10.10.0.11:<nodeport>

# Clean up
kubectl delete deployment test-nginx
kubectl delete svc test-nginx
```

---

## Cleanup

To tear down the entire cluster, delete resources in reverse order:

### Delete Worker Instances

```bash
# Drain all workers first
kubectl drain k8s-worker-01 --ignore-daemonsets --delete-emptydir-data
kubectl drain k8s-worker-02 --ignore-daemonsets --delete-emptydir-data
kubectl drain k8s-worker-03 --ignore-daemonsets --delete-emptydir-data
```

```
POST /:login/machines/<worker-machine-uuid-001>?action=stop
POST /:login/machines/<worker-machine-uuid-002>?action=stop
POST /:login/machines/<worker-machine-uuid-003>?action=stop
```

Wait for all to reach `stopped` state, then:

```
DELETE /:login/machines/<worker-machine-uuid-001>
DELETE /:login/machines/<worker-machine-uuid-002>
DELETE /:login/machines/<worker-machine-uuid-003>
```

### Delete Load Balancer (if deployed)

```
POST /:login/machines/<lb-machine-uuid-001>?action=stop
```

```
DELETE /:login/machines/<lb-machine-uuid-001>
```

### Delete Control Plane

```
POST /:login/machines/<master-machine-uuid-001>?action=stop
```

```
DELETE /:login/machines/<master-machine-uuid-001>
```

### Delete Firewall Rules

```
GET /:login/fwrules
```

Delete each rule by UUID:

```
DELETE /:login/fwrules/<fwrule-uuid>
```

### Delete Network Infrastructure

```
DELETE /:login/fabrics/default/vlans/10/networks/<K8S_NETWORK_ID>
DELETE /:login/fabrics/default/vlans/10
```

---

## Summary

| Resource | Count | Purpose |
|----------|-------|---------|
| VLAN | 1 | k8s-vlan (ID 10) |
| Fabric Network | 1 | k8s-network (10.10.0.0/24) |
| Control Plane | 1 | k3s server, API on port 6443 |
| Worker Nodes | 3 | k3s agents, run workloads |
| Load Balancer | 1 (optional) | Moirai, TLS termination, routes to NodePorts |
| Firewall Rules | 7-9 | SSH, K8s API, intra-cluster, NodePorts, HTTP/S |

The k3s cluster uses CNS for internal service discovery between control plane and workers, flannel VXLAN for pod networking, and optionally Moirai for external ingress with automatic TLS. Scaling is a single CloudAPI call to add a new worker -- k3s handles cluster join automatically.
