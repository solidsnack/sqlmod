#!/usr/bin/env ruby
decl = /^([^ ]+( [^ ]+)*) ([^ (]+)[(]([^,)]+(, +[^,)]+)*)[)];$/
decls = []

File::open("target/include/qselect.h") do |h|
  h.each_line do |line|
    decls << $~ if line =~ decl
  end
end

parsed = decls.map do |d|
 [d[3],
  d[4].split(/, */).map{|s| s.sub(/[^ *]+$/, "").strip},
  d[1]]
end

def map_types(ty)
  types = {"sqlmod_str_t" => "StrHandle.val",
           "sqlmod_queries_t const*" => "Queries",
           "sqlmod_queries_t*" => "Queries",
           "sqlmod_query_t const*" => "Query"}
  case
  when types[ty] then types[ty]
  when ty.include?("*") then ":pointer"
  else ":#{ty.to_sym}"
  end
end

in_ffi_form = parsed.map do |name, args, return_type|
  args = args.map{|s| map_types(s)}
  ret = map_types(return_type)
  new_name = name.sub(/^sqlmod_/, "").to_sym
  [":#{new_name}", ":#{name}", "[#{args.join(", ")}]", ret]
end

in_ffi_form.each do |new_name, name, args, ret|
  puts "  attach_function #{new_name},"
  puts "                  #{name},"
  puts "                  #{args}, #{ret}"
end
