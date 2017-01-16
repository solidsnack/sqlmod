# `sqlmod`

Modular SQL excitement: `sqlmod` allows you to treat a file of SQL queries as a
module in your favorite programming language.

For longer queries, embedding as strings is error prone and confusing; and ORMs
and SQL expression languages don't generally provide clear and communicative
access to analytic functionality. A collection of named queries in a separate
file:

* allows queries to be shared between projects,
* allows queries to be shared with analysts and copy-pasted into BI tools,
* allows access to advanced or database specific SQL in a way that is more
  modular and readable than embedded SQL.

## Example

```sql
--@ now ro
SELECT now()

--@ maybe_create_user
--- Uses anonymous PL/pgSQL to handle potentially redundant user creation.
DO $$
DECLARE
  sql_state text;
BEGIN
  BEGIN
    CREATE ROLE the_user;
  EXCEPTION
    WHEN duplicate_object THEN
      --- Do nothing.
    WHEN OTHERS THEN
      GET STACKED DIAGNOSTICS sql_state = RETURNED_SQLSTATE;
      RAISE INFO 'SQLState: %', sql_state;
  END;
END
$$;
```

## Historical Context

ORMs provide a way to work with SQL databases without writing SQL. This is
great for those cases where the database is more or less modelling a hash
table; but little provision is made for bulk operations, aggregations, windows
or administrative commands. SQL expression languages, like SQLAlchemy, allow us
to go further with SQL, but these tools stress portability more than exactness,
which can make granular control of the resulting query or its plan impossible.

Analysts have historically used SQL directly for both prototyping and
production analysis.

To make use of raw SQL, either to facilitate cooperation with analysts, control
performance or access advanced features, developers use some form of embedded
SQL. As a formal part of the SQL standard, "embedded SQL" includes a
pre-processor to rewrite the code to insert the actual database calls and type
translation. This approach is well documented for the Oracle database, which
offers a variety of bindings: Ada, Pascal, C, Fortran. Postgres, on the other
hand, only provides it for C.

Embedded SQL is disadvantageous because of the absence of syntax highlighting
and the awkward formatting of the queries (double-escaping and weird
indentation, &c). Embedded SQL, like ORMs, can make it hard to tell exactly
what queries an app issues or where it issues them from.

To address the awkwardness implicit in embedded SQL, there arose _SQL modules_.
There is a SQL module processor that is part of Oracle and the Vax had one, as
well. These systems generally operate on an extended SQL, which contains
declarations similar to prepared statements. The extended SQL is compiled to
the target language, usually a statically typed language like Pascal or C.

While the principle of separating queries into their own file inspired
`sqlmod`, which thus finds its historical ancestors in these SQL module
languages, the queries are not compiled but rather parsed into objects which
can be introspected as well as directly executed. This keeps the implementation
small and allows for both interactive development and flexible debugging, while
retaining the modularity and clarity advantages of SQL modules.
