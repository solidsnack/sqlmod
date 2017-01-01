#!/usr/bin/env ruby
require "pp"

load "bindings/queryselector.rb"

queries = <<-SQL
--@ now ro
SELECT now();

--@ today ro
SELECT 'today'::timestamptz;
SQL

qs = QuerySelector::parse(queries)

pp qs
