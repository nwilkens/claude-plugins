# Metadata-Based Provisioning

Use instance metadata for automated provisioning instead of manual SSH configuration.

## Overview

Triton supports running scripts on instance boot through the `user-script` metadata key. For Linux VMs (KVM/bhyve), this integrates with cloud-init. For SmartOS zones, scripts run directly via the zone's init system.

## User-Script Metadata

The `user-script` metadata key specifies a script that runs on first boot.

### Basic Usage

```bash
triton instance create \
  -n my-server \
  -m "user-script=#!/bin/bash
apt-get update
apt-get install -y nginx
systemctl enable nginx
systemctl start nginx
" \
  ubuntu-24.04 m5d.small
```

### Using --script Flag

For longer scripts, use the `--script` flag to read from a file:

```bash
# Create provision script
cat > provision.sh << 'EOF'
#!/bin/bash
set -e

# Wait for cloud-init to complete
cloud-init status --wait

# Install packages
apt-get update
apt-get install -y nginx certbot python3-certbot-nginx

# Configure nginx
cat > /etc/nginx/sites-available/default << 'NGINX'
server {
    listen 80;
    server_name _;
    location / {
        proxy_pass http://localhost:3000;
    }
}
NGINX

systemctl restart nginx

# Mark provisioning complete
touch /root/.provisioned
EOF

# Create instance with script
triton instance create \
  -n web-01 \
  --script provision.sh \
  -w \
  ubuntu-24.04 m5d.medium
```

### --script vs -m user-script=

| Feature | `--script PATH` | `-m user-script=` |
|---------|-----------------|-------------------|
| Source | External file | Inline string |
| Best for | Long scripts | Short commands |
| Readability | Better for complex scripts | Good for one-liners |
| Version control | Easy to track changes | Embedded in command |

Both methods set the same `user-script` metadata key.

## Best Practices

### 1. Always Use set -e

Fail fast on errors to avoid partial configurations:

```bash
#!/bin/bash
set -e  # Exit immediately on error
set -o pipefail  # Fail on pipe errors
```

### 2. Wait for Cloud-Init (Linux VMs)

Cloud-init may still be running when your script starts:

```bash
#!/bin/bash
set -e

# Wait for cloud-init to complete networking setup
cloud-init status --wait

# Now safe to install packages
apt-get update
apt-get install -y my-package
```

### 3. Create Completion Markers

Track provisioning state for debugging and idempotency:

```bash
#!/bin/bash
set -e

# Check if already provisioned
if [ -f /root/.provisioned ]; then
    echo "Already provisioned, skipping"
    exit 0
fi

# ... provisioning steps ...

# Mark complete with timestamp
date > /root/.provisioned
```

### 4. Log Output for Debugging

Redirect output for troubleshooting:

```bash
#!/bin/bash
exec > /var/log/provision.log 2>&1
set -ex  # -x prints each command

echo "Starting provisioning at $(date)"
# ... provisioning steps ...
echo "Completed provisioning at $(date)"
```

### 5. Make Scripts Idempotent

Scripts may run multiple times (reboots, reprovisioning):

```bash
#!/bin/bash
set -e

# Only install if not present
if ! command -v nginx &> /dev/null; then
    apt-get update
    apt-get install -y nginx
fi

# Only configure if not already done
if ! grep -q "my-config" /etc/nginx/nginx.conf; then
    # Add configuration
    echo "my-config" >> /etc/nginx/nginx.conf
fi
```

## Common Provisioning Patterns

### Install Docker

```bash
triton instance create \
  -n docker-host \
  -m "user-script=#!/bin/bash
set -e
cloud-init status --wait
curl -fsSL https://get.docker.com | sh
usermod -aG docker admin
systemctl enable docker
touch /root/.docker-installed
" \
  -w \
  ubuntu-24.04 m5d.large
```

### Configure SSH Keys for Application User

```bash
triton instance create \
  -n app-server \
  -m "user-script=#!/bin/bash
set -e
cloud-init status --wait
useradd -m -s /bin/bash deploy
mkdir -p /home/deploy/.ssh
echo 'ssh-rsa AAAA... deploy@company.com' >> /home/deploy/.ssh/authorized_keys
chown -R deploy:deploy /home/deploy/.ssh
chmod 700 /home/deploy/.ssh
chmod 600 /home/deploy/.ssh/authorized_keys
" \
  -w \
  ubuntu-24.04 m5d.medium
```

### Install and Configure Monitoring Agent

```bash
triton instance create \
  -n monitored-server \
  -m "user-script=#!/bin/bash
set -e
cloud-init status --wait
curl -fsSL https://monitoring.example.com/install.sh | sh
cat > /etc/monitoring-agent.conf << 'EOF'
server=monitoring.svc.account-uuid.dc.cns.mnx.io
port=8125
EOF
systemctl enable monitoring-agent
systemctl start monitoring-agent
" \
  -t triton.cns.services=app:8080 \
  -w \
  ubuntu-24.04 m5d.medium
```

### Set Up Application from Git

```bash
triton instance create \
  -n app-server \
  -m "user-script=#!/bin/bash
set -e
cloud-init status --wait

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
apt-get install -y nodejs git

# Clone and setup application
git clone https://github.com/company/app.git /opt/app
cd /opt/app
npm ci --production
npm run build

# Create systemd service
cat > /etc/systemd/system/app.service << 'EOF'
[Unit]
Description=My Application
After=network.target

[Service]
Type=simple
User=nobody
WorkingDirectory=/opt/app
ExecStart=/usr/bin/node dist/index.js
Restart=always

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable app
systemctl start app
touch /root/.app-deployed
" \
  -t triton.cns.services=app:3000 \
  -w \
  ubuntu-24.04 m5d.medium
```

## SmartOS Zone Provisioning

For SmartOS zones (not KVM/bhyve), user-scripts run directly via SMF:

```bash
triton instance create \
  -n smartos-zone \
  -m "user-script=#!/bin/bash
pkgin -y install nginx
svcadm enable nginx
" \
  base-64-lts g4-highcpu-1G
```

Note: SmartOS zones don't use cloud-init, so skip the `cloud-init status --wait` step.

## Debugging Provisioning

### Check Script Output (Linux)

```bash
# Cloud-init logs
triton instance ssh my-server "cat /var/log/cloud-init-output.log"

# Custom log if you configured one
triton instance ssh my-server "cat /var/log/provision.log"
```

### Check Script Output (SmartOS)

```bash
# SMF log for user-script service
triton instance ssh my-server "cat /var/svc/log/system-smartdc-mdata:execute.log"
```

### Verify Metadata Was Set

```bash
triton instance get my-server | grep -A5 metadata
```

## See Also

- [Deploy Kubernetes](../workflows/deploy-kubernetes.md) - Complete k3s deployment using user-scripts
- [CNS Service Discovery](cns-service-discovery.md) - DNS-based service discovery
- [Deploy Web App](../workflows/deploy-web-app.md) - Full 3-tier application deployment
