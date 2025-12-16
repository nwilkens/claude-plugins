# PostgreSQL DBA Plugin

A Claude Code plugin for PostgreSQL database administration, performance analysis, and optimization.

## Features

- **Systematic Performance Investigation**: Step-by-step workflow for diagnosing database issues
- **Comprehensive Diagnostics**: Pre-built queries for bloat, indexes, locks, connections, and more
- **Query Optimization Guidance**: Patterns for rewriting slow queries
- **Index Strategy**: Recommendations for creating effective indexes
- **Configuration Tuning**: Best practices for PostgreSQL parameters

## Installation

### Via Plugin Marketplace

```bash
/plugin marketplace add nwilkens/claude-plugins
/plugin install postgresql-dba@claude-plugins
```

### Manual Installation

Copy the `postgresql-dba/` directory to your Claude Code plugins location.

## Usage

### Slash Command

Use the `/dba` command to start a performance investigation:

```
/dba
```

This will guide you through:
1. Gathering symptoms and context
2. Running appropriate diagnostic queries
3. Analyzing results
4. Providing recommendations

### Skill (Auto-Invoked)

The plugin's skill is automatically available when discussing PostgreSQL topics. Ask questions like:

- "Why is my PostgreSQL query slow?"
- "How do I find unused indexes?"
- "What's causing lock contention?"
- "How should I configure autovacuum?"

## Diagnostic Queries Included

| Category | Purpose |
|----------|---------|
| Table Bloat | Identify tables needing VACUUM |
| Index Usage | Find most/least used indexes |
| Unused Indexes | Candidates for removal |
| Missing Indexes | Tables with excessive sequential scans |
| Long Queries | Currently running slow queries |
| Lock Contention | Blocked and blocking processes |
| Connections | Connection counts by state |
| Checkpoints | WAL and checkpoint statistics |
| Table Sizes | Storage overview |

## Query Optimization Patterns

The plugin includes solutions for common performance issues:

- Correlated subqueries → JOINs
- NOT IN → NOT EXISTS
- Counting recent rows → Pre-aggregation
- Full table scans → Proper indexing

## Index Recommendations

Guidance for creating:

- **Composite indexes** for multi-column WHERE clauses
- **Covering indexes** for index-only scans
- **Partial indexes** for filtered queries
- **BRIN indexes** for time-series data

## Configuration Tuning

Best practices for:

- Memory settings (shared_buffers, work_mem)
- Connection settings (max_connections, timeouts)
- Vacuum settings (scale factors, cost limits)

## Requirements

- PostgreSQL 12 or higher
- Access to `pg_stat_*` views
- For full functionality: `pg_stat_statements` extension

## Example Session

```
User: /dba

Claude: I'll help you investigate PostgreSQL performance issues.
What symptoms are you seeing?
- Slow queries
- High CPU
- Connection issues
- Lock contention
- Something else?

User: Slow queries, especially on the orders table

Claude: Let me run some diagnostics on the orders table...
[Runs bloat check, index usage, sequential scan analysis]

Here's what I found:
1. **Root Cause**: Missing index on `created_at` column
2. **Evidence**: 45,000 sequential scans averaging 2.3M rows each
3. **Impact**: ~500ms per query, affecting 200 queries/minute
4. **Recommendation**:
   CREATE INDEX CONCURRENTLY idx_orders_created_at ON orders (created_at);
5. **Trade-offs**: Index will add ~50MB storage, slight write overhead
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests if applicable
4. Submit a pull request

## License

MIT License
