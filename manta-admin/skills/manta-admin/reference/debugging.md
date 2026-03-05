# Debugging and Log Tracing

## Log Locations

| Service | Log Path | Format |
|---|---|---|
| Muskie (webapi) | `/var/log/manta/upload/muskie_<zone>_<timestamp>_<port>.log` | bunyan JSON |
| Muskie SMF | `/var/svc/log/*muskie*.log` | SMF |
| Moray | `/var/log/moray.log` | bunyan JSON |
| Mako (nginx access) | `/var/log/mako-access.log` | nginx combined |
| Mako (nginx error) | `/var/log/mako-error.log` | nginx error |
| Buckets API | `/var/svc/log/*buckets-api*.log` | SMF |
| Buckets MDAPI | `/var/svc/log/*buckets-mdapi*.log` | SMF |
| Config-agent | `/var/svc/log/*config-agent*.log` | SMF |
| Garbage collector | `/var/svc/log/*garbage*.log` | SMF |

Muskie runs multiple workers (ports 8081-8096), each with its own log file. Logs rotate hourly.

## Tracing a Request by ID

Every Manta response includes an `x-request-id` header. Use it to trace through the stack:

```bash
# Get request ID from a failed request
curl -si <MANTA_URL> 2>&1 | grep x-request-id

# Search muskie logs in a specific webapi zone
manta-oneach -z <zone_uuid> "cd /var/log/manta/upload && \
  grep -rh <request_id> muskie_*.log | bunyan"

# Search across all webapi zones (slower)
manta-oneach -s webapi "cd /var/log/manta/upload && \
  grep -rl <request_id> \$(ls -t muskie_*.log | head -16)"
```

The muskie log entry for a request contains:
- `req.method`, `req.url` — the request
- `res.statusCode` — response code
- `latency` — total request time in ms
- `sharks` — storage nodes contacted (for object reads/writes)
- `err` — error details if the request failed

## Debugging API Failures

### 500 Internal Server Error

Common causes:
- **Storage node unreachable** — muskie could not connect to a shark. Check `err` in the log for connection timeout or refused.
- **Metadata tier error** — moray or postgres returned an error. Check moray logs on the relevant shard.
- **Object missing from storage** — metadata says the object is on a shark, but the shark returns 404.

### 503 Service Unavailable

Common causes:
- **Muskie throttle** — request was rejected due to overload. Look for `ThrottledError` in muskie logs.
- **No storage nodes available** — all sharks are over utilization threshold (`MUSKIE_MAX_PERCENT_UTIL`).
- **Upstream timeout** — haproxy timed out waiting for a webapi backend.

### Slow Requests

If `x-response-time` is high (>5000ms):
- Check storage node I/O: `ssh <CN_IP> "zpool iostat zones 5 1"`
- Check nginx connection saturation: `manta-oneach -s storage "curl -s http://localhost/nginx_status"`
- Check for long-running postgres queries on the relevant shard

## Locating Object Data

To find where an object's data is physically stored:

```bash
# Get object metadata (from webapi zone or via API)
curl -si <MANTA_URL> | grep -E '(etag|durability-level|x-server-name)'

# The etag is the object UUID. Storage path is:
# /manta/<account_uuid>/<object_uuid>
# on each storage node listed in the metadata
```

The `mlocate` tool (available in some deployments) can also locate objects:
```bash
mlocate -f <account_login> -r <object_path>
```

## Bunyan Log Viewing

Muskie and moray logs are in bunyan JSON format. Use the `bunyan` CLI to make them readable:

```bash
# Pretty-print a log file
bunyan /var/log/manta/upload/muskie_*.log

# Filter by level (error and above)
bunyan -l error /var/log/manta/upload/muskie_*.log

# Filter by time range
bunyan --time "2024-01-01T00:00:00" /var/log/manta/upload/muskie_*.log

# Pipe grep results through bunyan
grep <request_id> muskie_*.log | bunyan
```

## Further Reading

See the [Manta Operator Guide](https://github.com/TritonDataCenter/manta/tree/master/docs/operator-guide) for complete debugging procedures.
