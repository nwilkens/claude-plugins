# Deploy a 3-Tier Web Application via CloudAPI

Complete walkthrough for deploying a production-ready 3-tier web application on Triton DataCenter using CloudAPI REST calls.

## Architecture

```
Internet --> HTTPS:443 --> Load Balancer (Moirai / HAProxy)
                               | HTTP:8080
                          Web Tier (2x instances)
                               | TCP:3000
                          API Tier (2x instances)
                               | TCP:5432
                          Database (isolated network, no NAT)
```

Each tier runs on its own VLAN and fabric network. Firewall rules enforce strict inter-tier communication. CNS provides automatic DNS-based service discovery between tiers. The Moirai load balancer handles TLS termination and auto-discovers web backends via CNS DNS.

## Prerequisites

All requests use the base URL `https://<cloudapi-host>/<account>/` with HTTP Signature authentication. Replace `<account>` with your login name throughout.

### Step 0a: Get Account UUID

```
GET /:login
```

**Response:**
```json
{
  "id": "b4bb1880-4f6a-11e3-8236-70b3d5459559",
  "login": "myaccount",
  "email": "admin@example.com",
  "companyName": "Example Corp",
  "triton_cns_enabled": false
}
```

Save the `id` value -- you will need it for firewall rules and other operations.

### Step 0b: Enable CNS (Container Name Service)

CNS provides automatic DNS registration for instances. Check if it is already enabled:

```
GET /:login/config
```

**Response:**
```json
{
  "default_network": "...",
  "triton_cns_enabled": false
}
```

If `triton_cns_enabled` is `false`, enable it:

```
PUT /:login/config
Content-Type: application/json

{
  "triton_cns_enabled": true
}
```

### Step 0c: Identify Image and Package UUIDs

List available images and select one for your instances:

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

List packages and select an appropriate size:

```
GET /:login/packages?name=g1.xsmall
```

**Response:**
```json
[
  {
    "id": "7b17343c-94af-6266-e0e8-893a3b9993d0",
    "name": "g1.xsmall",
    "memory": 1024,
    "disk": 25600,
    "vcpus": 1
  }
]
```

For the database tier, you may want a larger package:

```
GET /:login/packages?name=g1.medium
```

Save the UUIDs for use in subsequent steps.

---

## Step 1: Create VLANs

Create three VLANs to isolate each tier. VLAN IDs are integers between 0 and 4095.

### Web VLAN (ID 100)

```
POST /:login/fabrics/default/vlans
Content-Type: application/json

{
  "vlan_id": 100,
  "name": "web-vlan",
  "description": "Web tier VLAN - public-facing instances"
}
```

**Response:**
```json
{
  "vlan_id": 100,
  "name": "web-vlan",
  "description": "Web tier VLAN - public-facing instances"
}
```

### API VLAN (ID 200)

```
POST /:login/fabrics/default/vlans
Content-Type: application/json

{
  "vlan_id": 200,
  "name": "api-vlan",
  "description": "API tier VLAN - internal services"
}
```

### Database VLAN (ID 300)

```
POST /:login/fabrics/default/vlans
Content-Type: application/json

{
  "vlan_id": 300,
  "name": "db-vlan",
  "description": "Database tier VLAN - fully isolated"
}
```

---

## Step 2: Create Fabric Networks

Create a fabric network on each VLAN. Each network gets its own /24 subnet.

### Web Network

```
POST /:login/fabrics/default/vlans/100/networks
Content-Type: application/json

{
  "name": "web-network",
  "description": "Web tier fabric network",
  "subnet": "10.100.0.0/24",
  "gateway": "10.100.0.1",
  "provision_start_ip": "10.100.0.10",
  "provision_end_ip": "10.100.0.250",
  "resolvers": ["8.8.8.8", "8.8.4.4"],
  "internet_nat": true
}
```

**Response:**
```json
{
  "id": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeee0100",
  "name": "web-network",
  "public": false,
  "fabric": true,
  "subnet": "10.100.0.0/24",
  "gateway": "10.100.0.1",
  "provision_start_ip": "10.100.0.10",
  "provision_end_ip": "10.100.0.250",
  "resolvers": ["8.8.8.8", "8.8.4.4"],
  "internet_nat": true,
  "vlan_id": 100
}
```

Save the `id` as `WEB_NETWORK_ID`.

### API Network

```
POST /:login/fabrics/default/vlans/200/networks
Content-Type: application/json

{
  "name": "api-network",
  "description": "API tier fabric network",
  "subnet": "10.200.0.0/24",
  "gateway": "10.200.0.1",
  "provision_start_ip": "10.200.0.10",
  "provision_end_ip": "10.200.0.250",
  "resolvers": ["8.8.8.8", "8.8.4.4"],
  "internet_nat": true
}
```

Save the `id` as `API_NETWORK_ID`.

### Database Network

The database network has `internet_nat: false` to ensure complete isolation from the public internet.

```
POST /:login/fabrics/default/vlans/300/networks
Content-Type: application/json

{
  "name": "db-network",
  "description": "Database tier fabric network - no internet access",
  "subnet": "10.30.0.0/24",
  "gateway": "10.30.0.1",
  "provision_start_ip": "10.30.0.10",
  "provision_end_ip": "10.30.0.250",
  "resolvers": ["8.8.8.8", "8.8.4.4"],
  "internet_nat": false
}
```

Save the `id` as `DB_NETWORK_ID`.

---

## Step 3: Create Firewall Rules

Firewall rules use Triton's rule language. Rules reference instances by tag, making them dynamic -- new instances with matching tags are automatically included.

### Allow HTTPS to Web Tier (from internet)

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"web\" ALLOW tcp PORT 443",
  "enabled": true,
  "description": "Allow HTTPS from internet to web tier"
}
```

**Response:**
```json
{
  "id": "fwrule-uuid-1",
  "rule": "FROM any TO tag \"role\" = \"web\" ALLOW tcp PORT 443",
  "enabled": true,
  "description": "Allow HTTPS from internet to web tier"
}
```

### Allow HTTP to Web Tier (for load balancer health checks)

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"web\" ALLOW tcp PORT 8080",
  "enabled": true,
  "description": "Allow HTTP/8080 from load balancer to web tier"
}
```

### Allow Web Tier to API Tier

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM tag \"role\" = \"web\" TO tag \"role\" = \"api\" ALLOW tcp PORT 3000",
  "enabled": true,
  "description": "Allow web tier to reach API tier on port 3000"
}
```

### Allow API Tier to Database Tier

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM tag \"role\" = \"api\" TO tag \"role\" = \"db\" ALLOW tcp PORT 5432",
  "enabled": true,
  "description": "Allow API tier to reach database on port 5432"
}
```

### Allow SSH for Administration

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"web\" ALLOW tcp PORT 22",
  "enabled": true,
  "description": "Allow SSH to web tier for administration"
}
```

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"api\" ALLOW tcp PORT 22",
  "enabled": true,
  "description": "Allow SSH to API tier for administration"
}
```

```
POST /:login/fwrules
Content-Type: application/json

{
  "rule": "FROM any TO tag \"role\" = \"db\" ALLOW tcp PORT 22",
  "enabled": true,
  "description": "Allow SSH to database tier for administration"
}
```

### Block All Other Traffic (default deny)

Triton firewalls default to deny. Enabling `firewall_enabled: true` on instances activates the rules. Only traffic explicitly allowed by rules above will pass.

---

## Step 4: Deploy Database Tier

The database instance runs on the isolated `db-network` only, with no public IP and no NAT. It uses a user-script for first-boot provisioning of PostgreSQL.

```
POST /:login/machines
Content-Type: application/json

{
  "name": "myapp-db-01",
  "image": "<db-image-uuid>",
  "package": "<db-package-uuid>",
  "networks": ["<DB_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "db",
    "app": "myapp",
    "triton.cns.services": "myapp-db"
  },
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\n\n# Install PostgreSQL 16\napt-get update\napt-get install -y postgresql-16 postgresql-client-16\n\n# Configure PostgreSQL to listen on all interfaces\nsed -i \"s/#listen_addresses = 'localhost'/listen_addresses = '*'/\" /etc/postgresql/16/main/postgresql.conf\n\n# Allow connections from API tier subnet\necho 'host all all 10.200.0.0/24 md5' >> /etc/postgresql/16/main/pg_hba.conf\n\n# Create application database and user\nsudo -u postgres psql -c \"CREATE USER appuser WITH PASSWORD 'changeme-use-strong-password';\"\nsudo -u postgres psql -c \"CREATE DATABASE appdb OWNER appuser;\"\n\n# Restart PostgreSQL\nsystemctl restart postgresql\nsystemctl enable postgresql\n\necho 'PostgreSQL provisioning complete'\n"
  }
}
```

**Response:**
```json
{
  "id": "db-machine-uuid-001",
  "name": "myapp-db-01",
  "state": "provisioning",
  "image": "<db-image-uuid>",
  "package": "<db-package-uuid>",
  "ips": ["10.30.0.10"],
  "networks": ["<DB_NETWORK_ID>"],
  "firewall_enabled": true
}
```

### Wait for Database to Be Running

Poll until `state` is `"running"`:

```
GET /:login/machines/<db-machine-uuid-001>
```

**Response (when ready):**
```json
{
  "id": "db-machine-uuid-001",
  "name": "myapp-db-01",
  "state": "running",
  "primaryIp": "10.30.0.10",
  "ips": ["10.30.0.10"]
}
```

The database is now accessible at `myapp-db.svc.<account>.<dc>.cns.triton.zone` from other fabric networks (via CNS), or directly at `10.30.0.10`.

---

## Step 5: Deploy API Tier (2 Instances)

The API tier instances connect to both the `api-network` and the `db-network` so they can reach the database. CNS tags enable automatic DNS round-robin for load distribution.

### API Instance 1

```
POST /:login/machines
Content-Type: application/json

{
  "name": "myapp-api-01",
  "image": "<app-image-uuid>",
  "package": "<app-package-uuid>",
  "networks": ["<API_NETWORK_ID>", "<DB_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "api",
    "app": "myapp",
    "triton.cns.services": "myapp-api:3000"
  },
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\n\n# Install Node.js 24.x\ncurl -fsSL https://deb.nodesource.com/setup_24.x | bash -\napt-get install -y nodejs git\n\n# Clone and build the API application\ncd /root\ngit clone https://github.com/your-org/your-api-repo.git app\ncd app\nnpm install\nnpm run build 2>/dev/null || true\n\n# Create systemd service\ncat > /etc/systemd/system/app.service <<EOF\n[Unit]\nDescription=API Server\nAfter=network-online.target\n\n[Service]\nExecStart=/usr/bin/node /root/app/server.js\nRestart=always\nWorkingDirectory=/root/app\nEnvironment=NODE_ENV=production\nEnvironment=PORT=3000\nEnvironment=DATABASE_URL=postgresql://appuser:changeme-use-strong-password@myapp-db.svc.triton.zone:5432/appdb\n\n[Install]\nWantedBy=multi-user.target\nEOF\n\nsystemctl enable app.service\nsystemctl start app.service\n\necho 'API provisioning complete'\n"
  }
}
```

**Response:**
```json
{
  "id": "api-machine-uuid-001",
  "name": "myapp-api-01",
  "state": "provisioning",
  "ips": ["10.200.0.10", "10.30.0.11"]
}
```

### API Instance 2

```
POST /:login/machines
Content-Type: application/json

{
  "name": "myapp-api-02",
  "image": "<app-image-uuid>",
  "package": "<app-package-uuid>",
  "networks": ["<API_NETWORK_ID>", "<DB_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "api",
    "app": "myapp",
    "triton.cns.services": "myapp-api:3000"
  },
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\n\ncurl -fsSL https://deb.nodesource.com/setup_24.x | bash -\napt-get install -y nodejs git\n\ncd /root\ngit clone https://github.com/your-org/your-api-repo.git app\ncd app\nnpm install\nnpm run build 2>/dev/null || true\n\ncat > /etc/systemd/system/app.service <<EOF\n[Unit]\nDescription=API Server\nAfter=network-online.target\n\n[Service]\nExecStart=/usr/bin/node /root/app/server.js\nRestart=always\nWorkingDirectory=/root/app\nEnvironment=NODE_ENV=production\nEnvironment=PORT=3000\nEnvironment=DATABASE_URL=postgresql://appuser:changeme-use-strong-password@myapp-db.svc.triton.zone:5432/appdb\n\n[Install]\nWantedBy=multi-user.target\nEOF\n\nsystemctl enable app.service\nsystemctl start app.service\n"
  }
}
```

Both API instances share the same CNS service tag `myapp-api:3000`, so CNS automatically creates a DNS round-robin record at `myapp-api.svc.<account>.<dc>.cns.triton.zone`.

---

## Step 6: Deploy Web Tier (2 Instances)

The web tier runs a frontend application (e.g., Nginx serving static files and proxying to the API tier). It connects to the `web-network` and `api-network`.

### Web Instance 1

```
POST /:login/machines
Content-Type: application/json

{
  "name": "myapp-web-01",
  "image": "<app-image-uuid>",
  "package": "<app-package-uuid>",
  "networks": ["<WEB_NETWORK_ID>", "<API_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "web",
    "app": "myapp",
    "triton.cns.services": "myapp-web:8080"
  },
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\n\napt-get update\napt-get install -y nginx\n\n# Configure Nginx as reverse proxy\ncat > /etc/nginx/sites-available/default <<'NGINX'\nupstream api_backend {\n    server myapp-api.svc.triton.zone:3000;\n}\n\nserver {\n    listen 8080;\n    server_name _;\n\n    # Health check endpoint\n    location /health {\n        return 200 'ok';\n        add_header Content-Type text/plain;\n    }\n\n    # Static files\n    location / {\n        root /var/www/html;\n        try_files $uri $uri/ /index.html;\n    }\n\n    # Proxy API requests\n    location /api/ {\n        proxy_pass http://api_backend;\n        proxy_set_header Host $host;\n        proxy_set_header X-Real-IP $remote_addr;\n        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;\n        proxy_set_header X-Forwarded-Proto $scheme;\n    }\n}\nNGINX\n\nsystemctl restart nginx\nsystemctl enable nginx\n\necho 'Web tier provisioning complete'\n"
  }
}
```

### Web Instance 2

```
POST /:login/machines
Content-Type: application/json

{
  "name": "myapp-web-02",
  "image": "<app-image-uuid>",
  "package": "<app-package-uuid>",
  "networks": ["<WEB_NETWORK_ID>", "<API_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "web",
    "app": "myapp",
    "triton.cns.services": "myapp-web:8080"
  },
  "metadata": {
    "user-script": "#!/bin/bash\nset -e\n\napt-get update\napt-get install -y nginx\n\ncat > /etc/nginx/sites-available/default <<'NGINX'\nupstream api_backend {\n    server myapp-api.svc.triton.zone:3000;\n}\n\nserver {\n    listen 8080;\n    server_name _;\n\n    location /health {\n        return 200 'ok';\n        add_header Content-Type text/plain;\n    }\n\n    location / {\n        root /var/www/html;\n        try_files $uri $uri/ /index.html;\n    }\n\n    location /api/ {\n        proxy_pass http://api_backend;\n        proxy_set_header Host $host;\n        proxy_set_header X-Real-IP $remote_addr;\n        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;\n        proxy_set_header X-Forwarded-Proto $scheme;\n    }\n}\nNGINX\n\nsystemctl restart nginx\nsystemctl enable nginx\n"
  }
}
```

---

## Step 7: Deploy Load Balancer (Moirai)

Triton-Moirai is a metadata-driven HAProxy load balancer. It uses a SmartOS zone image (minimal-64-lts) and auto-discovers backends via CNS DNS. The instance requires both a public network (for internet-facing traffic) and the web fabric network (to reach backends).

### Find the Public Network

```
GET /:login/networks
```

Look for the network with `"public": true`:

```json
[
  {
    "id": "public-net-uuid",
    "name": "external",
    "public": true,
    "fabric": false
  },
  ...
]
```

Save the public network UUID as `PUBLIC_NETWORK_ID`.

### Create the Load Balancer Instance

```
POST /:login/machines
Content-Type: application/json

{
  "name": "myapp-lb-01",
  "image": "<lb-image-uuid>",
  "package": "<lb-package-uuid>",
  "networks": ["<PUBLIC_NETWORK_ID>", "<WEB_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "lb",
    "app": "myapp",
    "triton.cns.services": "myapp-lb"
  },
  "metadata": {
    "cloud.tritoncompute:loadbalancer": "true",
    "cloud.tritoncompute:backend_service": "myapp-web",
    "cloud.tritoncompute:backend_port": "8080",
    "cloud.tritoncompute:frontend_port": "443",
    "cloud.tritoncompute:health_check_path": "/health",
    "cloud.tritoncompute:health_check_interval": "5000",
    "cloud.tritoncompute:tls_domain": "myapp.example.com",
    "cloud.tritoncompute:tls_email": "admin@example.com"
  }
}
```

**Response:**
```json
{
  "id": "lb-machine-uuid-001",
  "name": "myapp-lb-01",
  "state": "provisioning",
  "primaryIp": "203.0.113.50",
  "ips": ["203.0.113.50", "10.100.0.11"]
}
```

**Key Moirai Metadata Fields:**

| Metadata Key | Description |
|-------------|-------------|
| `cloud.tritoncompute:loadbalancer` | Set to `"true"` to activate Moirai |
| `cloud.tritoncompute:backend_service` | CNS service name of backends (without port) |
| `cloud.tritoncompute:backend_port` | Port on which backends listen |
| `cloud.tritoncompute:frontend_port` | Port the LB listens on (443 for HTTPS) |
| `cloud.tritoncompute:health_check_path` | HTTP health check endpoint |
| `cloud.tritoncompute:health_check_interval` | Health check interval in milliseconds |
| `cloud.tritoncompute:tls_domain` | Domain for automatic Let's Encrypt TLS certificate |
| `cloud.tritoncompute:tls_email` | Contact email for Let's Encrypt |

### Wait for Load Balancer to Be Running

```
GET /:login/machines/<lb-machine-uuid-001>
```

Poll until `state` is `"running"`.

---

## Step 8: Verify the Deployment

### 8a: Verify CNS DNS Resolution

CNS creates DNS records automatically. After instances are running, verify:

- **Private CNS (fabric):** `myapp-db.svc.<account>.<dc>.cns.triton.zone`
- **Private CNS (fabric):** `myapp-api.svc.<account>.<dc>.cns.triton.zone`
- **Private CNS (fabric):** `myapp-web.svc.<account>.<dc>.cns.triton.zone`
- **Public CNS:** `myapp-lb.svc.<account>.<dc>.triton.zone`

### 8b: Verify All Instances Are Running

```
GET /:login/machines?tag.app=myapp
```

**Response:**
```json
[
  { "id": "db-machine-uuid-001", "name": "myapp-db-01", "state": "running" },
  { "id": "api-machine-uuid-001", "name": "myapp-api-01", "state": "running" },
  { "id": "api-machine-uuid-002", "name": "myapp-api-02", "state": "running" },
  { "id": "web-machine-uuid-001", "name": "myapp-web-01", "state": "running" },
  { "id": "web-machine-uuid-002", "name": "myapp-web-02", "state": "running" },
  { "id": "lb-machine-uuid-001", "name": "myapp-lb-01", "state": "running" }
]
```

### 8c: Health Check via Load Balancer

Once DNS propagates (usually under 30 seconds for CNS), test the application:

```bash
# Health check
curl -k https://myapp.example.com/health

# Or via the public IP directly
curl -k https://203.0.113.50/health
```

### 8d: Verify Firewall Rules

```
GET /:login/fwrules
```

Confirm all rules are present and `enabled: true`.

---

## Step 9: Scaling

### Scale Up: Add More Web Instances

Create additional instances with the same CNS tag. Moirai automatically discovers new backends via DNS.

```
POST /:login/machines
Content-Type: application/json

{
  "name": "myapp-web-03",
  "image": "<app-image-uuid>",
  "package": "<app-package-uuid>",
  "networks": ["<WEB_NETWORK_ID>", "<API_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "web",
    "app": "myapp",
    "triton.cns.services": "myapp-web:8080"
  },
  "metadata": {
    "user-script": "<same user-script as other web instances>"
  }
}
```

No load balancer reconfiguration is needed. Moirai polls CNS DNS and picks up the new backend automatically.

### Scale Up: Add More API Instances

```
POST /:login/machines
Content-Type: application/json

{
  "name": "myapp-api-03",
  "image": "<app-image-uuid>",
  "package": "<app-package-uuid>",
  "networks": ["<API_NETWORK_ID>", "<DB_NETWORK_ID>"],
  "firewall_enabled": true,
  "tags": {
    "role": "api",
    "app": "myapp",
    "triton.cns.services": "myapp-api:3000"
  },
  "metadata": {
    "user-script": "<same user-script as other api instances>"
  }
}
```

### Graceful Scale Down

Before removing an instance, mark it as down in CNS so the load balancer stops routing traffic to it. Then wait for in-flight requests to drain before deleting.

**Step 1: Mark instance as down in CNS**

```
PUT /:login/machines/<machine-uuid>/tags
Content-Type: application/json

{
  "triton.cns.status": "down"
}
```

This removes the instance from DNS resolution. Moirai will stop sending new requests to it within one health check interval.

**Step 2: Wait for connections to drain (30-60 seconds)**

**Step 3: Delete the instance**

```
POST /:login/machines/<machine-uuid>?action=stop
```

Wait for the machine to stop, then:

```
DELETE /:login/machines/<machine-uuid>
```

---

## Step 10: Cleanup

To tear down the entire deployment, delete resources in reverse order:

### Delete Instances

```
DELETE /:login/machines/<lb-machine-uuid-001>
DELETE /:login/machines/<web-machine-uuid-001>
DELETE /:login/machines/<web-machine-uuid-002>
DELETE /:login/machines/<api-machine-uuid-001>
DELETE /:login/machines/<api-machine-uuid-002>
DELETE /:login/machines/<db-machine-uuid-001>
```

Instances must be in `stopped` state before deletion. Stop them first:

```
POST /:login/machines/<machine-uuid>?action=stop
```

### Delete Firewall Rules

```
DELETE /:login/fwrules/<fwrule-uuid>
```

Repeat for each rule.

### Delete Fabric Networks

```
DELETE /:login/fabrics/default/vlans/100/networks/<WEB_NETWORK_ID>
DELETE /:login/fabrics/default/vlans/200/networks/<API_NETWORK_ID>
DELETE /:login/fabrics/default/vlans/300/networks/<DB_NETWORK_ID>
```

Networks can only be deleted after all instances on them have been removed.

### Delete VLANs

```
DELETE /:login/fabrics/default/vlans/100
DELETE /:login/fabrics/default/vlans/200
DELETE /:login/fabrics/default/vlans/300
```

VLANs can only be deleted after all their fabric networks are removed.

---

## Summary

| Resource | Count | Network Placement |
|----------|-------|-------------------|
| VLANs | 3 | web:100, api:200, db:300 |
| Fabric Networks | 3 | web-network, api-network, db-network |
| Database Instances | 1 | db-network only (no NAT) |
| API Instances | 2 | api-network + db-network |
| Web Instances | 2 | web-network + api-network |
| Load Balancer | 1 | public + web-network |
| Firewall Rules | 7 | Tag-based per tier |

The architecture enforces strict tier isolation: the database can only be reached from the API tier, the API can only be reached from the web tier, and only the load balancer is exposed to the internet. Scaling any tier is a single API call -- CNS and Moirai handle discovery and routing automatically.
