# Images (Datasets) API

Manage machine images used for provisioning instances.

## Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/:login/images` | List images |
| GET | `/:login/images/:id` | Get image details |
| POST | `/:login/images` | Create custom image from machine |
| POST | `/:login/images/:id?action=clone` | Clone image to your account |
| POST | `/:login/images/:id?action=update` | Update image metadata |
| POST | `/:login/images/:id?action=export` | Export image to Manta |
| DELETE | `/:login/images/:id` | Delete image |

## List Images

```
GET /:login/images
```

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| name | string | Filter by image name |
| os | string | Filter by OS (e.g., `smartos`, `linux`, `windows`) |
| version | string | Filter by version |
| public | boolean | `true` for public images, `false` for private |
| state | string | Filter by state (`active`, `disabled`, `unactivated`) |
| owner | string | Filter by owner account UUID |
| type | string | Filter by image type |

## Image Types

| Type | Description |
|------|-------------|
| `zone-dataset` | SmartOS native zone (joyent brand) |
| `lx-dataset` | Linux container zone (lx brand) |
| `zvol` | Virtual disk for KVM/bhyve VMs |

## Response Fields

| Field | Type | Description |
|-------|------|-------------|
| id | string (UUID) | Unique image identifier |
| name | string | Image name |
| version | string | Semver version string |
| os | string | Operating system |
| type | string | Image type (`zone-dataset`, `lx-dataset`, `zvol`) |
| description | string | Human-readable description |
| requirements | object | Provisioning constraints |
| requirements.min_memory | number | Minimum RAM in MiB |
| requirements.min_disk | number | Minimum disk in MiB |
| published_at | string (ISO 8601) | Publication timestamp |
| state | string | `active`, `disabled`, or `unactivated` |
| owner | string (UUID) | Account that owns the image |
| public | boolean | Whether image is publicly visible |
| tags | object | Key-value metadata tags |
| files | array | Image file details (size, compression, sha1) |

## Create Custom Image

Create an image from an existing stopped machine.

```
POST /:login/images
{
  "machine": "<machine-uuid>",
  "name": "my-app-image",
  "version": "1.0.0",
  "description": "Custom app image"
}
```

The machine must be in the `stopped` state. The resulting image will be private to your account.

## Update Image Metadata

```
POST /:login/images/:id?action=update
{
  "name": "updated-name",
  "description": "Updated description",
  "tags": { "env": "production" }
}
```

## Clone Image

Copy a public image into your account as a private image.

```
POST /:login/images/:id?action=clone
```

## Delete Image

```
DELETE /:login/images/:id
```

Only custom (private) images owned by your account can be deleted.
