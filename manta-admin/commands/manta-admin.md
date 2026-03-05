---
description: Diagnose and resolve Manta object storage administrative issues
---

# Manta Administration

You are tasked with diagnosing and resolving Manta object storage issues.

## Process

1. **Understand the symptom** — what is failing and how (HTTP status, frequency, timing)?
2. **Set up headnode access** — SSH to headnode, set PATH for manta-oneach/sdc-sapi/sapiadm
3. **Work top-down through the stack** — loadbalancer → webapi → metadata → storage
4. **Check service health** with `manta-oneach -s <service> "svcs -x"` across all tiers
5. **Identify the failing tier** — use response headers, timing, and log correlation
6. **Apply fix via SAPI** — never edit config files directly; use sapiadm for persistent changes
7. **Verify the fix** — run health checks, check nginx_status, confirm improvement

Use the manta-admin skill for detailed reference material, service inventory, and step-by-step workflows.
