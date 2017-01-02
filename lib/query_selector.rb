require "ffi"

module QuerySelector

def self.parse(text)
  # Ruby hashes since 1.9 preserve insertion order so we can don't need a
  # special data structure.
  result = {}
  bridged = Bridge::Queries::parse(text)
  if bridged
    (0...bridged.size).each do |i|
      q = bridged[i]
      result[q.name.to_sym] = Query.new(q)
    end
    result
  end
end


class Query
  attr_reader :underlying

  def initialize(underlying)
    @underlying = underlying
  end

  def name
    @name ||= underlying.name
  end

  def text
    @text ||= underlying.text
  end

  def attributes
    @attributes ||= underlying.attributes
  end

  def inspect
    @formatted ||=
      "Query(name: #{name.inspect}, text: #{text.inspect}, " +
      "attributes: #{attributes})"
  end

  def to_s
    inspect
  end
end


module Bridge
  extend FFI::Library
  ffi_lib ["libqselect",
           "target/release/libqselect.dylib",
           "target/debug/libqselect.dylib"]
 
  class Queries < FFI::AutoPointer
    def self.release(ptr)
      Bridge::queries_free(ptr)
    end

    def self.parse(text)
      text = StrHandle.new(text) unless text.is_a? FFI::Struct
      obj = Bridge::queries_parse(text)
      obj.null? ? nil : obj
    end

    def [](i)
      q = Bridge::queries_get_query_by_index(self, i)
      q.queries = self                    # Keep obj alive while query is alive
      q
    end

    def size
      Bridge::queries_num_queries(self)
    end
  end

  class Query < FFI::AutoPointer
    attr_accessor :queries

    def self.release(ptr)
      # Do nothing. Freed when @queries goes away.
    end

    def name
      Bridge::query_get_name(self).to_s
    end

    def text
      Bridge::query_get_text(self).to_s
    end
 
    def attributes 
      num = Bridge::query_num_attributes(self)
      (0...num).map{|i| Bridge::query_get_attribute_by_index(self, i).to_s}
    end
  end

  class StrHandle < FFI::Struct
    layout :ptr, :pointer,
           :len, :size_t

    def self.new(*args)
      case args[0]
      when Symbol, String
        s = args[0].to_s
        ptr = FFI::MemoryPointer.new(:uint8, s.bytesize)
        ptr.write_bytes(s, 0, ptr.size)
        struct = self.new
        struct[:ptr] = ptr
        struct[:len] = ptr.size
        struct
      else super
      end
    end

    def to_s
        @str ||= case
                 when self[:ptr].null? then nil
                 when self[:len] == 0 then ""
                 else self[:ptr].read_string(self[:len])
                 end
    end
  end

  attach_function :queries_parse,
                  :qselect_queries_parse,
                  [StrHandle.val], Queries
  attach_function :queries_get_query_by_name,
                  :qselect_queries_get_query_by_name,
                  [Queries, StrHandle.val], Query
  attach_function :queries_get_query_by_index,
                  :qselect_queries_get_query_by_index,
                  [Queries, :size_t], Query
  attach_function :queries_num_queries,
                  :qselect_queries_num_queries,
                  [Queries], :size_t
  attach_function :queries_free,
                  :qselect_queries_free,
                  [Queries], :void

  attach_function :query_get_name,
                  :qselect_query_get_name,
                  [Query], StrHandle.val
  attach_function :query_get_text,
                  :qselect_query_get_text,
                  [Query], StrHandle.val
  attach_function :query_num_attributes,
                  :qselect_query_num_attributes,
                  [Query], :size_t
  attach_function :query_get_attribute_by_index,
                  :qselect_query_get_attribute_by_index,
                  [Query, :size_t], StrHandle.val

  attach_function :str,
                  :qselect_str,
                  [:pointer, :size_t], StrHandle.val

end

end
