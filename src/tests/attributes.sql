--- An example SQL file with annotations.


--@ now
SELECT now()


--@ now_ro ro
SELECT now()


--@ now_etc ro &c. -> (1)
SELECT now()


--@ utc_parens(t: datetime) ro
SELECT :t::timestamptz AT TIME ZONE 'UTC'


--@ utc_parens2(t timestamp with time zone)
SELECT :t::timestamptz AT TIME ZONE 'UTC'


--@ utc_parens3(t timestamp with time zone) (1)
SELECT :t::timestamptz AT TIME ZONE 'UTC'
