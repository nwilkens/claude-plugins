# Metadata-Based Provisioning

Triton instances support metadata key-value pairs that can be set at creation time or updated later. The `user-script` metadata key is special -- its value is executed as a shell script on first boot, enabling fully automated instance provisioning.

## Key Concepts

- **Metadata**: Arbitrary key-value pairs attached to an instance, accessible from inside the instance
- **user-script**: A reserved metadata key whose value is executed as a shell script on first boot
- **user-data**: A reserved metadata key for passing arbitrary configuration data (not executed)
- **Metadata endpoint**: Instances can query their own metadata from within via the metadata API

## Setting Metadata at Instance Creation

Pass metadata as top-level keys prefixed with `metadata.` when creating an instance.

```
POST /:login/machines
{
  "name": "app-01",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "web",
  "metadata.user-script": "#!/bin/bash\nset -e\nexec > /var/log/provision.log 2>&1\napt-get update && apt-get install -y nginx\ntouch /root/.provisioned",
  "metadata.app-version": "1.2.0",
  "metadata.environment": "production"
}
```

## Updating Metadata After Creation

### Set or update metadata keys

```
POST /:login/machines/:id/metadata
{
  "user-script": "#!/bin/bash\necho 'updated script'",
  "config-version": "2"
}
```

### Get all metadata

```
GET /:login/machines/:id/metadata
```

### Get a specific metadata key

```
GET /:login/machines/:id/metadata/:key
```

### Delete a metadata key

```
DELETE /:login/machines/:id/metadata/:key
```

Note: Updating `user-script` after first boot does **not** re-execute the script. It only takes effect if the instance is reprovisioned or a new instance is created from the same template.

## User-Script Best Practices

### 1. Always use `set -e` to fail fast

Stop execution on the first error instead of silently continuing with a broken state.

```bash
#!/bin/bash
set -e
```

### 2. Log all output

Redirect stdout and stderr to a log file for debugging.

```bash
exec > /var/log/provision.log 2>&1
```

### 3. Wait for cloud-init (Linux VMs)

On bhyve/KVM Linux VMs, cloud-init may still be running when user-script starts.

```bash
if command -v cloud-init &>/dev/null; then
  cloud-init status --wait
fi
```

### 4. Create completion markers

Signal that provisioning finished successfully.

```bash
touch /root/.provisioned
```

### 5. Make scripts idempotent

Guard against partial re-runs. Check for existing state before modifying.

```bash
if [ -f /root/.provisioned ]; then
  echo "Already provisioned, skipping"
  exit 0
fi
```

### 6. Use variables for configuration

Pull values from instance metadata or set them at the top of the script for clarity.

```bash
APP_VERSION=$(mdata-get app-version 2>/dev/null || echo "latest")
ENVIRONMENT=$(mdata-get environment 2>/dev/null || echo "development")
```

## SmartOS Zones vs Linux VMs

### SmartOS Native Zones (joyent brand)

- User-script runs via SMF (Service Management Facility)
- Metadata accessed with `mdata-get` command
- pkgsrc is the package manager (`pkgin install`)
- No cloud-init

```bash
#!/bin/bash
set -e
exec > /var/log/provision.log 2>&1

# Read metadata
APP_VERSION=$(mdata-get app-version 2>/dev/null || echo "latest")

# Install packages via pkgsrc
pkgin -y install nodejs nginx

# Signal completion
mdata-put provision-status "complete"
touch /root/.provisioned
```

### Linux Zones (lx brand)

- User-script runs via SMF (inherited from SmartOS)
- Metadata accessed with `mdata-get`
- Use apt-get, yum, or apk depending on the image
- No cloud-init

```bash
#!/bin/bash
set -e
exec > /var/log/provision.log 2>&1

APP_VERSION=$(mdata-get app-version 2>/dev/null || echo "latest")

apt-get update
apt-get install -y curl git

touch /root/.provisioned
```

### Linux VMs (bhyve/KVM brand)

- User-script runs after cloud-init completes
- Metadata accessed via cloud-init data source or metadata API
- Standard Linux package managers
- Wait for cloud-init before proceeding

```bash
#!/bin/bash
set -e
exec > /var/log/provision.log 2>&1

# Wait for cloud-init
cloud-init status --wait

# Read metadata via curl to metadata API
APP_VERSION=$(curl -s http://169.254.169.254/metadata/v1/user-data | jq -r '.app_version // "latest"')

apt-get update
apt-get install -y docker.io

touch /root/.provisioned
```

## Common Provisioning Patterns

### Install Docker

```
POST /:login/machines
{
  "name": "docker-host-01",
  "image": "<ubuntu-22-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "docker",
  "metadata.user-script": "#!/bin/bash\nset -e\nexec > /var/log/provision.log 2>&1\ncloud-init status --wait\n\napt-get update\napt-get install -y ca-certificates curl gnupg\n\ninstall -m 0755 -d /etc/apt/keyrings\ncurl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg\nchmod a+r /etc/apt/keyrings/docker.gpg\n\necho \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo $VERSION_CODENAME) stable\" > /etc/apt/sources.list.d/docker.list\n\napt-get update\napt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin\n\nsystemctl enable docker\nsystemctl start docker\n\ntouch /root/.provisioned"
}
```

### Deploy from Git

```
POST /:login/machines
{
  "name": "app-01",
  "image": "<ubuntu-22-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "web",
  "tag.triton.cns.services": "webapp:8080",
  "metadata.repo-url": "https://github.com/myorg/myapp.git",
  "metadata.branch": "main",
  "metadata.user-script": "#!/bin/bash\nset -e\nexec > /var/log/provision.log 2>&1\ncloud-init status --wait\n\nREPO_URL=$(mdata-get repo-url 2>/dev/null || echo '')\nBRANCH=$(mdata-get branch 2>/dev/null || echo 'main')\n\napt-get update\napt-get install -y git nodejs npm\n\ngit clone --branch \"$BRANCH\" \"$REPO_URL\" /opt/app\ncd /opt/app\nnpm install --production\n\ncat > /etc/systemd/system/app.service <<EOF\n[Unit]\nDescription=Application\nAfter=network.target\n\n[Service]\nWorkingDirectory=/opt/app\nExecStart=/usr/bin/node server.js\nRestart=always\nEnvironment=NODE_ENV=production\nEnvironment=PORT=8080\n\n[Install]\nWantedBy=multi-user.target\nEOF\n\nsystemctl daemon-reload\nsystemctl enable app\nsystemctl start app\n\ntouch /root/.provisioned"
}
```

### Configure Monitoring Agent (Prometheus node_exporter)

```
POST /:login/machines
{
  "name": "web-02",
  "image": "<ubuntu-22-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "web",
  "tag.env": "production",
  "metadata.user-script": "#!/bin/bash\nset -e\nexec > /var/log/provision.log 2>&1\ncloud-init status --wait\n\nNODE_EXPORTER_VERSION=\"1.7.0\"\n\nuseradd --no-create-home --shell /bin/false node_exporter || true\n\ncurl -fsSL https://github.com/prometheus/node_exporter/releases/download/v${NODE_EXPORTER_VERSION}/node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64.tar.gz | tar xz -C /tmp\ncp /tmp/node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64/node_exporter /usr/local/bin/\nchown node_exporter:node_exporter /usr/local/bin/node_exporter\n\ncat > /etc/systemd/system/node_exporter.service <<EOF\n[Unit]\nDescription=Prometheus Node Exporter\nAfter=network.target\n\n[Service]\nUser=node_exporter\nExecStart=/usr/local/bin/node_exporter\nRestart=always\n\n[Install]\nWantedBy=multi-user.target\nEOF\n\nsystemctl daemon-reload\nsystemctl enable node_exporter\nsystemctl start node_exporter\n\ntouch /root/.provisioned"
}
```

### Install and Start k3s (Lightweight Kubernetes)

#### k3s Server (Control Plane)

```
POST /:login/machines
{
  "name": "k3s-server-01",
  "image": "<ubuntu-22-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "k3s-server",
  "tag.triton.cns.services": "k3s-server:6443",
  "metadata.k3s-token": "my-secure-cluster-token",
  "metadata.user-script": "#!/bin/bash\nset -e\nexec > /var/log/provision.log 2>&1\ncloud-init status --wait\n\nK3S_TOKEN=$(mdata-get k3s-token 2>/dev/null || echo 'default-token')\n\ncurl -sfL https://get.k3s.io | K3S_TOKEN=\"$K3S_TOKEN\" sh -\n\n# Wait for k3s to be ready\nuntil kubectl get nodes &>/dev/null; do sleep 2; done\n\ntouch /root/.provisioned"
}
```

#### k3s Agent (Worker Node)

```
POST /:login/machines
{
  "name": "k3s-agent-01",
  "image": "<ubuntu-22-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "k3s-agent",
  "metadata.k3s-token": "my-secure-cluster-token",
  "metadata.k3s-server-url": "https://k3s-server.svc.account.cns.triton.zone:6443",
  "metadata.user-script": "#!/bin/bash\nset -e\nexec > /var/log/provision.log 2>&1\ncloud-init status --wait\n\nK3S_TOKEN=$(mdata-get k3s-token 2>/dev/null || echo 'default-token')\nK3S_URL=$(mdata-get k3s-server-url 2>/dev/null)\n\ncurl -sfL https://get.k3s.io | K3S_URL=\"$K3S_URL\" K3S_TOKEN=\"$K3S_TOKEN\" sh -\n\ntouch /root/.provisioned"
}
```

### Set Up Application with systemd

```
POST /:login/machines
{
  "name": "api-01",
  "image": "<ubuntu-22-image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "firewall_enabled": true,
  "tag.role": "api",
  "tag.triton.cns.services": "api:3000",
  "metadata.app-url": "https://releases.example.com/myapp-v1.2.0.tar.gz",
  "metadata.user-script": "#!/bin/bash\nset -e\nexec > /var/log/provision.log 2>&1\ncloud-init status --wait\n\nAPP_URL=$(mdata-get app-url 2>/dev/null)\n\nuseradd --system --create-home --shell /bin/false appuser || true\n\nmkdir -p /opt/app\ncurl -fsSL \"$APP_URL\" | tar xz -C /opt/app\nchown -R appuser:appuser /opt/app\n\ncat > /etc/systemd/system/myapp.service <<EOF\n[Unit]\nDescription=My Application\nAfter=network.target\n\n[Service]\nUser=appuser\nWorkingDirectory=/opt/app\nExecStart=/opt/app/bin/server\nRestart=always\nRestartSec=5\nEnvironment=PORT=3000\n\n[Install]\nWantedBy=multi-user.target\nEOF\n\nsystemctl daemon-reload\nsystemctl enable myapp\nsystemctl start myapp\n\ntouch /root/.provisioned"
}
```

## Debugging Provisioning

### Check the provision log

SSH into the instance and inspect the log file:

```bash
cat /var/log/provision.log
```

### Check the provisioning marker

```bash
ls -la /root/.provisioned
```

If the file does not exist, provisioning either failed or did not complete.

### Check user-script execution status (SmartOS/lx zones)

```bash
svcs -l mdata:execute
```

The SMF service `mdata:execute` runs the user-script. Check its log:

```bash
cat $(svcs -L mdata:execute)
```

### Read metadata from inside the instance

On SmartOS and lx zones, use the `mdata-get` command:

```bash
mdata-get user-script        # View the script that was executed
mdata-get app-version         # Read custom metadata
mdata-list                    # List all metadata keys
```

On Linux VMs (bhyve/KVM), use the metadata HTTP endpoint:

```bash
curl -s http://169.254.169.254/metadata/v1/user-data
```

### Verify via the CloudAPI

Retrieve instance metadata remotely:

```
GET /:login/machines/:id/metadata
```

```
GET /:login/machines/:id/metadata/user-script
```

### Common Failures

| Symptom | Cause | Fix |
|---------|-------|-----|
| `/var/log/provision.log` missing | Script never ran | Check image supports user-scripts; verify metadata was set |
| Log shows error on first command | `set -e` stopped execution | Fix the failing command, reprovision |
| `mdata-get: No metadata` | Key not set at creation | Set metadata via `POST /:login/machines/:id/metadata` |
| `cloud-init status --wait` hangs | cloud-init not available | Remove the wait (zones do not use cloud-init) |
| Packages fail to install | No internet access | Ensure network has `internet_nat: true` or use a local mirror |
| Script runs but app not listening | Systemd service failed | Check `systemctl status <service>` and `journalctl -u <service>` |

## Best Practices Summary

1. **Always `set -e`** -- fail fast on errors
2. **Always log output** -- `exec > /var/log/provision.log 2>&1`
3. **Create markers** -- `touch /root/.provisioned` on success
4. **Make scripts idempotent** -- guard with `if [ -f /root/.provisioned ]`
5. **Wait for cloud-init** on bhyve/KVM VMs before installing packages
6. **Use `mdata-get`** for dynamic configuration instead of hardcoding values
7. **Pass secrets as metadata** -- they are only accessible from inside the instance
8. **Use metadata for parameterization** -- same script, different config per instance
9. **Keep scripts focused** -- one script per concern; use metadata to toggle features
10. **Test locally first** -- run the script manually on a test instance before automating
