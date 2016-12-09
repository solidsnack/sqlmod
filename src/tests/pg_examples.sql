--- An example SQL file using various odds-and-ends of Postgres.
--- It includes comments, decorative whitespace and a variety of queries
--- and statements.

--@ now ro
SELECT now()

--@ maybe_create_user
--- Uses anonymous PL/pgSQL to redundant user creation.
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


--@ as_timestamp ro
--- Requests database interprets timestamp.
SELECT :t::timestamptz AS t;
--@ as_interval ro
--- Requests database interprets time interval.
SELECT :i::interval AS i;