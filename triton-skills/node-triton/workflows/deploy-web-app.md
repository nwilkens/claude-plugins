# Deploy Three-Tier Web Application

Complete workflow for deploying a production-ready three-tier web application on Triton with load balancing, CNS service discovery, and proper network isolation.

## Architecture Overview

```
                         Internet
                             │
                       ┌─────┴─────┐
                       │   HTTPS   │
                       │  (443)    │
                       └─────┬─────┘
                             │
                    ┌────────┴────────┐
                    │  Load Balancer  │
                    │  (Moirai)       │
                    │  lb.svc.*.cns   │
                    └────────┬────────┘
                             │ :8080
          ┌──────────────────┼──────────────────┐
          │                  │                  │
     ┌────┴────┐        ┌────┴────┐        ┌────┴────┐
     │ web-01  │        │ web-02  │        │ web-03  │
     │ web.svc │        │ web.svc │        │ web.svc │
     └────┬────┘        └────┬────┘        └────┬────┘
          │                  │                  │
          └──────────────────┼──────────────────┘
                             │ :3000
          ┌──────────────────┼──────────────────┐
          │                  │                  │
     ┌────┴────┐        ┌────┴────┐        ┌────┴────┐
     │ api-01  │        │ api-02  │        │ api-03  │
     │ api.svc │        │ api.svc │        │ api.svc │
     └────┬────┘        └────┬────┘        └────┬────┘
          │                  │                  │
          └──────────────────┼──────────────────┘
                             │ :5432
                    ┌────────┴────────┐
                    │     db-01       │
                    │   (PostgreSQL)  │
                    │    db.svc       │
                    └─────────────────┘
```

## Prerequisites

### 1. Get Account Information
```bash
# Get account UUID (needed for CNS names)
triton account get

# Note your account UUID, e.g.: a1b2c3d4-e5f6-7890-abcd-ef1234567890
export ACCOUNT_UUID="YOUR_ACCOUNT_UUID"

# Get datacenter name
triton datacenters

# Note your datacenter, e.g.: us-central-1
export DATACENTER="YOUR_DATACENTER"
```

### 2. Enable CNS
```bash
triton account update triton_cns_enabled=true
```

### 3. Verify Available Images and Packages
```bash
triton image list name=base-64
triton package list | grep g4
```

## Step 1: Create Network Infrastructure

### Create VLANs
```bash
triton vlan create --name web 100
triton vlan create --name api 200
triton vlan create --name db 300
```

### Create Networks
```bash
# Web tier network
triton network create \
  --name web-net \
  --vlan-id 100 \
  --subnet 10.100.0.0/24 \
  --start-ip 10.100.0.10 \
  --end-ip 10.100.0.250 \
  --gateway 10.100.0.1

# API tier network
triton network create \
  --name api-net \
  --vlan-id 200 \
  --subnet 10.200.0.0/24 \
  --start-ip 10.200.0.10 \
  --end-ip 10.200.0.250 \
  --gateway 10.200.0.1

# Database tier network (no NAT - internal only)
triton network create \
  --name db-net \
  --vlan-id 300 \
  --subnet 10.30.0.0/24 \
  --start-ip 10.30.0.10 \
  --end-ip 10.30.0.250 \
  --no-nat
```

## Step 2: Create Firewall Rules

```bash
# SSH from bastion/jump host only (update with your bastion IP)
triton fwrule create -D "ssh-internal" \
  "FROM ip 10.0.0.0/8 TO all vms ALLOW tcp PORT 22"

# Public HTTPS to load balancer
triton fwrule create -D "https-public" \
  "FROM any TO tag role=lb ALLOW tcp PORT 443"

# HTTP redirect (optional)
triton fwrule create -D "http-public" \
  "FROM any TO tag role=lb ALLOW tcp PORT 80"

# LB to web tier
triton fwrule create -D "lb-to-web" \
  "FROM tag role=lb TO tag role=web ALLOW tcp PORT 8080"

# Web to API tier
triton fwrule create -D "web-to-api" \
  "FROM tag role=web TO tag role=api ALLOW tcp PORT 3000"

# API to database
triton fwrule create -D "api-to-db" \
  "FROM tag role=api TO tag role=db ALLOW tcp PORT 5432"

# Health checks from LB
triton fwrule create -D "lb-healthcheck" \
  "FROM tag role=lb TO tag role=web ALLOW tcp PORT 9090"
```

## Step 3: Create Database Tier

```bash
triton instance create \
  -n myapp-db-01 \
  -t triton.cns.services=myapp-db:5432 \
  -t app=myapp \
  -t role=db \
  -t env=production \
  -N db-net \
  --firewall \
  -w \
  base-64-lts g4-highmem-4G
```

**Verify:**
```bash
triton instance get myapp-db-01
triton instance tag list myapp-db-01
```

## Step 4: Create API Tier

```bash
# API server 1
triton instance create \
  -n myapp-api-01 \
  -t triton.cns.services=myapp-api:3000 \
  -t app=myapp \
  -t role=api \
  -t env=production \
  -N api-net \
  -N db-net \
  --firewall \
  -w \
  base-64-lts g4-highcpu-2G

# API server 2
triton instance create \
  -n myapp-api-02 \
  -t triton.cns.services=myapp-api:3000 \
  -t app=myapp \
  -t role=api \
  -t env=production \
  -N api-net \
  -N db-net \
  --firewall \
  -w \
  base-64-lts g4-highcpu-2G
```

**Verify CNS registration:**
```bash
# Wait 60 seconds for DNS propagation
sleep 60

dig myapp-api.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io A +short
```

## Step 5: Create Web Tier

```bash
# Web server 1
triton instance create \
  -n myapp-web-01 \
  -t triton.cns.services=myapp-web:8080 \
  -t app=myapp \
  -t role=web \
  -t env=production \
  -N web-net \
  -N api-net \
  --firewall \
  -w \
  base-64-lts g4-highcpu-1G

# Web server 2
triton instance create \
  -n myapp-web-02 \
  -t triton.cns.services=myapp-web:8080 \
  -t app=myapp \
  -t role=web \
  -t env=production \
  -N web-net \
  -N api-net \
  --firewall \
  -w \
  base-64-lts g4-highcpu-1G
```

**Verify CNS registration:**
```bash
sleep 60
dig myapp-web.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io A +short
```

## Step 6: Create Load Balancer

```bash
triton instance create \
  -n myapp-lb-01 \
  -t app=myapp \
  -t role=lb \
  -t env=production \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=https-http://443:myapp-web.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io:8080{check:/healthz},http://80:myapp-web.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io:8080" \
  -m cloud.tritoncompute:certificate_name=myapp.example.com \
  -m "cloud.tritoncompute:metrics_acl=10.0.0.0/8" \
  -m cloud.tritoncompute:metrics_port=9090 \
  -N web-net \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

**Note:** Update `myapp.example.com` with your actual domain and ensure DNS points to the LB IP before deploying (for Let's Encrypt).

**Verify:**
```bash
triton instance get myapp-lb-01
# Note the primaryIp - this is your load balancer's public IP
```

## Step 7: Configure DNS

Point your domain to the load balancer's public IP:

```
myapp.example.com  A  <LB_PUBLIC_IP>
www.myapp.example.com  CNAME  myapp.example.com
```

## Step 8: Deploy Application Code

SSH to each instance and deploy your application:

```bash
# Web tier
triton ssh myapp-web-01
triton ssh myapp-web-02

# API tier
triton ssh myapp-api-01
triton ssh myapp-api-02

# Database
triton ssh myapp-db-01
```

Configure applications with CNS service names:

**Web tier config:**
```
API_URL=http://myapp-api.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io:3000
```

**API tier config:**
```
DATABASE_URL=postgres://user:pass@myapp-db.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io:5432/myapp
```

## Verification

### Check All Instances
```bash
triton instance list -o name,state,primaryIp,tags | grep myapp
```

### Test CNS Resolution
```bash
# From any Triton instance
dig myapp-web.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io A +short
dig myapp-api.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io A +short
dig myapp-db.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io A +short
```

### Test Load Balancer
```bash
curl -I https://myapp.example.com/healthz
```

### Check LB Metrics
```bash
curl http://<LB_INTERNAL_IP>:9090/metrics
```

## Scaling

### Scale Up Web Tier
```bash
triton instance create \
  -n myapp-web-03 \
  -t triton.cns.services=myapp-web:8080 \
  -t app=myapp \
  -t role=web \
  -t env=production \
  -N web-net \
  -N api-net \
  --firewall \
  -w \
  base-64-lts g4-highcpu-1G

# Wait for DNS propagation
sleep 60

# Verify new instance in CNS
dig myapp-web.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io A +short
```

### Scale Down Web Tier
```bash
# 1. Remove from CNS (graceful)
triton instance metadata set myapp-web-03 triton.cns.status=down

# 2. Wait for connections to drain
sleep 60

# 3. Delete instance
triton instance delete -w myapp-web-03
```

### Scale API Tier
Same process as web tier - add/remove instances with the `myapp-api:3000` CNS tag.

## High Availability Load Balancer

For LB redundancy, add a second load balancer:

```bash
triton instance create \
  -n myapp-lb-02 \
  -t triton.cns.services=myapp-lb:443 \
  -t app=myapp \
  -t role=lb \
  -t env=production \
  -m cloud.tritoncompute:loadbalancer=true \
  -m "cloud.tritoncompute:portmap=https-http://443:myapp-web.svc.${ACCOUNT_UUID}.${DATACENTER}.cns.mnx.io:8080{check:/healthz}" \
  -m cloud.tritoncompute:certificate_name=myapp.example.com \
  -N web-net \
  --firewall \
  -w \
  cloud-load-balancer g4-highcpu-1G
```

Then configure DNS with both LB IPs or use a floating IP/external DNS failover.

## Cleanup

To tear down the entire stack:

```bash
# Delete instances
triton instance delete -w myapp-lb-01 myapp-lb-02
triton instance delete -w myapp-web-01 myapp-web-02 myapp-web-03
triton instance delete -w myapp-api-01 myapp-api-02
triton instance delete -w myapp-db-01

# Delete firewall rules
triton fwrule list | grep myapp
triton fwrule delete <RULE_IDS>

# Delete networks
triton network delete web-net
triton network delete api-net
triton network delete db-net

# Delete VLANs
triton vlan delete 100
triton vlan delete 200
triton vlan delete 300
```

## Summary

This deployment provides:
- **Load balancing** with TLS termination via Triton-Moirai
- **Auto-scaling backends** via CNS service discovery
- **Network isolation** with separate VLANs per tier
- **Firewall protection** with tag-based rules
- **High availability** capability with multiple LBs
- **Graceful scaling** using CNS status control
