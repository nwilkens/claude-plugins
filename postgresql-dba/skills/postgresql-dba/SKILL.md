# PostgreSQL DBA Skill

Expert PostgreSQL database administration for performance analysis, optimization, and troubleshooting.

## When to Use This Skill

Use this skill when the user asks to:
- Analyze PostgreSQL performance issues
- Optimize slow queries
- Investigate database bloat or vacuum issues
- Analyze index usage or create new indexes
- Debug lock contention or deadlocks
- Review query execution plans
- Tune PostgreSQL configuration
- Investigate connection issues

## Core Diagnostic Approach

### 1. Performance Investigation Checklist

When investigating performance issues, always check:

1. **Active Queries** - Long-running or blocked queries
2. **Table Statistics** - Bloat, dead tuples, vacuum status
3. **Index Health** - Usage patterns, unused indexes, missing indexes
4. **Lock Contention** - Blocked processes, lock waits
5. **Connection Pool** - Connection counts, states
6. **System Resources** - CPU, memory, I/O (if accessible)

### 2. Key System Views

| View | Purpose |
|------|---------|
| `pg_stat_activity` | Current connections and queries |
| `pg_stat_user_tables` | Table statistics and vacuum info |
| `pg_stat_user_indexes` | Index usage statistics |
| `pg_locks` | Current lock information |
| `pg_stat_bgwriter` | Background writer and checkpoint stats |
| `pg_stat_statements` | Query performance history (extension) |

### 3. Query Optimization Hierarchy

When optimizing queries, follow this order:

1. **Rewrite the query** - Often the biggest gains
2. **Add/modify indexes** - Second biggest impact
3. **Update statistics** - Run ANALYZE on affected tables
4. **Adjust parameters** - work_mem, effective_cache_size, etc.
5. **Schema changes** - Partitioning, denormalization (last resort)

## Advanced Diagnostics

### Table Bloat Estimation

```sql
WITH constants AS (
  SELECT current_setting('block_size')::numeric AS bs,
         23 AS hdr, 8 AS ma
),
bloat_info AS (
  SELECT
    schemaname, tablename,
    cc.reltuples::bigint,
    cc.relpages::bigint,
    COALESCE((
      SELECT sum(1 + coalesce(s.avg_width, 1024))
      FROM pg_stats s WHERE s.tablename = a.tablename
    ), 0) AS avg_row_width,
    (bs - hdr % ma - ma *
     ceil((hdr % ma)::numeric / ma))::numeric AS effective_blocksize
  FROM pg_catalog.pg_stat_user_tables a
  JOIN pg_class cc ON cc.relname = a.tablename
  CROSS JOIN constants
)
SELECT
  schemaname, tablename,
  pg_size_pretty(relpages * 8192::bigint) as current_size,
  round(100 * (relpages - ceil(reltuples * avg_row_width / effective_blocksize))::numeric /
        nullif(relpages, 0), 1) as bloat_pct
FROM bloat_info
WHERE relpages > 100
ORDER BY bloat_pct DESC NULLS LAST
LIMIT 20;
```

### Identifying Hot Tables

```sql
SELECT schemaname, relname,
       seq_scan + idx_scan as total_scans,
       n_tup_ins as inserts,
       n_tup_upd as updates,
       n_tup_del as deletes,
       n_tup_hot_upd as hot_updates,
       round(100 * n_tup_hot_upd::numeric / nullif(n_tup_upd, 0), 1) as hot_update_pct
FROM pg_stat_user_tables
ORDER BY n_tup_upd + n_tup_ins + n_tup_del DESC
LIMIT 20;
```

### Index Efficiency Analysis

```sql
SELECT schemaname, tablename, indexname,
       idx_scan,
       pg_size_pretty(pg_relation_size(indexrelid)) as size,
       round(100 * idx_scan::numeric / nullif(
         (SELECT sum(idx_scan) FROM pg_stat_user_indexes i2
          WHERE i2.tablename = pg_stat_user_indexes.tablename), 0), 1) as usage_pct
FROM pg_stat_user_indexes
WHERE schemaname NOT IN ('pg_catalog', 'information_schema')
ORDER BY pg_relation_size(indexrelid) DESC
LIMIT 30;
```

### Wait Event Analysis (PostgreSQL 10+)

```sql
SELECT wait_event_type, wait_event, count(*),
       round(100 * count(*)::numeric / sum(count(*)) over(), 1) as pct
FROM pg_stat_activity
WHERE state != 'idle' AND wait_event IS NOT NULL
GROUP BY wait_event_type, wait_event
ORDER BY count(*) DESC;
```

## Configuration Tuning

### Memory Settings

| Parameter | Starting Value | Notes |
|-----------|---------------|-------|
| `shared_buffers` | 25% of RAM | Max ~8GB for most workloads |
| `effective_cache_size` | 75% of RAM | Hint for planner, not allocation |
| `work_mem` | 64-256MB | Per-operation, be careful |
| `maintenance_work_mem` | 512MB-1GB | For VACUUM, CREATE INDEX |

### Connection Settings

| Parameter | Typical Value | Notes |
|-----------|--------------|-------|
| `max_connections` | 100-200 | Use connection pooler for more |
| `idle_in_transaction_session_timeout` | 30min | Prevent abandoned transactions |
| `statement_timeout` | 30-60s | Prevent runaway queries |

### Vacuum Settings

| Parameter | Typical Value | Notes |
|-----------|--------------|-------|
| `autovacuum_vacuum_scale_factor` | 0.1 | Trigger at 10% dead tuples |
| `autovacuum_analyze_scale_factor` | 0.05 | Trigger ANALYZE at 5% |
| `autovacuum_vacuum_cost_delay` | 2ms | Reduce for faster vacuum |
| `autovacuum_vacuum_cost_limit` | 1000 | Increase for faster vacuum |

## Index Strategy

### Index Types and Use Cases

| Index Type | Best For |
|------------|----------|
| B-tree (default) | Equality and range queries, ORDER BY |
| Hash | Equality-only queries (rarely needed) |
| GiST | Full-text search, geometric data, ranges |
| GIN | Arrays, JSONB, full-text search |
| BRIN | Very large tables with naturally ordered data |

### Index Design Rules

1. **Column order matters** - Most selective column first in composite indexes
2. **Cover queries** - Include columns for index-only scans
3. **Partial indexes** - Filter out unneeded rows (e.g., `WHERE deleted_at IS NULL`)
4. **Avoid over-indexing** - Each index slows writes
5. **Use CONCURRENTLY** - Avoid locking production tables

### Duplicate Index Detection

```sql
SELECT
  pg_size_pretty(sum(pg_relation_size(idx))::bigint) as size,
  (array_agg(idx))[1] as idx1, (array_agg(idx))[2] as idx2,
  (array_agg(idx))[3] as idx3, (array_agg(idx))[4] as idx4
FROM (
  SELECT indexrelid::regclass as idx, indrelid, indkey, indclass, indoption,
         (indpred IS NOT NULL) as has_pred, (indexprs IS NOT NULL) as has_exprs
  FROM pg_index
) sub
GROUP BY indrelid, indkey, indclass, indoption, has_pred, has_exprs
HAVING count(*) > 1
ORDER BY sum(pg_relation_size(idx)) DESC;
```

## Common Issues and Solutions

### Issue: Vacuum Not Running

**Symptoms**: High dead tuple count, table bloat
**Check**:
```sql
SELECT relname, last_autovacuum, last_autoanalyze,
       autovacuum_count, autoanalyze_count
FROM pg_stat_user_tables
WHERE last_autovacuum IS NULL OR last_autovacuum < now() - interval '7 days'
ORDER BY n_dead_tup DESC;
```
**Fix**: Check autovacuum settings, run manual VACUUM

### Issue: Lock Waits

**Symptoms**: Queries stuck, connection buildup
**Check**: Use the lock contention query from the dba command
**Fix**: Identify blocking query, consider killing it, review transaction boundaries

### Issue: Connection Exhaustion

**Symptoms**: "too many connections" errors
**Check**:
```sql
SELECT count(*), state FROM pg_stat_activity GROUP BY state;
```
**Fix**: Use connection pooler, reduce max_connections, check for connection leaks

### Issue: Slow Sequential Scans

**Symptoms**: Full table scans on large tables
**Check**: Look at seq_scan vs idx_scan ratio
**Fix**: Add appropriate index, update statistics, check random_page_cost

## Best Practices

1. **Always use EXPLAIN ANALYZE** before and after changes
2. **Test on staging** with production-like data volumes
3. **Monitor pg_stat_statements** for query performance trends
4. **Schedule maintenance** during low-traffic periods
5. **Keep PostgreSQL updated** for performance improvements
6. **Use connection pooling** (PgBouncer, PgDog) for high connection counts
7. **Partition large tables** that grow beyond 100GB
8. **Regular REINDEX** for heavily-updated tables
