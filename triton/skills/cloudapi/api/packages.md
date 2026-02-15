# Packages (Sizing) API

List and inspect pre-defined instance sizing packages. Packages define the compute, memory, disk, and other resource limits for provisioned instances.

Packages are configured by the operator. Users select from the available set when creating instances.

## Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/:login/packages` | List packages |
| GET | `/:login/packages/:id` | Get package details |

## List Packages

```
GET /:login/packages
```

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| name | string | Filter by package name |
| memory | number | Filter by RAM (MiB) |
| disk | number | Filter by disk quota (MiB) |
| swap | number | Filter by swap (MiB) |
| lwps | number | Filter by max lightweight processes |
| vcpus | number | Filter by virtual CPUs |
| version | string | Filter by version |
| group | string | Filter by package group |

## Response Fields

| Field | Type | Description |
|-------|------|-------------|
| id | string (UUID) | Unique package identifier |
| name | string | Package name (e.g., `g1.xsmall`) |
| memory | number | RAM in MiB |
| disk | number | Disk quota in MiB |
| swap | number | Swap space in MiB |
| vcpus | number | Virtual CPU count (0 = no cap for zones) |
| lwps | number | Max lightweight processes |
| version | string | Package version |
| group | string | Logical group (e.g., `general`, `compute`, `memory`) |
| description | string | Human-readable description |

## Example Response

```json
{
  "id": "7b17343c-94af-6266-e0e8-893a3b9993d0",
  "name": "g1.xsmall",
  "memory": 1024,
  "disk": 25600,
  "swap": 2048,
  "vcpus": 1,
  "lwps": 4000,
  "version": "1.0.0",
  "group": "general",
  "description": "General Purpose 1GB RAM, 1 vCPU, 25GB Disk"
}
```

## Usage

Specify a package by UUID or name when creating an instance:

```
POST /:login/machines
{
  "name": "my-instance",
  "image": "<image-uuid>",
  "package": "g1.xsmall",
  ...
}
```

When using a name, the API resolves it to the matching package UUID. If multiple packages share a name, use the UUID to be explicit.
