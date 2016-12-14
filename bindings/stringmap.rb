require "ffi"

class StringMap < FFI::AutoPointer
  def self.release(ptr)
    Bridge.free(ptr)
  end

  def self.new(strings)
    return super if strings.is_a? FFI::Pointer
    keys, vals = strings.to_a.transpose.map do |arr|
      arr.map do |item|
        FFI::MemoryPointer.from_string(item.to_s)
      end
    end
    count = keys.size
    k = FFI::MemoryPointer.new(:pointer, count)
    k.write_array_of_pointer(keys)
    v = FFI::MemoryPointer.new(:pointer, count)
    v.write_array_of_pointer(vals)
    Bridge::new(count, k, v)
  end

  def size
    Bridge::len(self)
  end

  def [](k)
    Bridge::get(self, k)
  end

  def keys
    ptr = Bridge::names(self)
    data = []
    while ptr.read_pointer != FFI::Pointer::NULL do
      data << ptr.read_pointer.read_string
      ptr += FFI::Pointer::SIZE
    end
    data
  end

  module Bridge
    extend FFI::Library
    ffi_lib ["libqselect",
             "target/release/libqselect.dylib",
             "target/debug/libqselect.dylib"]

    attach_function :new, :stringmap_new,
                    [:size_t, :pointer, :pointer], StringMap
    attach_function :free, :stringmap_free, [StringMap], :void
    attach_function :len, :stringmap_len, [StringMap], :size_t
    attach_function :names, :stringmap_names, [StringMap], :pointer
    attach_function :get, :stringmap_get, [StringMap, :string], :string
  end
end


__END__

# Usage example:

load "bindings/stringmap.rb"
=> true

m = StringMap::new({"a" => "b"})
=> #<StringMap address=0x007f91c56158e0>

m.keys
=> ["a"]

m["a"]
=> "b"
