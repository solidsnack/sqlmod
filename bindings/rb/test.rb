#!/usr/bin/env ruby
require "pp"

$LOAD_PATH.unshift "lib/"

require "sqlmod"


queries = <<-SQL
--@ now ro
SELECT now();

--@ today ro
SELECT 'today'::timestamptz;
SQL

qs = SQLMod::parse(queries)

pp qs
