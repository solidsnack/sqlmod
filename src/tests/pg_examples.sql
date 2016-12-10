--- An example SQL file using various odds-and-ends of Postgres.
--- It includes comments, decorative whitespace and a variety of queries
--- and statements.

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


--@ as_timestamp ro
--- Requests database interprets timestamp.
SELECT :t::timestamptz AS t;
--@ as_interval ro
--- Requests database interprets time interval.
SELECT :i::interval AS i;


--- Temporary tables.

--@ load_archive

CREATE TABLE tmp (LIKE original);

COPY tmp FROM '/tmp/original.tsv';

INSERT INTO original SELECT * FROM tmp
ON CONFLICT DO NOTHING;
