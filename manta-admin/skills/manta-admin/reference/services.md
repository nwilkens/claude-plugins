# Manta Service Inventory

## Service Tiers

### Load Balancer (haproxy/muppet)
- **Function:** TLS termination, routes requests to webapi or buckets-api
- **Routing logic:** SigV4 auth header or `/buckets` path → buckets-api; default → webapi (secure_api)
- **Config:** `/opt/smartdc/muppet/etc/haproxy.cfg`
- **Health check:** `option httpchk GET /ping` against backends
- **Service discovery:** muppet watches ZooKeeper for backend zone changes and updates haproxy config automatically

### Webapi (Muskie)
- **Function:** Directory API — handles PUT/GET/DELETE for objects, directories, and multipart uploads
- **Ports:** 80 (secure), 81 (insecure), workers on 8081-8096
- **Logs:** `/var/log/manta/upload/muskie_<zone>_<timestamp>_<port>.log` (bunyan JSON)
- **SMF log:** `/var/svc/log/*muskie*.log`
- **Metadata path:** webapi → electric-moray → moray → postgres
- **Storage path:** webapi → mako (nginx on storage nodes)
- **Object writes:** selects candidate storage nodes via storinfo, issues PUT with 100-continue to each, streams data, then updates metadata
- **Object reads:** contacts storage nodes listed in metadata, streams from whichever responds first

### Buckets API
- **Function:** S3-compatible API for bucket operations (experimental)
- **Metadata path:** buckets-api → buckets-mdapi → buckets-postgres
- **Storage path:** buckets-api → mako (nginx on storage nodes)

### Electric-Moray
- **Function:** Consistent hashing proxy for directory API moray shards
- **How it works:** hashes the directory name of the object path to determine which shard handles the request, then proxies to the correct moray instance

### Moray
- **Function:** Key-value store interface over postgres for directory API metadata
- Multiple shards, each with multiple instances
- Clients never talk to postgres directly — moray tracks the replication topology and directs reads/writes to the primary
- Instances are shard-specific: "1.moray", "2.moray", etc.
- **Log:** `/var/log/moray.log` (bunyan)

### Buckets-MDAPI
- **Function:** Metadata API for buckets, equivalent to moray for directory API

### Buckets-MDPlacement
- **Function:** Placement/sharding for bucket objects
- Unlike electric-moray, not on the data path — hash ring is cached in buckets-api at startup

### Postgres (Directory)
- **Function:** PostgreSQL database for directory API metadata
- **Replication:** each shard has primary, synchronous secondary, and async peer
- **Managed by:** manatee (uses ZooKeeper for leader election)
- **Failover:** primary crashes → shard goes read-only → secondary promoted → async promoted → once caught up, shard goes read-write → old primary becomes async
- **Sharding:** metadata is sharded by consistent hashing on the directory name

### Buckets-Postgres
- **Function:** PostgreSQL clusters for buckets metadata (primary, sync, async per shard)

### Storage (Mako)
- **Function:** nginx-based object storage, serves raw object bytes via PUT/GET
- **Config:** `/opt/smartdc/mako/nginx/conf/nginx.conf` (SAPI-managed template)
- **Data:** `/manta/$account_uuid/$object_uuid` (ZFS delegated dataset)
- **Access log:** `/var/log/mako-access.log`
- **Error log:** `/var/log/mako-error.log`
- **Shared resource:** Both webapi and buckets-api read/write to all storage nodes
- **Sub-components:**
  - **mako:** nginx for object PUT/GET
  - **minnow:** reports storage capacity to metadata tier periodically
  - **rebalancer-agent:** processes object copy requests from the rebalancer service
  - **garbage-deleter:** deletes objects from `/manta` based on GC instructions

### Storinfo
- **Function:** Storage metadata cache and picker — caches storage node capacity data, selects storage nodes for writes
- Alternative to the local "picker" function in muskie (controlled by `WEBAPI_USE_PICKER` SAPI variable)

### AuthCache (Mahi)
- **Function:** Read-only cache of Joyent/Triton account database, backed by Redis
- All front door requests authenticated against this cache

### Garbage Collector
- **Function:** Multi-component system for removing deleted objects
- **garbage-dir-consumer:** consumes deletion records from `manta_fastdelete_queue` (directory API)
- **garbage-buckets-consumer:** consumes deletion records from buckets-mdapi
- **garbage-uploader:** sends deletion instructions to storage zones
- **garbage-mpu-cleaner:** cleans up finalized multipart uploads
- Each runs as its own SMF service with logs in `/var/svc/log`

### Nameservice (Binder/ZooKeeper)
- **Function:** Internal service discovery via ZooKeeper + DNS
- Zones publish their IPs to ZooKeeper via "registrar" SMF service
- Other zones discover services via DNS lookups against binder
- DNS TTL is 60s — takes up to a minute for failed zones to fall out of DNS
- Must have odd number of instances (3 or 5) for consensus

### Rebalancer
- **Function:** Orchestrates migration of objects between storage zones
- Used for storage zone evacuation during hardware replacement

### Madtom
- **Function:** Web-based dashboard showing state of all Manta components
- Access via port 80 on the madtom zone IP

## Network Topology

Manta zones use two networks:
- **Manta network** — high-volume inter-service communication
- **Admin network** — management, CN access

### Discovering Zone-to-CN Mapping

```bash
# List all zones with CN admin IPs
manta-adm show
# Output columns: SERVICE, SH, ZONENAME, GZ ADMIN IP

# List compute nodes with hostnames and storage IDs
manta-adm cn -o host,server_uuid,admin_ip
manta-adm cn -o host,storage_ids storage

# Show zones across all datacenters
manta-adm show -a
```

Storage nodes have a `MANTA_STORAGE_ID` in SAPI metadata (e.g., `1.stor`, `2.stor`).

### Accessing Zones

From the headnode, use `manta-oneach` for most operations:
```bash
manta-oneach -s <service> "<command>"       # All zones for a service
manta-oneach -z <full_zone_uuid> "<command>" # Specific zone (needs full UUID)
```

For interactive access:
```bash
manta-login <service>                       # Login to a zone by service name
manta-login <partial_zone_uuid>             # Login to a zone by partial UUID
```

For operations that need CN-level access or when `manta-oneach` times out:
```bash
ssh <CN_IP> "zlogin <zone_uuid> <command>"
```
