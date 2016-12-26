require "ffi"

module QuerySelector

def self.parse(text)
  Queries::parse(text)
end


class Queries < FFI::AutoPointer
  def self.release(ptr)
    Bridge::free(ptr)
  end

  def self.parse(text)
    text = StrHandle.new(text) unless text.is_a? FFI::Struct
    obj = Bridge::parse(text)
    obj.null? ? nil : obj
  end

  def [](k)
    k = StrHandle.new(k) unless k.is_a? FFI::Struct
    obj = Bridge::get(self, k)
    if obj.null?
      nil
    else
      obj.queries = self                 # Keep self alive while query is alive
      obj
    end
  end

  def size
    Bridge::len(self)
  end
end


class Query < FFI::AutoPointer
  attr_accessor :queries

  def self.release(ptr)
    # Do nothing. Freed when @queries goes away.
  end

  def name
    Bridge::name(self).to_s
  end

  def text
    Bridge::text(self).to_s
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


module Bridge
  extend FFI::Library
  ffi_lib ["libqselect",
           "target/release/libqselect.dylib",
           "target/debug/libqselect.dylib"]

  attach_function :parse, :qselect_parse, [StrHandle.val], Queries
  attach_function :len, :qselect_len, [Queries], :size_t
  attach_function :get, :qselect_get, [Queries, StrHandle.val], Query
  attach_function :free, :qselect_free, [Queries], :void

  attach_function :name, :qselect_name, [Query], StrHandle.val
  attach_function :text, :qselect_text, [Query], StrHandle.val
end

end