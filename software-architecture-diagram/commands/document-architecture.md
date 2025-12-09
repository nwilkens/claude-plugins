---
description: Create architecture documentation with .drawio diagrams for a software project
---

# Document Architecture Command

You are tasked with creating comprehensive architecture documentation for the user's software project.

## Process

1. **Understand the Project**
   - Ask the user about their project if not already clear
   - Identify the main components and their technologies
   - Understand the data flow and system boundaries

2. **Create Directory Structure**
   Create an `architecture/` directory with:
   - `README.md` with color scheme and diagram index
   - `logical-architecture.drawio` and `.png`
   - `physical-architecture.drawio` and `.png`
   - `layers/` subdirectory for tier diagrams
   - `diagrams/` subdirectory for flow diagrams
   - `infrastructure/` subdirectory for deployment diagrams

3. **Apply the Three-Tier Pattern**
   - **API Tier**: Gateways, load balancers, auth, rate limiting
   - **Business Tier**: Services, workers, orchestration
   - **Data Tier**: Databases, caches, storage

4. **Use Consistent Color Scheme**
   - API/Gateway: `#dbeafe` (bg), `#3b82f6` (border)
   - Business: `#e9d5ff` (bg), `#8b5cf6` (border)
   - Data: `#d1fae5` (bg), `#10b981` (border)
   - Support: `#f1f5f9` (bg), `#64748b` (border)

5. **Create File Triplets**
   For each diagram, create:
   - `.drawio` source file
   - `.png` exported image
   - `.md` documentation file

6. **Include GitHub Actions**
   Add `.github/workflows/drawio-export.yml` for automatic PNG export.

## Output

Create professional, well-documented architecture diagrams that follow industry best practices and are easy to maintain and update.

Use the software-architecture-diagram skill for detailed templates and examples.
