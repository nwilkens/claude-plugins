# Triton CLI Command Reference

Complete reference for `triton` CLI commands.

## Instance Commands

### triton instance create
Create a new compute instance.

```bash
triton instance create [OPTIONS] IMAGE PACKAGE
```

**Key Options:**
| Option | Description |
|--------|-------------|
| `-n, --name NAME` | Instance name |
| `-t, --tag KEY=VALUE` | Add tags (repeatable) |
| `-m, --metadata KEY=VALUE` | Add metadata (repeatable) |
| `-N, --network NETWORK` | Attach to network (repeatable) |
| `--firewall` | Enable cloud firewall |
| `-w, --wait` | Wait for creation to complete |
| `-v, --volume NAME:MOUNTPOINT` | Mount a volume |
| `--script PATH` | User script to run on boot |

**Examples:**
```bash
# Basic instance
triton instance create -n web-01 -w base-64-lts g4-highcpu-1G

# With CNS service tag
triton instance create \
  -n api-01 \
  -t triton.cns.services=api:3000 \
  -t env=production \
  -w \
  base-64-lts g4-highcpu-2G

# On private network with firewall
triton instance create \
  -n db-01 \
  -t triton.cns.services=postgres:5432 \
  -N my-private-network \
  --firewall \
  -w \
  base-64-lts g4-highmem-4G
```

### triton instance list
List instances.

```bash
triton instance list [OPTIONS] [FILTERS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `-o FIELDS` | Output fields (comma-separated) |
| `-l, --long` | Long format output |
| `-j, --json` | JSON output |
| `-H` | No header row |

**Examples:**
```bash
# List all instances
triton instance list

# Custom columns
triton instance list -o name,state,primaryIp,tags

# Filter by state
triton instance list state=running

# JSON output for scripting
triton instance list -j | jq '.[] | select(.tags.env == "production")'
```

### triton instance get
Get instance details.

```bash
triton instance get INSTANCE
```

### triton instance delete
Delete instance(s).

```bash
triton instance delete [-w] INSTANCE [INSTANCE ...]
```

### triton instance start/stop/reboot
Control instance state.

```bash
triton instance start [-w] INSTANCE
triton instance stop [-w] INSTANCE
triton instance reboot [-w] INSTANCE
```

### triton instance ssh
SSH to an instance.

```bash
triton instance ssh [OPTIONS] INSTANCE [COMMAND]
```

**Options:**
| Option | Description |
|--------|-------------|
| `-u, --user USER` | SSH user (default: root) |
| `-i KEY` | SSH identity file |

### triton instance tag set
Set tags on an instance.

```bash
triton instance tag set [-w] INSTANCE KEY=VALUE [KEY=VALUE ...]
```

**CNS Service Tag Examples:**
```bash
# Single service
triton instance tag set myinstance triton.cns.services=web:8080

# Multiple services
triton instance tag set myinstance triton.cns.services=web:8080,api:3000

# With priority and weight (for SRV records)
triton instance tag set myinstance triton.cns.services=web:8080:priority=10:weight=50
```

### triton instance metadata set
Set metadata on an instance.

```bash
triton instance metadata set [-w] INSTANCE KEY=VALUE [KEY=VALUE ...]
```

**CNS Status Control:**
```bash
# Remove from CNS (for maintenance)
triton instance metadata set myinstance triton.cns.status=down

# Restore to CNS
triton instance metadata set myinstance triton.cns.status=up
```

---

## Network Commands

### triton network list
List available networks.

```bash
triton network list [OPTIONS]
```

**Examples:**
```bash
# List all networks
triton network list

# Show only public networks
triton network list public=true

# Long format
triton network list -l
```

### triton network get
Get network details.

```bash
triton network get NETWORK
```

### triton network create
Create a fabric network.

```bash
triton network create [OPTIONS] --vlan-id VLAN_ID
```

**Options:**
| Option | Description |
|--------|-------------|
| `--name NAME` | Network name |
| `--subnet CIDR` | Subnet (e.g., 10.0.0.0/24) |
| `--start-ip IP` | Start of IP range |
| `--end-ip IP` | End of IP range |
| `--gateway IP` | Gateway IP |
| `--resolver IP` | DNS resolver (repeatable) |
| `--no-nat` | Disable NAT |
| `--route CIDR=GW` | Static route (repeatable) |

**Example:**
```bash
triton network create \
  --name internal \
  --vlan-id 100 \
  --subnet 10.10.0.0/24 \
  --start-ip 10.10.0.10 \
  --end-ip 10.10.0.250 \
  --gateway 10.10.0.1
```

---

## VLAN Commands

### triton vlan list
List fabric VLANs.

```bash
triton vlan list
```

### triton vlan create
Create a fabric VLAN.

```bash
triton vlan create --name NAME VLAN_ID
```

**Example:**
```bash
triton vlan create --name production 100
triton vlan create --name staging 200
```

### triton vlan networks
List networks on a VLAN.

```bash
triton vlan networks VLAN_ID
```

---

## Firewall Rule Commands

### triton fwrule create
Create a firewall rule.

```bash
triton fwrule create [OPTIONS] RULE
```

**Options:**
| Option | Description |
|--------|-------------|
| `-D, --description TEXT` | Rule description |
| `-d, --disabled` | Create rule disabled |
| `--log` | Enable logging |

**Rule Syntax:**
```
FROM <source> TO <target> <action> <protocol> PORT <port>
```

**Source/Target Types:**
- `any` - Any IP
- `all vms` - All VMs in account
- `ip <IP>` - Specific IP
- `ip <CIDR>` - IP range
- `subnet <CIDR>` - Subnet
- `tag <key>` - VMs with tag key
- `tag <key>=<value>` - VMs with specific tag value
- `vm <uuid>` - Specific VM

**Actions:**
- `ALLOW` - Allow traffic
- `BLOCK` - Block traffic

**Examples:**
```bash
# Allow SSH from anywhere
triton fwrule create -D "ssh" "FROM any TO all vms ALLOW tcp PORT 22"

# Allow HTTPS to web tier
triton fwrule create -D "https" "FROM any TO tag role=web ALLOW tcp PORT 443"

# Allow internal service communication
triton fwrule create -D "web-to-api" \
  "FROM tag role=web TO tag role=api ALLOW tcp PORT 3000"

# Allow database access from API tier
triton fwrule create -D "api-to-db" \
  "FROM tag role=api TO tag role=db ALLOW tcp PORT 5432"

# Restrict SSH to internal network
triton fwrule create -D "ssh-internal" \
  "FROM ip 10.0.0.0/8 TO all vms ALLOW tcp PORT 22"
```

### triton fwrule list
List firewall rules.

```bash
triton fwrule list
```

### triton fwrule enable/disable
Enable or disable a rule.

```bash
triton fwrule enable RULE_ID
triton fwrule disable RULE_ID
```

### triton instance enable-firewall
Enable firewall on an instance.

```bash
triton instance enable-firewall INSTANCE
```

---

## Volume Commands

### triton volume create
Create an NFS volume.

```bash
triton volume create [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `-n, --name NAME` | Volume name |
| `-s, --size SIZE` | Volume size (e.g., 10G, 100G) |
| `-N, --network NETWORK` | Network for volume |
| `-w, --wait` | Wait for creation |

**Example:**
```bash
# Create volume
triton volume create -n app-data -s 100G -w

# Mount in instance
triton instance create \
  -n app-01 \
  -v app-data:/data \
  base-64-lts g4-highcpu-2G
```

### triton volume list
List volumes.

```bash
triton volume list
```

---

## Image and Package Commands

### triton image list
List available images.

```bash
triton image list [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--all` | Show all images (including deprecated) |
| `-j, --json` | JSON output |
| `name=PATTERN` | Filter by name |
| `type=TYPE` | Filter by type |

**Examples:**
```bash
# List all current images
triton image list

# Filter by name
triton image list name=base-64

# Show specific fields
triton image list -o name,version,os,published
```

### triton package list
List available packages (instance sizes).

```bash
triton package list [OPTIONS]
```

**Examples:**
```bash
# List all packages
triton package list

# Filter by memory
triton package list memory=2048

# Custom output
triton package list -o name,memory,disk,vcpus
```

---

## Profile Commands

### triton profile list
List CLI profiles.

```bash
triton profile list
```

### triton profile create
Create a new profile.

```bash
triton profile create
```

Interactive prompts for:
- CloudAPI URL
- Account name
- SSH key

### triton profile set-current
Switch active profile.

```bash
triton profile set-current PROFILE_NAME
```

### triton profile get
Get current profile details.

```bash
triton profile get [PROFILE_NAME]
```

---

## Account Commands

### triton account get
Get account details.

```bash
triton account get
```

### triton account update
Update account settings.

```bash
triton account update KEY=VALUE [KEY=VALUE ...]
```

**Enable CNS:**
```bash
triton account update triton_cns_enabled=true
```

---

## Utility Commands

### triton info
Show account summary.

```bash
triton info
```

### triton datacenters
List available datacenters.

```bash
triton datacenters
```

### triton env
Output shell environment variables.

```bash
eval $(triton env)
```
