---
description: Triton DataCenter infrastructure management and deployment workflows
---

# Triton CLI Infrastructure Management

You are tasked with managing Triton DataCenter infrastructure using the triton CLI.

## Process

1. **Verify Prerequisites**
   - Check triton profile is configured (`triton profile get`)
   - Verify CNS is enabled on account (`triton account get`)
   - Note account UUID for CNS DNS names

2. **Understand the Task**
   - What infrastructure needs to be created/modified?
   - What networking requirements exist?
   - What security rules are needed?

3. **Apply Best Practices**
   - Always enable firewall on production instances with `--firewall`
   - Use CNS tags for service discovery: `triton.cns.services=service:port`
   - Use tag-based firewall rules for dynamic security
   - Keep databases on private fabric networks

4. **Execute Commands**
   - Use `triton instance create` for new instances
   - Use `triton fwrule create` for security rules
   - Use metadata for load balancer configuration

Use the triton skill for detailed patterns and complete command reference.
