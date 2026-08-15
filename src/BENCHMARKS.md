# pg_lab benchmarks

Measured results from pg_lab SPI exercises, kept alongside the code so
future changes can be checked against real numbers instead of assumptions.

---

## Manual N+1 join vs SQL JOIN (Phase D #18)

**Date:** 2026-08-15
**Functions compared:**
- `pg_lab_orders_with_person_manual` — fetches all orders, then issues one
  additional SPI query per row to look up the person's name (N+1 pattern)
- `pg_lab_orders_with_person_joined` — single query using `JOIN orders o
  ON o.person_id = p.id`

**Method:**
```sql
\timing on
SELECT count(*) FROM generate_series(1, 200) i, LATERAL pg_lab_orders_with_person_manual() x;
SELECT count(*) FROM generate_series(1, 200) i, LATERAL pg_lab_orders_with_person_joined() x;
```
Each function returns 18 rows per call (orders table size at time of test),
so 200 iterations = 3600 total rows either way. Correctness was verified
separately: both functions return identical row sets.

**Results:**

| Version | Total time (200 calls) | Avg per call | Relative |
|---|---|---|---|
| Manual join (N+1) | 777.153 ms | ~3.9 ms | 66x slower |
| SQL JOIN | 11.677 ms | ~0.06 ms | baseline |

**Why:** each SPI call carries fixed overhead (context switch, catalog/plan
lookup, tuple table setup) independent of how simple the query itself is.
The manual version pays this cost once per order row -- 18 extra
round-trips per function call, on top of the initial orders fetch. The
JOIN version pays SPI overhead once per call, and lets Postgres's planner
merge both tables in a single execution pass.

**Takeaway:** for anything beyond trivial row counts, looping over rows in
application/extension code and issuing one query per row does not scale
the way a single relational join does. This holds even inside pgrx, where
the "query" is a same-process SPI call rather than a network round-trip --
the per-call overhead is still real.