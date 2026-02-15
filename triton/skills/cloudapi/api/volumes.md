# Volumes API

Manage persistent storage volumes (RFD 26). Volumes provide NFS-backed shared storage that can be mounted by one or more instances.

## Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/:login/volumes` | List volumes |
| GET | `/:login/volumes/:id` | Get volume details |
| POST | `/:login/volumes` | Create volume |
| DELETE | `/:login/volumes/:id` | Delete volume |

## Create Volume

```
POST /:login/volumes
{
  "name": "my-data",
  "type": "tritonnfs",
  "size": 10240,
  "networks": ["<network-uuid>"]
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| name | string | yes | Volume name (unique within account) |
| type | string | yes | Volume type, currently only `tritonnfs` |
| size | number | yes | Size in MiB |
| networks | array | yes | List of network UUIDs the volume is reachable on |

## Response Fields

| Field | Type | Description |
|-------|------|-------------|
| id | string (UUID) | Unique volume identifier |
| name | string | Volume name |
| type | string | Volume type (`tritonnfs`) |
| size | number | Size in MiB |
| state | string | Current state (see below) |
| networks | array | Network UUIDs the volume is attached to |
| filesystem_path | string | NFS mount path (e.g., `/exports/my-data`) |
| owner_uuid | string (UUID) | Owning account |
| created | string (ISO 8601) | Creation timestamp |

## Volume States

| State | Description |
|-------|-------------|
| `creating` | Provisioning in progress |
| `ready` | Available for use |
| `failed` | Provisioning failed |
| `deleting` | Deletion in progress |

## Mounting Volumes

Attach a volume to an instance at creation time using the `volumes` metadata field:

```
POST /:login/machines
{
  "name": "my-app",
  "image": "<image-uuid>",
  "package": "<package-uuid>",
  "networks": ["<network-uuid>"],
  "volumes": [
    {
      "name": "my-data",
      "type": "tritonnfs",
      "mountpoint": "/data"
    }
  ]
}
```

The instance and the volume must share at least one network.

## Delete Volume

```
DELETE /:login/volumes/:id
```

A volume can only be deleted when no instances reference it and its state is `ready` or `failed`.
