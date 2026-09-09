#!/usr/bin/env python3
"""Read-only PostgreSQL evidence exporter for table/column retirement targets."""
import argparse
import json
from datetime import datetime, timezone

import psycopg
from psycopg import sql


def emit(signal: str, *, count: int, active: bool, **extra):
    row = {
        "signal": signal,
        "source": "postgres-readonly",
        "observed_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "count": count,
        "active": active,
        **extra,
    }
    print(json.dumps(row, separators=(",", ":")))


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--dsn", required=True, help="PostgreSQL DSN; use a least-privilege read-only role")
    p.add_argument("--table", required=True)
    p.add_argument("--column")
    p.add_argument("--statement-timeout-ms", type=int, default=5000)
    args = p.parse_args()

    with psycopg.connect(args.dsn, autocommit=False) as conn:
        with conn.cursor() as cur:
            cur.execute("BEGIN READ ONLY")
            cur.execute("SET LOCAL statement_timeout = %s", (args.statement_timeout_ms,))
            cur.execute("SELECT to_regclass(%s)", (args.table,))
            exists = cur.fetchone()[0] is not None
            emit("database.table.exists", count=int(exists), active=exists, table=args.table)
            if not exists:
                conn.rollback()
                return
            query = sql.SQL("SELECT count(*) FROM {} ").format(sql.Identifier(*args.table.split(".", 1)) if "." in args.table else sql.Identifier(args.table))
            cur.execute(query)
            rows = int(cur.fetchone()[0])
            emit("database.rows", count=rows, active=rows > 0, table=args.table)
            cur.execute(
                """SELECT count(*) FROM information_schema.view_table_usage
                   WHERE table_name = %s""",
                (args.table.split(".")[-1],),
            )
            views = int(cur.fetchone()[0])
            emit("database.views", count=views, active=views > 0, table=args.table)
            cur.execute(
                """SELECT count(*) FROM information_schema.table_constraints
                   WHERE table_name = %s AND constraint_type = 'FOREIGN KEY'""",
                (args.table.split(".")[-1],),
            )
            fks = int(cur.fetchone()[0])
            emit("database.foreign_keys", count=fks, active=fks > 0, table=args.table)
            if args.column:
                cur.execute(
                    """SELECT count(*) FROM information_schema.columns
                       WHERE table_name = %s AND column_name = %s""",
                    (args.table.split(".")[-1], args.column),
                )
                present = int(cur.fetchone()[0])
                emit("database.column.exists", count=present, active=present > 0, table=args.table, column=args.column)
            conn.rollback()


if __name__ == "__main__":
    main()
